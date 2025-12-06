use chordcraft_core::{
    chord::Chord,
    generator::{generate_fingerings, GeneratorOptions},
    instrument::Guitar,
};
use std::time::Instant;

fn main() {
    let guitar = Guitar::default();
    let options = GeneratorOptions::default();

    // Simple chord
    println!("=== Simple Chord (C major) ===");
    let chord = Chord::parse("C").unwrap();
    let start = Instant::now();
    for _ in 0..100 {
        let _ = generate_fingerings(&chord, &guitar, &options);
    }
    let elapsed = start.elapsed();
    println!("100 iterations: {elapsed:?}");
    println!("Per iteration: {:?}", elapsed / 100);

    // Complex 7th chord
    println!("\n=== 7th Chord (Cmaj7) ===");
    let chord = Chord::parse("Cmaj7").unwrap();
    let start = Instant::now();
    for _ in 0..100 {
        let _ = generate_fingerings(&chord, &guitar, &options);
    }
    let elapsed = start.elapsed();
    println!("100 iterations: {elapsed:?}");
    println!("Per iteration: {:?}", elapsed / 100);

    // Very complex chord
    println!("\n=== Complex Extended Chord (Cmaj9) ===");
    let chord = Chord::parse("Cmaj9").unwrap();
    let start = Instant::now();
    for _ in 0..100 {
        let _ = generate_fingerings(&chord, &guitar, &options);
    }
    let elapsed = start.elapsed();
    println!("100 iterations: {elapsed:?}");
    println!("Per iteration: {:?}", elapsed / 100);

    // Chord analysis
    println!("\n=== Chord Analysis (name command) ===");
    let fingering = chordcraft_core::fingering::Fingering::parse("x32010").unwrap();
    let start = Instant::now();
    for _ in 0..1000 {
        let _ = chordcraft_core::analyzer::analyze_fingering(&fingering, &guitar);
    }
    let elapsed = start.elapsed();
    println!("1000 iterations: {elapsed:?}");
    println!("Per iteration: {:?}", elapsed / 1000);

    // Memory test - generate many fingerings
    println!("\n=== Memory Test - Generate All Common Chords ===");
    let chords = vec![
        "C", "D", "E", "F", "G", "A", "B", "Cm", "Dm", "Em", "Fm", "Gm", "Am", "Bm", "Cmaj7",
        "Dmaj7", "Emaj7", "Fmaj7", "Gmaj7", "Amaj7", "Bmaj7", "C7", "D7", "E7", "F7", "G7", "A7",
        "B7", "Cm7", "Dm7", "Em7", "Fm7", "Gm7", "Am7", "Bm7",
    ];

    let start = Instant::now();
    let mut total_fingerings = 0;
    for chord_name in &chords {
        let chord = Chord::parse(chord_name).unwrap();
        let fingerings = generate_fingerings(&chord, &guitar, &options);
        total_fingerings += fingerings.len();
    }
    let elapsed = start.elapsed();
    println!("Generated {} chords: {} total fingerings", chords.len(), total_fingerings);
    println!("Total time: {elapsed:?}");
    println!("Per chord: {:?}", elapsed / chords.len() as u32);
}
