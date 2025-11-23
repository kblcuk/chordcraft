use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use colored::Colorize;

use chordcraft_core::chord::{Chord, VoicingType};
use chordcraft_core::generator::{format_fingering_diagram, generate_fingerings, GeneratorOptions};
use chordcraft_core::instrument::Guitar;

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

        /// Capo position (fret number)
        #[arg(short, long)]
        capo: Option<u8>,
    },

    /// Identify chord from fingering notation
    Name {
        /// Tab notation (e.g., "x32010", "022100")
        fingering: String,

        /// Capo position (fret number)
        #[arg(short, long)]
        capo: Option<u8>,
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
            capo,
        } => {
            find_fingerings(&chord, limit, position, voicing, capo)?;
        }
        Commands::Name { fingering, capo } => {
            name_chord(&fingering, capo)?;
        }
    }

    Ok(())
}

fn find_fingerings(
    chord_str: &str,
    limit: usize,
    position: Option<u8>,
    voicing: Option<String>,
    capo: Option<u8>,
) -> Result<()> {
    // Parse the chord
    let original_chord = Chord::parse(chord_str)
        .with_context(|| format!("Invalid chord name: '{chord_str}'"))?;

    // If using capo, we generate fingerings for the SHAPE chord (transposed down)
    // Otherwise, generate for the actual chord
    let (search_chord, shape_chord) = if let Some(capo_fret) = capo {
        let shape = original_chord.transpose(-(capo_fret as i32));
        (shape.clone(), Some(shape))
    } else {
        (original_chord.clone(), None)
    };

    // Parse voicing type
    let voicing_type = voicing.as_ref().and_then(|v| match v.to_lowercase().as_str() {
        "core" => Some(VoicingType::Core),
        "full" => Some(VoicingType::Full),
        "jazzy" | "jazz" => Some(VoicingType::Jazzy),
        _ => None,
    });

    // Set up options
    let options = GeneratorOptions {
        limit,
        preferred_position: position,
        voicing_type,
        ..Default::default()
    };

    // Use standard guitar
    let guitar = Guitar::default();

    // Generate fingerings for the search chord (shape when using capo)
    let fingerings = generate_fingerings(&search_chord, &guitar, &options);

    if fingerings.is_empty() {
        println!(
            "{}",
            format!("No fingerings found for chord: {original_chord}").yellow()
        );
        return Ok(());
    }

    // Display header
    if let Some(shape) = shape_chord {
        // Show both actual chord and the shape being used
        println!(
            "\n{} {} {} (showing {} of {} found)",
            "Fingerings for".bold(),
            chord_str.green().bold(),
            format!("(Capo {})", capo.unwrap()).yellow(),
            fingerings.len().min(limit),
            fingerings.len()
        );
        println!(
            "{} {}\n",
            "Shape:".dimmed(),
            shape.to_string().cyan()
        );
    } else {
        println!(
            "\n{} {} (showing {} of {} found)\n",
            "Fingerings for".bold(),
            original_chord.to_string().green().bold(),
            fingerings.len().min(limit),
            fingerings.len()
        );
    }

    // Display each fingering
    for (i, scored) in fingerings.iter().take(limit).enumerate() {
        println!("{}. {}", (i + 1).to_string().cyan().bold(), scored.fingering);
        println!("{}", format_fingering_diagram(scored, &guitar));
        println!();
    }

    Ok(())
}

fn name_chord(fingering_str: &str) -> Result<()> {
    use chordcraft_core::fingering::Fingering;
    use chordcraft_core::analyzer::analyze_fingering;

    // Parse the fingering
    let fingering = Fingering::parse(fingering_str)
        .with_context(|| format!("Invalid fingering notation: '{fingering_str}'"))?;

    let guitar = Guitar::default();

    // Get the notes
    let pitches = fingering.unique_pitch_classes(&guitar);

    println!(
        "\n{} {}\n",
        "Analyzing fingering:".bold(),
        fingering_str.green().bold()
    );

    println!("Notes played: {}\n", pitches.iter().map(|p| p.to_string()).collect::<Vec<_>>().join(", "));

    // Analyze the fingering
    let matches = analyze_fingering(&fingering, &guitar);

    if matches.is_empty() {
        println!("{}", "Could not identify chord (not enough notes)".yellow());
        return Ok(());
    }

    // Display the top match
    let top = &matches[0];
    println!(
        "{} {}\n",
        "Best match:".bold().green(),
        top.chord.to_string().green().bold()
    );

    println!("  Confidence: {:.0}%", top.completeness * 100.0);
    println!("  Root in bass: {}", if top.root_in_bass { "Yes".green() } else { "No".yellow() });
    println!("  Score: {}", top.score);

    // Display alternatives if there are any
    if matches.len() > 1 {
        println!("\n{}", "Alternative interpretations:".bold());
        for (i, m) in matches.iter().skip(1).take(4).enumerate() {
            println!(
                "  {}. {} (confidence: {:.0}%, score: {})",
                i + 1,
                m.chord.to_string().cyan(),
                m.completeness * 100.0,
                m.score
            );
        }
    }

    Ok(())
}
