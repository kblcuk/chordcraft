//! Standard chord shapes for common instruments
//!
//! This module defines well-known chord shapes that guitarists and ukulele players
//! learn as foundational patterns. These shapes can be barred at different frets
//! to produce different chords while maintaining the same finger pattern.
//!
//! For example, the Am shape (x02210) barred at fret 2 becomes Bm (x24432).

use crate::fingering::{Fingering, StringState};

/// A standard chord shape that can be recognized and matched against fingerings.
#[derive(Debug, Clone)]
pub struct StandardShape {
	/// Name of the shape (e.g., "Am", "E", "C")
	pub name: &'static str,
	/// The relative fret pattern from the barre position.
	/// None = muted, Some(n) = n frets above the barre (0 = at barre/open)
	pub pattern: &'static [Option<u8>],
	/// Number of strings this shape uses
	pub string_count: usize,
}

impl StandardShape {
	/// Check if a fingering matches this shape when normalized to its base fret.
	/// Returns the base fret (barre position) if it matches, None otherwise.
	pub fn matches(&self, fingering: &Fingering) -> Option<u8> {
		let strings = fingering.strings();
		if strings.len() != self.string_count {
			return None;
		}

		// First, determine the base fret by looking at where Some(0) entries map to.
		// All Some(0) entries in the pattern should map to the same fret (the barre position).
		let mut base_fret: Option<u8> = None;

		for (state, expected) in strings.iter().zip(self.pattern.iter()) {
			if let (StringState::Fretted(fret), Some(0)) = (state, expected) {
				match base_fret {
					None => base_fret = Some(*fret),
					Some(bf) if bf != *fret => return None, // Inconsistent base frets
					_ => {}
				}
			}
		}

		// If no Some(0) in pattern, use the minimum fret
		let base_fret = base_fret.unwrap_or_else(|| fingering.min_fret().unwrap_or(0));

		// Now verify all entries match at base_fret + offset
		for (state, expected) in strings.iter().zip(self.pattern.iter()) {
			match (state, expected) {
				// Both muted - match
				(StringState::Muted, None) => continue,
				// Pattern expects muted but fingering is fretted - no match
				(StringState::Fretted(_), None) => return None,
				// Pattern expects fretted but fingering is muted - no match
				(StringState::Muted, Some(_)) => return None,
				// Both fretted - check relative position
				(StringState::Fretted(fret), Some(offset)) => {
					let expected_fret = base_fret + offset;
					if *fret != expected_fret {
						return None;
					}
				}
			}
		}

		Some(base_fret)
	}
}

/// Standard guitar chord shapes (6 strings, EADGBE tuning)
pub mod guitar {
	use super::StandardShape;

	/// Am shape: x02210 - very common, used for Bm, Cm, C#m, etc.
	pub const AM_SHAPE: StandardShape = StandardShape {
		name: "Am",
		pattern: &[None, Some(0), Some(2), Some(2), Some(1), Some(0)],
		string_count: 6,
	};

	/// A shape: x02220 - used for B, C, C#, etc.
	pub const A_SHAPE: StandardShape = StandardShape {
		name: "A",
		pattern: &[None, Some(0), Some(2), Some(2), Some(2), Some(0)],
		string_count: 6,
	};

	/// Em shape: 022000 - used for Fm, F#m, Gm, etc.
	pub const EM_SHAPE: StandardShape = StandardShape {
		name: "Em",
		pattern: &[Some(0), Some(2), Some(2), Some(0), Some(0), Some(0)],
		string_count: 6,
	};

	/// E shape: 022100 - the classic barre chord shape, used for F, G, A, etc.
	pub const E_SHAPE: StandardShape = StandardShape {
		name: "E",
		pattern: &[Some(0), Some(2), Some(2), Some(1), Some(0), Some(0)],
		string_count: 6,
	};

	/// C shape: x32010 - less common as barre, but recognizable
	pub const C_SHAPE: StandardShape = StandardShape {
		name: "C",
		pattern: &[None, Some(3), Some(2), Some(0), Some(1), Some(0)],
		string_count: 6,
	};

	/// G shape: 320003 - less common as barre due to stretch
	pub const G_SHAPE: StandardShape = StandardShape {
		name: "G",
		pattern: &[Some(3), Some(2), Some(0), Some(0), Some(0), Some(3)],
		string_count: 6,
	};

	/// D shape: xx0232 - partial shape, used in some voicings
	pub const D_SHAPE: StandardShape = StandardShape {
		name: "D",
		pattern: &[None, None, Some(0), Some(2), Some(3), Some(2)],
		string_count: 6,
	};

	/// Dm shape: xx0231 - partial shape for minor chords
	pub const DM_SHAPE: StandardShape = StandardShape {
		name: "Dm",
		pattern: &[None, None, Some(0), Some(2), Some(3), Some(1)],
		string_count: 6,
	};

	/// All standard guitar shapes for iteration
	pub const ALL_SHAPES: &[&StandardShape] = &[
		&AM_SHAPE, &A_SHAPE, &EM_SHAPE, &E_SHAPE, &C_SHAPE, &G_SHAPE, &D_SHAPE, &DM_SHAPE,
	];

	/// Find which standard shape a fingering matches, if any.
	/// Returns the shape name and base fret if found.
	pub fn find_matching_shape(
		fingering: &crate::fingering::Fingering,
	) -> Option<(&'static str, u8)> {
		for shape in ALL_SHAPES {
			if let Some(base_fret) = shape.matches(fingering) {
				return Some((shape.name, base_fret));
			}
		}
		None
	}
}

/// Standard ukulele chord shapes (4 strings, GCEA tuning)
pub mod ukulele {
	use super::StandardShape;

	/// A shape: 2100 - common shape, barred for Bb, B, C, etc.
	pub const A_SHAPE: StandardShape = StandardShape {
		name: "A",
		pattern: &[Some(2), Some(1), Some(0), Some(0)],
		string_count: 4,
	};

	/// Am shape: 2000 - very simple, one finger
	pub const AM_SHAPE: StandardShape = StandardShape {
		name: "Am",
		pattern: &[Some(2), Some(0), Some(0), Some(0)],
		string_count: 4,
	};

	/// C shape: 0003 - the classic ukulele C
	pub const C_SHAPE: StandardShape = StandardShape {
		name: "C",
		pattern: &[Some(0), Some(0), Some(0), Some(3)],
		string_count: 4,
	};

	/// F shape: 2010 - common shape
	pub const F_SHAPE: StandardShape = StandardShape {
		name: "F",
		pattern: &[Some(2), Some(0), Some(1), Some(0)],
		string_count: 4,
	};

	/// G shape: 0232 - common shape
	pub const G_SHAPE: StandardShape = StandardShape {
		name: "G",
		pattern: &[Some(0), Some(2), Some(3), Some(2)],
		string_count: 4,
	};

	/// D shape: 2220 - barre-friendly
	pub const D_SHAPE: StandardShape = StandardShape {
		name: "D",
		pattern: &[Some(2), Some(2), Some(2), Some(0)],
		string_count: 4,
	};

	/// Dm shape: 2210 - common minor shape
	pub const DM_SHAPE: StandardShape = StandardShape {
		name: "Dm",
		pattern: &[Some(2), Some(2), Some(1), Some(0)],
		string_count: 4,
	};

	/// E shape: 4442 (or 1402) - common barre shape
	pub const E_SHAPE: StandardShape = StandardShape {
		name: "E",
		pattern: &[Some(4), Some(4), Some(4), Some(2)],
		string_count: 4,
	};

	/// Em shape: 0432 - common minor shape
	pub const EM_SHAPE: StandardShape = StandardShape {
		name: "Em",
		pattern: &[Some(0), Some(4), Some(3), Some(2)],
		string_count: 4,
	};

	/// Bb shape: 3211 - important barre shape
	pub const BB_SHAPE: StandardShape = StandardShape {
		name: "Bb",
		pattern: &[Some(3), Some(2), Some(1), Some(1)],
		string_count: 4,
	};

	/// All standard ukulele shapes for iteration
	pub const ALL_SHAPES: &[&StandardShape] = &[
		&A_SHAPE, &AM_SHAPE, &C_SHAPE, &F_SHAPE, &G_SHAPE, &D_SHAPE, &DM_SHAPE, &E_SHAPE,
		&EM_SHAPE, &BB_SHAPE,
	];

	/// Find which standard shape a fingering matches, if any.
	/// Returns the shape name and base fret if found.
	pub fn find_matching_shape(
		fingering: &crate::fingering::Fingering,
	) -> Option<(&'static str, u8)> {
		for shape in ALL_SHAPES {
			if let Some(base_fret) = shape.matches(fingering) {
				return Some((shape.name, base_fret));
			}
		}
		None
	}
}

/// Standard mandolin chord shapes (4 strings, GDAE tuning - tuned in 5ths)
/// The 5ths tuning creates symmetric, movable shapes different from guitar.
pub mod mandolin {
	use super::StandardShape;

	/// G shape: 0023 - open G major, very common
	pub const G_SHAPE: StandardShape = StandardShape {
		name: "G",
		pattern: &[Some(0), Some(0), Some(2), Some(3)],
		string_count: 4,
	};

	/// C shape: 0230 - the classic "chop chord" position
	pub const C_SHAPE: StandardShape = StandardShape {
		name: "C",
		pattern: &[Some(0), Some(2), Some(3), Some(0)],
		string_count: 4,
	};

	/// D shape: 2002 - closed D, very movable
	pub const D_SHAPE: StandardShape = StandardShape {
		name: "D",
		pattern: &[Some(2), Some(0), Some(0), Some(2)],
		string_count: 4,
	};

	/// A shape: 2245 - common A major barre shape
	pub const A_SHAPE: StandardShape = StandardShape {
		name: "A",
		pattern: &[Some(2), Some(2), Some(4), Some(5)],
		string_count: 4,
	};

	/// E shape: 0442 - open position E major
	pub const E_SHAPE: StandardShape = StandardShape {
		name: "E",
		pattern: &[Some(0), Some(4), Some(4), Some(2)],
		string_count: 4,
	};

	/// F shape: 3553 - movable F major (like D shape moved up)
	pub const F_SHAPE: StandardShape = StandardShape {
		name: "F",
		pattern: &[Some(3), Some(5), Some(5), Some(3)],
		string_count: 4,
	};

	/// Am shape: 2200 - simple A minor
	pub const AM_SHAPE: StandardShape = StandardShape {
		name: "Am",
		pattern: &[Some(2), Some(2), Some(0), Some(0)],
		string_count: 4,
	};

	/// Em shape: 0402 - open E minor
	pub const EM_SHAPE: StandardShape = StandardShape {
		name: "Em",
		pattern: &[Some(0), Some(4), Some(0), Some(2)],
		string_count: 4,
	};

	/// Dm shape: 2001 - D minor
	pub const DM_SHAPE: StandardShape = StandardShape {
		name: "Dm",
		pattern: &[Some(2), Some(0), Some(0), Some(1)],
		string_count: 4,
	};

	/// Gm shape: 0021 - G minor
	pub const GM_SHAPE: StandardShape = StandardShape {
		name: "Gm",
		pattern: &[Some(0), Some(0), Some(2), Some(1)],
		string_count: 4,
	};

	/// All standard mandolin shapes for iteration
	pub const ALL_SHAPES: &[&StandardShape] = &[
		&G_SHAPE, &C_SHAPE, &D_SHAPE, &A_SHAPE, &E_SHAPE, &F_SHAPE, &AM_SHAPE, &EM_SHAPE,
		&DM_SHAPE, &GM_SHAPE,
	];

	/// Find which standard shape a fingering matches, if any.
	/// Returns the shape name and base fret if found.
	pub fn find_matching_shape(
		fingering: &crate::fingering::Fingering,
	) -> Option<(&'static str, u8)> {
		for shape in ALL_SHAPES {
			if let Some(base_fret) = shape.matches(fingering) {
				return Some((shape.name, base_fret));
			}
		}
		None
	}
}

/// Standard banjo chord shapes (5 strings, gDGBD open G tuning)
/// The 5th string (high G drone) is often left open or not fretted.
/// String order: g (drone), D, G, B, D
pub mod banjo {
	use super::StandardShape;

	/// G shape: 00000 - all open strings (the beauty of open G tuning!)
	pub const G_SHAPE: StandardShape = StandardShape {
		name: "G",
		pattern: &[Some(0), Some(0), Some(0), Some(0), Some(0)],
		string_count: 5,
	};

	/// C shape: x2012 - common C major with drone muted
	pub const C_SHAPE: StandardShape = StandardShape {
		name: "C",
		pattern: &[None, Some(2), Some(0), Some(1), Some(2)],
		string_count: 5,
	};

	/// C shape alternate: 02012 - C major with open drone
	pub const C_SHAPE_ALT: StandardShape = StandardShape {
		name: "C-alt",
		pattern: &[Some(0), Some(2), Some(0), Some(1), Some(2)],
		string_count: 5,
	};

	/// D shape: x0024 - common D major
	pub const D_SHAPE: StandardShape = StandardShape {
		name: "D",
		pattern: &[None, Some(0), Some(0), Some(2), Some(4)],
		string_count: 5,
	};

	/// D7 shape: x0020 - D7 chord
	pub const D7_SHAPE: StandardShape = StandardShape {
		name: "D7",
		pattern: &[None, Some(0), Some(0), Some(2), Some(0)],
		string_count: 5,
	};

	/// Em shape: x0002 - E minor
	pub const EM_SHAPE: StandardShape = StandardShape {
		name: "Em",
		pattern: &[None, Some(0), Some(0), Some(0), Some(2)],
		string_count: 5,
	};

	/// Am shape: x2200 - A minor
	pub const AM_SHAPE: StandardShape = StandardShape {
		name: "Am",
		pattern: &[None, Some(2), Some(2), Some(0), Some(0)],
		string_count: 5,
	};

	/// F shape: x3211 - F major barre (pattern relative to barre at fret 1)
	pub const F_SHAPE: StandardShape = StandardShape {
		name: "F",
		pattern: &[None, Some(2), Some(1), Some(0), Some(0)],
		string_count: 5,
	};

	/// A shape: x2222 - A major barre (all strings at same fret)
	pub const A_SHAPE: StandardShape = StandardShape {
		name: "A",
		pattern: &[None, Some(0), Some(0), Some(0), Some(0)],
		string_count: 5,
	};

	/// Bm shape: x4432 - B minor (pattern relative to barre at fret 2)
	pub const BM_SHAPE: StandardShape = StandardShape {
		name: "Bm",
		pattern: &[None, Some(2), Some(2), Some(1), Some(0)],
		string_count: 5,
	};

	/// E shape: x2100 - E major
	pub const E_SHAPE: StandardShape = StandardShape {
		name: "E",
		pattern: &[None, Some(2), Some(1), Some(0), Some(0)],
		string_count: 5,
	};

	/// All standard banjo shapes for iteration
	pub const ALL_SHAPES: &[&StandardShape] = &[
		&G_SHAPE,
		&C_SHAPE,
		&C_SHAPE_ALT,
		&D_SHAPE,
		&D7_SHAPE,
		&EM_SHAPE,
		&AM_SHAPE,
		&F_SHAPE,
		&A_SHAPE,
		&BM_SHAPE,
		&E_SHAPE,
	];

	/// Find which standard shape a fingering matches, if any.
	/// Returns the shape name and base fret if found.
	pub fn find_matching_shape(
		fingering: &crate::fingering::Fingering,
	) -> Option<(&'static str, u8)> {
		for shape in ALL_SHAPES {
			if let Some(base_fret) = shape.matches(fingering) {
				return Some((shape.name, base_fret));
			}
		}
		None
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::fingering::Fingering;

	#[test]
	fn test_am_shape_open() {
		let am = Fingering::parse("x02210").unwrap();
		let result = guitar::AM_SHAPE.matches(&am);
		assert_eq!(result, Some(0), "Open Am should match Am shape at fret 0");
	}

	#[test]
	fn test_am_shape_as_bm() {
		let bm = Fingering::parse("x24432").unwrap();
		let result = guitar::AM_SHAPE.matches(&bm);
		assert_eq!(
			result,
			Some(2),
			"Bm (x24432) should match Am shape at fret 2"
		);
	}

	#[test]
	fn test_am_shape_as_csm() {
		let csm = Fingering::parse("x46654").unwrap();
		let result = guitar::AM_SHAPE.matches(&csm);
		assert_eq!(
			result,
			Some(4),
			"C#m (x46654) should match Am shape at fret 4"
		);
	}

	#[test]
	fn test_e_shape_open() {
		let e = Fingering::parse("022100").unwrap();
		let result = guitar::E_SHAPE.matches(&e);
		assert_eq!(result, Some(0), "Open E should match E shape at fret 0");
	}

	#[test]
	fn test_e_shape_as_f() {
		let f = Fingering::parse("133211").unwrap();
		let result = guitar::E_SHAPE.matches(&f);
		assert_eq!(result, Some(1), "F (133211) should match E shape at fret 1");
	}

	#[test]
	fn test_e_shape_as_g() {
		let g = Fingering::parse("355433").unwrap();
		let result = guitar::E_SHAPE.matches(&g);
		assert_eq!(
			result,
			Some(3),
			"G barre (355433) should match E shape at fret 3"
		);
	}

	#[test]
	fn test_em_shape_as_fm() {
		let fm = Fingering::parse("133111").unwrap();
		let result = guitar::EM_SHAPE.matches(&fm);
		assert_eq!(
			result,
			Some(1),
			"Fm (133111) should match Em shape at fret 1"
		);
	}

	#[test]
	fn test_a_shape_as_b() {
		let b = Fingering::parse("x24442").unwrap();
		let result = guitar::A_SHAPE.matches(&b);
		assert_eq!(result, Some(2), "B (x24442) should match A shape at fret 2");
	}

	#[test]
	fn test_no_match_for_nonstandard() {
		let weird = Fingering::parse("x20402").unwrap();
		let result = guitar::find_matching_shape(&weird);
		assert!(
			result.is_none(),
			"Unusual voicing should not match any standard shape"
		);
	}

	#[test]
	fn test_find_matching_shape() {
		let bm = Fingering::parse("x24432").unwrap();
		let result = guitar::find_matching_shape(&bm);
		assert_eq!(result, Some(("Am", 2)), "Should find Am shape at fret 2");

		let f = Fingering::parse("133211").unwrap();
		let result = guitar::find_matching_shape(&f);
		assert_eq!(result, Some(("E", 1)), "Should find E shape at fret 1");
	}

	// Ukulele tests
	#[test]
	fn test_ukulele_c_shape() {
		let c = Fingering::parse("0003").unwrap();
		let result = ukulele::C_SHAPE.matches(&c);
		assert_eq!(result, Some(0), "Ukulele C should match C shape at fret 0");
	}

	#[test]
	fn test_ukulele_a_shape() {
		let a = Fingering::parse("2100").unwrap();
		let result = ukulele::A_SHAPE.matches(&a);
		assert_eq!(result, Some(0), "Ukulele A should match A shape at fret 0");
	}

	#[test]
	fn test_ukulele_a_shape_as_bb() {
		let bb = Fingering::parse("3211").unwrap();
		let result = ukulele::A_SHAPE.matches(&bb);
		assert_eq!(result, Some(1), "Ukulele Bb should match A shape at fret 1");
	}

	#[test]
	fn test_ukulele_find_matching_shape() {
		let dm = Fingering::parse("2210").unwrap();
		let result = ukulele::find_matching_shape(&dm);
		assert_eq!(result, Some(("Dm", 0)), "Should find Dm shape at fret 0");
	}

	// Mandolin tests
	#[test]
	fn test_mandolin_g_shape() {
		let g = Fingering::parse("0023").unwrap();
		let result = mandolin::G_SHAPE.matches(&g);
		assert_eq!(result, Some(0), "Mandolin G should match G shape at fret 0");
	}

	#[test]
	fn test_mandolin_d_shape() {
		let d = Fingering::parse("2002").unwrap();
		let result = mandolin::D_SHAPE.matches(&d);
		assert_eq!(result, Some(0), "Mandolin D should match D shape at fret 0");
	}

	#[test]
	fn test_mandolin_d_shape_as_e() {
		// D shape moved up 2 frets becomes E
		let e = Fingering::parse("4224").unwrap();
		let result = mandolin::D_SHAPE.matches(&e);
		assert_eq!(result, Some(2), "Mandolin E should match D shape at fret 2");
	}

	#[test]
	fn test_mandolin_c_shape() {
		let c = Fingering::parse("0230").unwrap();
		let result = mandolin::C_SHAPE.matches(&c);
		assert_eq!(result, Some(0), "Mandolin C should match C shape at fret 0");
	}

	#[test]
	fn test_mandolin_find_matching_shape() {
		let am = Fingering::parse("2200").unwrap();
		let result = mandolin::find_matching_shape(&am);
		assert_eq!(result, Some(("Am", 0)), "Should find Am shape at fret 0");
	}

	// Banjo tests
	#[test]
	fn test_banjo_g_shape() {
		let g = Fingering::parse("00000").unwrap();
		let result = banjo::G_SHAPE.matches(&g);
		assert_eq!(result, Some(0), "Banjo G should match G shape (all open)");
	}

	#[test]
	fn test_banjo_c_shape() {
		let c = Fingering::parse("x2012").unwrap();
		let result = banjo::C_SHAPE.matches(&c);
		assert_eq!(result, Some(0), "Banjo C should match C shape at fret 0");
	}

	#[test]
	fn test_banjo_d_shape() {
		let d = Fingering::parse("x0024").unwrap();
		let result = banjo::D_SHAPE.matches(&d);
		assert_eq!(result, Some(0), "Banjo D should match D shape at fret 0");
	}

	#[test]
	fn test_banjo_f_shape_as_g() {
		// F shape (x3211) at fret 1, moved up 2 frets to fret 3 becomes x5433
		let g_barre = Fingering::parse("x5433").unwrap();
		let result = banjo::F_SHAPE.matches(&g_barre);
		assert_eq!(
			result,
			Some(3),
			"Banjo barre at fret 3 should match F shape"
		);

		// Original F shape at fret 1
		let f = Fingering::parse("x3211").unwrap();
		let result = banjo::F_SHAPE.matches(&f);
		assert_eq!(result, Some(1), "Banjo F should match F shape at fret 1");
	}

	#[test]
	fn test_banjo_find_matching_shape() {
		let em = Fingering::parse("x0002").unwrap();
		let result = banjo::find_matching_shape(&em);
		assert_eq!(result, Some(("Em", 0)), "Should find Em shape at fret 0");
	}
}
