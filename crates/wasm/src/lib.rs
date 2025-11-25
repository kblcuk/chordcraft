//! WASM bindings for ChordCraft
//!
//! This crate provides JavaScript-friendly bindings for the ChordCraft core library,
//! allowing chord-fingering conversion to run in web browsers.

use chordcraft_core::{
	Chord, Fingering, Guitar, PlayingContext,
	analyzer::{ChordMatch, analyze_fingering},
	chord::VoicingType,
	generator::{GeneratorOptions, ScoredFingering, generate_fingerings},
	progression::{ProgressionOptions, ProgressionSequence, generate_progression},
};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

/// Initialize panic hook for better error messages in browser console
#[wasm_bindgen(start)]
pub fn init() {
	console_error_panic_hook::set_once();
}

// ============================================================================
// JS-Friendly Types
// ============================================================================

/// Instrument type for WASM API
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum InstrumentType {
	Guitar,
	// Future: Ukulele, Bass, Mandolin
}

/// Options for fingering generation (JS-friendly)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsGeneratorOptions {
	/// Maximum number of fingerings to return
	#[serde(default = "default_limit")]
	pub limit: usize,
	/// Preferred fret position (fingerings near this position are ranked higher)
	pub preferred_position: Option<u8>,
	/// Voicing type filter ("core", "full", "jazzy", or null for all)
	pub voicing_type: Option<String>,
	/// Whether to include fingerings with the root in the bass
	#[serde(default = "default_true")]
	pub root_in_bass: bool,
	/// Maximum fret to consider
	#[serde(default = "default_max_fret")]
	pub max_fret: u8,
	/// Playing context ("solo" or "band")
	#[serde(default)]
	pub playing_context: String,
	/// Capo position (0 = no capo)
	#[serde(default)]
	pub capo: u8,
}

fn default_limit() -> usize {
	10
}
fn default_true() -> bool {
	true
}
fn default_max_fret() -> u8 {
	12
}

impl Default for JsGeneratorOptions {
	fn default() -> Self {
		Self {
			limit: 10,
			preferred_position: None,
			voicing_type: None,
			root_in_bass: true,
			max_fret: 12,
			playing_context: "solo".to_string(),
			capo: 0,
		}
	}
}

/// Options for progression generation (JS-friendly)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsProgressionOptions {
	/// Number of alternative progressions to show
	#[serde(default = "default_progression_limit")]
	pub limit: usize,
	/// Maximum fret distance between consecutive fingerings
	#[serde(default = "default_max_distance")]
	pub max_fret_distance: u8,
	/// Number of fingering candidates to consider per chord
	#[serde(default = "default_candidates")]
	pub candidates_per_chord: usize,
	/// Generator options for each chord
	#[serde(default)]
	pub generator_options: JsGeneratorOptions,
}

fn default_progression_limit() -> usize {
	3
}
fn default_max_distance() -> u8 {
	3
}
fn default_candidates() -> usize {
	20
}

impl Default for JsProgressionOptions {
	fn default() -> Self {
		Self {
			limit: 3,
			max_fret_distance: 3,
			candidates_per_chord: 20,
			generator_options: JsGeneratorOptions::default(),
		}
	}
}

/// Scored fingering result (JS-friendly)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsScoredFingering {
	/// Tab notation (e.g., "x32010")
	pub tab: String,
	/// Playability score (0-100)
	pub score: u8,
	/// Voicing type ("core", "full", or "jazzy")
	pub voicing_type: String,
	/// Whether root is in bass
	pub has_root_in_bass: bool,
	/// Average fret position
	pub position: u8,
	/// Notes in the fingering (e.g., ["C", "E", "G"])
	pub notes: Vec<String>,
}

/// Chord match result (JS-friendly)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsChordMatch {
	/// Chord name (e.g., "Cmaj7")
	pub name: String,
	/// Confidence percentage (0-100)
	pub confidence: u8,
	/// Explanation of why this chord matches
	pub explanation: String,
}

/// Transition between chords (JS-friendly)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsChordTransition {
	pub from_chord: String,
	pub to_chord: String,
	pub from_fingering: JsScoredFingering,
	pub to_fingering: JsScoredFingering,
	pub score: i32,
	pub finger_movements: usize,
	pub common_anchors: usize,
	pub position_distance: u8,
}

/// Complete progression sequence (JS-friendly)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsProgressionSequence {
	pub chords: Vec<String>,
	pub fingerings: Vec<JsScoredFingering>,
	pub transitions: Vec<JsChordTransition>,
	pub total_score: i32,
	pub avg_transition_score: f32,
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Convert voicing type string to enum
fn parse_voicing_type(s: &str) -> Option<VoicingType> {
	match s.to_lowercase().as_str() {
		"core" => Some(VoicingType::Core),
		"full" => Some(VoicingType::Full),
		"jazzy" => Some(VoicingType::Jazzy),
		_ => None,
	}
}

/// Convert voicing type enum to string
fn voicing_type_to_string(vt: &VoicingType) -> String {
	match vt {
		VoicingType::Core => "core".to_string(),
		VoicingType::Full => "full".to_string(),
		VoicingType::Jazzy => "jazzy".to_string(),
	}
}

/// Convert playing context string to enum
fn parse_playing_context(s: &str) -> PlayingContext {
	match s.to_lowercase().as_str() {
		"band" => PlayingContext::Band,
		_ => PlayingContext::Solo,
	}
}

/// Convert JsGeneratorOptions to GeneratorOptions
fn js_to_generator_options(js_opts: &JsGeneratorOptions) -> GeneratorOptions {
	GeneratorOptions {
		limit: js_opts.limit,
		preferred_position: js_opts.preferred_position,
		voicing_type: js_opts
			.voicing_type
			.as_ref()
			.and_then(|s| parse_voicing_type(s)),
		root_in_bass: js_opts.root_in_bass,
		max_fret: js_opts.max_fret,
		playing_context: parse_playing_context(&js_opts.playing_context),
	}
}

/// Convert ScoredFingering to JsScoredFingering
fn scored_fingering_to_js(sf: &ScoredFingering, instrument: &Guitar) -> JsScoredFingering {
	let notes = sf
		.fingering
		.unique_pitch_classes(instrument)
		.into_iter()
		.map(|pc| format!("{:?}", pc))
		.collect();

	JsScoredFingering {
		tab: sf.fingering.to_string(),
		score: sf.score,
		voicing_type: voicing_type_to_string(&sf.voicing_type),
		has_root_in_bass: sf.has_root_in_bass,
		position: sf.position,
		notes,
	}
}

/// Convert ChordMatch to JsChordMatch
fn chord_match_to_js(cm: &ChordMatch) -> JsChordMatch {
	let confidence = (cm.completeness * 100.0) as u8;
	let explanation = if cm.root_in_bass {
		format!("{}% complete with root in bass", confidence)
	} else {
		format!("{}% complete", confidence)
	};

	JsChordMatch {
		name: cm.chord.to_string(),
		confidence,
		explanation,
	}
}

/// Convert ProgressionSequence to JsProgressionSequence
fn progression_to_js(seq: &ProgressionSequence, instrument: &Guitar) -> JsProgressionSequence {
	let js_fingerings: Vec<JsScoredFingering> = seq
		.fingerings
		.iter()
		.map(|sf| scored_fingering_to_js(sf, instrument))
		.collect();

	let js_transitions: Vec<JsChordTransition> = seq
		.transitions
		.iter()
		.map(|t| JsChordTransition {
			from_chord: t.from_chord.clone(),
			to_chord: t.to_chord.clone(),
			from_fingering: scored_fingering_to_js(&t.from_fingering, instrument),
			to_fingering: scored_fingering_to_js(&t.to_fingering, instrument),
			score: t.score,
			finger_movements: t.finger_movements,
			common_anchors: t.common_anchors,
			position_distance: t.position_distance,
		})
		.collect();

	JsProgressionSequence {
		chords: seq.chords.clone(),
		fingerings: js_fingerings,
		transitions: js_transitions,
		total_score: seq.total_score,
		avg_transition_score: seq.avg_transition_score,
	}
}

// ============================================================================
// WASM Exports
// ============================================================================

/// Find fingerings for a chord
///
/// # Arguments
/// * `chord_name` - Chord name (e.g., "Cmaj7", "Abm7")
/// * `instrument_type` - Instrument type (currently only "guitar")
/// * `options` - Generation options (or null for defaults)
///
/// # Returns
/// JSON array of scored fingerings
///
/// # Example (JavaScript)
/// ```javascript
/// import init, { findFingerings } from './chordcraft_wasm.js';
///
/// await init();
/// const results = findFingerings("Cmaj7", "guitar", {
///   limit: 5,
///   voicingType: "core",
///   playingContext: "band"
/// });
/// console.log(results);
/// ```
#[wasm_bindgen(js_name = findFingerings)]
pub fn find_fingerings(
	chord_name: &str,
	instrument_type: JsValue,
	options: JsValue,
) -> Result<JsValue, JsValue> {
	// Parse instrument type
	let _inst_type: InstrumentType = serde_wasm_bindgen::from_value(instrument_type)
		.map_err(|e| JsValue::from_str(&format!("Invalid instrument type: {}", e)))?;

	// Parse options (use defaults if null/undefined)
	let js_opts: JsGeneratorOptions = if options.is_null() || options.is_undefined() {
		JsGeneratorOptions::default()
	} else {
		serde_wasm_bindgen::from_value(options)
			.map_err(|e| JsValue::from_str(&format!("Invalid options: {}", e)))?
	};

	// Parse chord
	let chord = Chord::parse(chord_name)
		.map_err(|e| JsValue::from_str(&format!("Invalid chord name: {}", e)))?;

	// Create instrument
	let instrument = Guitar::default();

	// Generate fingerings (with capo applied if needed)
	let gen_opts = js_to_generator_options(&js_opts);
	let fingerings = if js_opts.capo > 0 {
		let capo_instrument = instrument
			.with_capo(js_opts.capo)
			.map_err(|e| JsValue::from_str(&format!("Invalid capo position: {}", e)))?;
		generate_fingerings(&chord, &capo_instrument, &gen_opts)
	} else {
		generate_fingerings(&chord, &instrument, &gen_opts)
	};

	// Convert to JS-friendly format
	let js_fingerings: Vec<JsScoredFingering> = fingerings
		.iter()
		.map(|sf| scored_fingering_to_js(sf, &instrument))
		.collect();

	// Serialize to JS
	serde_wasm_bindgen::to_value(&js_fingerings)
		.map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
}

/// Identify chord from fingering (tab notation)
///
/// # Arguments
/// * `tab_notation` - Tab notation (e.g., "x32010")
/// * `instrument_type` - Instrument type (currently only "guitar")
///
/// # Returns
/// JSON array of chord matches with confidence scores
///
/// # Example (JavaScript)
/// ```javascript
/// const matches = analyzeChord("x32010", "guitar");
/// console.log(matches[0].name); // "C"
/// console.log(matches[0].confidence); // 100
/// ```
#[wasm_bindgen(js_name = analyzeChord)]
pub fn analyze_chord(tab_notation: &str, instrument_type: JsValue) -> Result<JsValue, JsValue> {
	// Parse instrument type
	let _inst_type: InstrumentType = serde_wasm_bindgen::from_value(instrument_type)
		.map_err(|e| JsValue::from_str(&format!("Invalid instrument type: {}", e)))?;

	// Parse fingering
	let fingering = Fingering::parse(tab_notation)
		.map_err(|e| JsValue::from_str(&format!("Invalid tab notation: {}", e)))?;

	// Create instrument
	let instrument = Guitar::default();

	// Analyze fingering
	let matches = analyze_fingering(&fingering, &instrument);

	// Convert to JS-friendly format
	let js_matches: Vec<JsChordMatch> = matches.iter().map(chord_match_to_js).collect();

	// Serialize to JS
	serde_wasm_bindgen::to_value(&js_matches)
		.map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
}

/// Generate optimal fingering progressions for a chord sequence
///
/// # Arguments
/// * `chord_names` - Array of chord names (e.g., ["C", "Am", "F", "G"])
/// * `instrument_type` - Instrument type (currently only "guitar")
/// * `options` - Progression options (or null for defaults)
///
/// # Returns
/// JSON array of progression sequences, sorted by quality
///
/// # Example (JavaScript)
/// ```javascript
/// const progressions = generateProgression(
///   ["Cmaj7", "Am7", "Dm7", "G7"],
///   "guitar",
///   { limit: 3, maxFretDistance: 3 }
/// );
/// console.log(progressions[0].avgTransitionScore);
/// ```
#[wasm_bindgen(js_name = generateProgression)]
pub fn js_generate_progression(
	chord_names: JsValue,
	instrument_type: JsValue,
	options: JsValue,
) -> Result<JsValue, JsValue> {
	// Parse instrument type
	let _inst_type: InstrumentType = serde_wasm_bindgen::from_value(instrument_type)
		.map_err(|e| JsValue::from_str(&format!("Invalid instrument type: {}", e)))?;

	// Parse chord names
	let chord_names_vec: Vec<String> = serde_wasm_bindgen::from_value(chord_names)
		.map_err(|e| JsValue::from_str(&format!("Invalid chord names: {}", e)))?;

	// Parse options
	let js_opts: JsProgressionOptions = if options.is_null() || options.is_undefined() {
		JsProgressionOptions::default()
	} else {
		serde_wasm_bindgen::from_value(options)
			.map_err(|e| JsValue::from_str(&format!("Invalid options: {}", e)))?
	};

	// Create instrument
	let instrument = Guitar::default();

	// Build progression options
	let prog_opts = ProgressionOptions {
		limit: js_opts.limit,
		max_fret_distance: js_opts.max_fret_distance,
		candidates_per_chord: js_opts.candidates_per_chord,
		generator_options: js_to_generator_options(&js_opts.generator_options),
	};

	// Convert Vec<String> to Vec<&str> for API compatibility
	let chord_name_refs: Vec<&str> = chord_names_vec.iter().map(|s| s.as_str()).collect();

	// Generate progressions (with capo applied if needed)
	let progressions = if js_opts.generator_options.capo > 0 {
		let capo_instrument = instrument
			.with_capo(js_opts.generator_options.capo)
			.map_err(|e| JsValue::from_str(&format!("Invalid capo position: {}", e)))?;
		generate_progression(&chord_name_refs, &capo_instrument, &prog_opts)
	} else {
		generate_progression(&chord_name_refs, &instrument, &prog_opts)
	};

	// Convert to JS-friendly format
	let js_progressions: Vec<JsProgressionSequence> = progressions
		.iter()
		.map(|seq| progression_to_js(seq, &instrument))
		.collect();

	// Serialize to JS
	serde_wasm_bindgen::to_value(&js_progressions)
		.map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
	use super::*;
	use wasm_bindgen_test::*;

	#[wasm_bindgen_test]
	fn test_find_fingerings_basic() {
		let inst = serde_wasm_bindgen::to_value(&InstrumentType::Guitar).unwrap();
		let opts = JsValue::NULL;

		let result = find_fingerings("C", inst, opts);
		assert!(result.is_ok());
	}

	#[wasm_bindgen_test]
	fn test_analyze_chord_basic() {
		let inst = serde_wasm_bindgen::to_value(&InstrumentType::Guitar).unwrap();

		let result = analyze_chord("x32010", inst);
		assert!(result.is_ok());
	}
}
