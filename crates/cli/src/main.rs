use anyhow::{Context, Result};
use clap::{Parser, Subcommand, ValueEnum};
use colored::Colorize;

use chordcraft_core::chord::{Chord, VoicingType};
use chordcraft_core::generator::{
	GeneratorOptions, PlayingContext, ScoredFingering, format_fingering_diagram,
	generate_fingerings,
};
use chordcraft_core::instrument::{Guitar, Ukulele};

#[derive(Debug, Clone, Copy, Default, ValueEnum)]
enum InstrumentChoice {
	/// Standard 6-string guitar (EADGBE tuning)
	#[default]
	Guitar,
	/// Standard ukulele (GCEA tuning)
	Ukulele,
}

fn parse_voicing_type(voicing: Option<&String>) -> Option<VoicingType> {
	voicing.and_then(|v| match v.to_lowercase().as_str() {
		"core" => Some(VoicingType::Core),
		"full" => Some(VoicingType::Full),
		"jazzy" | "jazz" => Some(VoicingType::Jazzy),
		_ => None,
	})
}

fn parse_playing_context(context: Option<&String>) -> PlayingContext {
	context
		.map(|c| match c.to_lowercase().as_str() {
			"band" => PlayingContext::Band,
			_ => PlayingContext::Solo,
		})
		.unwrap_or(PlayingContext::Solo)
}

#[derive(Parser)]
#[command(name = "chordcraft")]
#[command(about = "A tool for chord-fingering conversion", long_about = None)]
#[command(version)]
struct Cli {
	#[command(subcommand)]
	command: Commands,
}

#[derive(Subcommand)]
enum Commands {
	/// Find fingerings for a chord
	Find {
		/// Chord name (e.g., "Cmaj7", "Abm", "G7")
		chord: String,

		/// Number of fingerings to show
		#[arg(short, long, default_value = "5")]
		limit: usize,

		/// Prefer fingerings near this fret position
		#[arg(short, long)]
		position: Option<u8>,

		/// Voicing type: core, full, or jazzy
		#[arg(short, long)]
		voicing: Option<String>,

		/// Playing context: solo or band (default: solo)
		#[arg(short = 'x', long)]
		context: Option<String>,

		/// Capo position (fret number)
		#[arg(short, long)]
		capo: Option<u8>,

		/// Instrument: guitar or ukulele (default: guitar)
		#[arg(short, long, default_value = "guitar")]
		instrument: InstrumentChoice,
	},

	/// Identify chord from fingering notation
	Name {
		/// Tab notation (e.g., "x32010", "022100")
		fingering: String,

		/// Capo position (fret number)
		#[arg(short, long)]
		capo: Option<u8>,

		/// Instrument: guitar or ukulele (default: guitar)
		#[arg(short, long, default_value = "guitar")]
		instrument: InstrumentChoice,
	},

	/// Find optimal fingerings for a chord progression
	Progression {
		/// Chord names separated by spaces (e.g., "C Am F G")
		chords: String,

		/// Number of alternative progressions to show
		#[arg(short, long, default_value = "3")]
		limit: usize,

		/// Maximum fret distance between consecutive chords
		#[arg(short = 'd', long, default_value = "3")]
		max_distance: u8,

		/// Prefer fingerings near this fret position
		#[arg(short, long)]
		position: Option<u8>,

		/// Voicing type: core, full, or jazzy
		#[arg(short, long)]
		voicing: Option<String>,

		/// Playing context: solo or band (default: solo)
		#[arg(short = 'x', long)]
		context: Option<String>,

		/// Capo position (fret number)
		#[arg(short, long)]
		capo: Option<u8>,

		/// Instrument: guitar or ukulele (default: guitar)
		#[arg(short, long, default_value = "guitar")]
		instrument: InstrumentChoice,
	},
}

fn main() -> Result<()> {
	let cli = Cli::parse();

	match cli.command {
		Commands::Find {
			chord,
			limit,
			position,
			voicing,
			context,
			capo,
			instrument,
		} => {
			find_fingerings(&chord, limit, position, voicing, context, capo, instrument)?;
		}
		Commands::Name {
			fingering,
			capo,
			instrument,
		} => {
			name_chord(&fingering, capo, instrument)?;
		}
		Commands::Progression {
			chords,
			limit,
			max_distance,
			position,
			voicing,
			context,
			capo,
			instrument,
		} => {
			find_progression(
				&chords,
				FindProgressionInstrumentOptions {
					voicing,
					context,
					capo,
					instrument,
				},
				FindProgressionOptions {
					limit,
					max_distance,
					position,
				},
			)?;
		}
	}

	Ok(())
}

fn find_fingerings(
	chord_str: &str,
	limit: usize,
	position: Option<u8>,
	voicing: Option<String>,
	context: Option<String>,
	capo: Option<u8>,
	instrument: InstrumentChoice,
) -> Result<()> {
	let original_chord =
		Chord::parse(chord_str).with_context(|| format!("Invalid chord name: '{chord_str}'"))?;

	let (search_chord, shape_chord) = if let Some(capo_fret) = capo {
		let shape = original_chord.transpose(-(capo_fret as i32));
		(shape.clone(), Some(shape))
	} else {
		(original_chord.clone(), None)
	};

	let voicing_type = parse_voicing_type(voicing.as_ref());
	let playing_context = parse_playing_context(context.as_ref());

	let options = GeneratorOptions {
		limit,
		preferred_position: position,
		voicing_type,
		playing_context,
		..Default::default()
	};

	let instrument_name = match instrument {
		InstrumentChoice::Guitar => "Guitar",
		InstrumentChoice::Ukulele => "Ukulele",
	};

	let fingerings: Vec<ScoredFingering> = match instrument {
		InstrumentChoice::Guitar => {
			let guitar = Guitar::default();
			generate_fingerings(&search_chord, &guitar, &options)
		}
		InstrumentChoice::Ukulele => {
			let ukulele = Ukulele::default();
			generate_fingerings(&search_chord, &ukulele, &options)
		}
	};

	if fingerings.is_empty() {
		println!(
			"{}",
			format!("No fingerings found for chord: {original_chord}").yellow()
		);
		return Ok(());
	}

	if let Some(shape) = shape_chord {
		println!(
			"\n{} {} {} [{instrument_name}] (showing {} of {} found)",
			"Fingerings for".bold(),
			chord_str.green().bold(),
			format!("(Capo {})", capo.unwrap()).yellow(),
			fingerings.len().min(limit),
			fingerings.len()
		);
		println!("{} {}\n", "Shape:".dimmed(), shape.to_string().cyan());
	} else {
		println!(
			"\n{} {} [{instrument_name}] (showing {} of {} found)\n",
			"Fingerings for".bold(),
			original_chord.to_string().green().bold(),
			fingerings.len().min(limit),
			fingerings.len()
		);
	}

	for (i, scored) in fingerings.iter().take(limit).enumerate() {
		println!(
			"{}. {}",
			(i + 1).to_string().cyan().bold(),
			scored.fingering
		);
		let diagram = match instrument {
			InstrumentChoice::Guitar => format_fingering_diagram(scored, &Guitar::default()),
			InstrumentChoice::Ukulele => format_fingering_diagram(scored, &Ukulele::default()),
		};
		println!("{diagram}");
		println!();
	}

	Ok(())
}

struct FindProgressionInstrumentOptions {
	instrument: InstrumentChoice,
	voicing: Option<String>,
	context: Option<String>,
	capo: Option<u8>,
}
struct FindProgressionOptions {
	limit: usize,
	max_distance: u8,
	position: Option<u8>,
}
fn find_progression(
	chords_str: &str,
	instrument_opts: FindProgressionInstrumentOptions,
	progression_opts: FindProgressionOptions,
) -> Result<()> {
	use chordcraft_core::progression::{ProgressionOptions, generate_progression};
	let FindProgressionInstrumentOptions {
		instrument,
		voicing,
		context,
		capo,
	} = instrument_opts;

	let FindProgressionOptions {
		limit,
		max_distance,
		position,
	} = progression_opts;

	let chord_names: Vec<&str> = chords_str.split_whitespace().collect();

	if chord_names.is_empty() {
		println!("{}", "No chords provided".yellow());
		return Ok(());
	}

	let transposed_chords: Vec<String> = if let Some(capo_fret) = capo {
		chord_names
			.iter()
			.filter_map(|name| {
				Chord::parse(name)
					.ok()
					.map(|c| c.transpose(-(capo_fret as i32)).to_string())
			})
			.collect()
	} else {
		vec![]
	};

	let search_chords: Vec<&str> = if capo.is_some() {
		transposed_chords.iter().map(|s| s.as_str()).collect()
	} else {
		chord_names.clone()
	};

	let voicing_type = parse_voicing_type(voicing.as_ref());
	let playing_context = parse_playing_context(context.as_ref());

	let gen_options = GeneratorOptions {
		preferred_position: position,
		voicing_type,
		playing_context,
		..Default::default()
	};

	let options = ProgressionOptions {
		limit,
		max_fret_distance: max_distance,
		generator_options: gen_options,
		..Default::default()
	};

	let instrument_name = match instrument {
		InstrumentChoice::Guitar => "Guitar",
		InstrumentChoice::Ukulele => "Ukulele",
	};

	let progressions = match instrument {
		InstrumentChoice::Guitar => {
			let guitar = Guitar::default();
			generate_progression(&search_chords, &guitar, &options)
		}
		InstrumentChoice::Ukulele => {
			let ukulele = Ukulele::default();
			generate_progression(&search_chords, &ukulele, &options)
		}
	};

	if progressions.is_empty() {
		println!("{}", "No valid progressions found".yellow());
		return Ok(());
	}

	display_progressions(
		&progressions,
		&chord_names,
		capo,
		instrument_name,
		instrument,
	);

	Ok(())
}

fn display_progressions(
	progressions: &[chordcraft_core::progression::ProgressionSequence],
	chord_names: &[&str],
	capo: Option<u8>,
	instrument_name: &str,
	instrument: InstrumentChoice,
) {
	let chord_display = chord_names.join(" → ");
	if let Some(capo_fret) = capo {
		println!(
			"\n{} {} {} [{instrument_name}]\n",
			"Progression:".bold(),
			chord_display.green().bold(),
			format!("(Capo {capo_fret})").yellow()
		);
	} else {
		println!(
			"\n{} {} [{instrument_name}]\n",
			"Progression:".bold(),
			chord_display.green().bold()
		);
	}

	for (alt_idx, progression) in progressions.iter().enumerate() {
		println!("{}", "━".repeat(60).dimmed());
		println!(
			"{} #{}",
			"Alternative".bold(),
			(alt_idx + 1).to_string().cyan().bold()
		);
		println!(
			"{}: {} | {}: {:.1}",
			"Total Score".bold(),
			progression.total_score,
			"Avg Transition".bold(),
			progression.avg_transition_score
		);
		println!("{}", "━".repeat(60).dimmed());
		println!();

		for (i, fingering) in progression.fingerings.iter().enumerate() {
			let chord_name = if capo.is_some() {
				chord_names[i]
			} else {
				&progression.chords[i]
			};

			println!(
				"[{}] {} - Fret {}",
				(i + 1).to_string().cyan().bold(),
				chord_name.green().bold(),
				fingering.position
			);

			let diagram = match instrument {
				InstrumentChoice::Guitar => format_fingering_diagram(fingering, &Guitar::default()),
				InstrumentChoice::Ukulele => {
					format_fingering_diagram(fingering, &Ukulele::default())
				}
			};
			for line in diagram.lines() {
				println!("  {line}");
			}

			if i < progression.transitions.len() {
				let trans = &progression.transitions[i];
				println!();
				println!(
					"  {} {}: {}",
					"↓".bold(),
					"Transition Score".dimmed(),
					trans.score.to_string().cyan()
				);
				println!(
					"    {}: {} fingers | {}: {} | {}: {} frets",
					"Movements".dimmed(),
					trans.finger_movements,
					"Anchors".dimmed(),
					trans.common_anchors,
					"Distance".dimmed(),
					trans.position_distance
				);
				println!();
			}
		}

		println!();
	}
}

fn name_chord(fingering_str: &str, capo: Option<u8>, instrument: InstrumentChoice) -> Result<()> {
	use chordcraft_core::analyzer::analyze_fingering;
	use chordcraft_core::fingering::Fingering;

	let fingering = Fingering::parse(fingering_str)
		.with_context(|| format!("Invalid fingering notation: '{fingering_str}'"))?;

	let instrument_name = match instrument {
		InstrumentChoice::Guitar => "Guitar",
		InstrumentChoice::Ukulele => "Ukulele",
	};

	let (pitches, matches) = match instrument {
		InstrumentChoice::Guitar => {
			let guitar = Guitar::default();
			let p = fingering.unique_pitch_classes(&guitar);
			let m = analyze_fingering(&fingering, &guitar);
			(p, m)
		}
		InstrumentChoice::Ukulele => {
			let ukulele = Ukulele::default();
			let p = fingering.unique_pitch_classes(&ukulele);
			let m = analyze_fingering(&fingering, &ukulele);
			(p, m)
		}
	};

	if let Some(capo_fret) = capo {
		println!(
			"\n{} {} {} [{instrument_name}]\n",
			"Analyzing fingering:".bold(),
			fingering_str.green().bold(),
			format!("(Capo {capo_fret})").yellow()
		);
	} else {
		println!(
			"\n{} {} [{instrument_name}]\n",
			"Analyzing fingering:".bold(),
			fingering_str.green().bold()
		);
	}

	println!(
		"Notes played: {}\n",
		pitches
			.iter()
			.map(|p| p.to_string())
			.collect::<Vec<_>>()
			.join(", ")
	);

	if matches.is_empty() {
		println!("{}", "Could not identify chord (not enough notes)".yellow());
		return Ok(());
	}

	let transposed_matches: Vec<_> = if let Some(capo_fret) = capo {
		matches
			.iter()
			.map(|m| {
				let mut transposed = m.clone();
				transposed.chord = m.chord.transpose(capo_fret as i32);
				transposed
			})
			.collect()
	} else {
		matches.clone()
	};

	let top = &transposed_matches[0];
	let shape_chord = &matches[0].chord;

	if capo.is_some() {
		println!(
			"{} {} {} {}\n",
			"Best match:".bold().green(),
			top.chord.to_string().green().bold(),
			"(".dimmed(),
			format!("{shape_chord} shape)").dimmed()
		);
	} else {
		println!(
			"{} {}\n",
			"Best match:".bold().green(),
			top.chord.to_string().green().bold()
		);
	}

	println!("  Confidence: {:.0}%", top.completeness * 100.0);
	println!(
		"  Root in bass: {}",
		if top.root_in_bass {
			"Yes".green()
		} else {
			"No".yellow()
		}
	);
	println!("  Score: {}", top.score);

	if transposed_matches.len() > 1 {
		println!("\n{}", "Alternative interpretations:".bold());
		for (i, (m, shape)) in transposed_matches
			.iter()
			.zip(matches.iter())
			.skip(1)
			.take(4)
			.enumerate()
		{
			if capo.is_some() {
				let shape_name = &shape.chord;
				println!(
					"  {}. {} {} (confidence: {:.0}%, score: {})",
					i + 1,
					m.chord.to_string().cyan(),
					format!("({shape_name} shape)").dimmed(),
					m.completeness * 100.0,
					m.score
				);
			} else {
				println!(
					"  {}. {} (confidence: {:.0}%, score: {})",
					i + 1,
					m.chord.to_string().cyan(),
					m.completeness * 100.0,
					m.score
				);
			}
		}
	}

	Ok(())
}
