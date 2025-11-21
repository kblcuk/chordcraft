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
    },

    /// Identify chord from fingering notation
    Name {
        /// Tab notation (e.g., "x32010", "022100")
        fingering: String,

        /// Key context hint (e.g., "C", "G")
        #[arg(short, long)]
        key: Option<String>,
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
        } => {
            find_fingerings(&chord, limit, position, voicing)?;
        }
        Commands::Name { fingering, key } => {
            name_chord(&fingering, key)?;
        }
    }

    Ok(())
}

fn find_fingerings(
    chord_str: &str,
    limit: usize,
    position: Option<u8>,
    voicing: Option<String>,
) -> Result<()> {
    // Parse the chord
    let chord = Chord::parse(chord_str)
        .with_context(|| format!("Invalid chord name: '{}'", chord_str))?;

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

    // Generate fingerings
    let fingerings = generate_fingerings(&chord, &guitar, &options);

    if fingerings.is_empty() {
        println!(
            "{}",
            format!("No fingerings found for chord: {}", chord).yellow()
        );
        return Ok(());
    }

    // Display header
    println!(
        "\n{} {} (showing {} of {} found)\n",
        "Fingerings for".bold(),
        chord.to_string().green().bold(),
        fingerings.len().min(limit),
        fingerings.len()
    );

    // Display each fingering
    for (i, scored) in fingerings.iter().take(limit).enumerate() {
        println!("{}. {}", (i + 1).to_string().cyan().bold(), scored.fingering);
        println!("{}", format_fingering_diagram(scored, &guitar));
        println!();
    }

    Ok(())
}

fn name_chord(fingering_str: &str, key: Option<String>) -> Result<()> {
    use chordcraft_core::fingering::Fingering;

    // Parse the fingering
    let fingering = Fingering::parse(fingering_str)
        .with_context(|| format!("Invalid fingering notation: '{}'", fingering_str))?;

    let guitar = Guitar::default();

    // Get the notes
    let pitches = fingering.unique_pitch_classes(&guitar);

    println!(
        "\n{} {}\n",
        "Analyzing fingering:".bold(),
        fingering_str.green().bold()
    );

    println!("Notes played: {}", pitches.iter().map(|p| p.to_string()).collect::<Vec<_>>().join(", "));

    if let Some(k) = key {
        println!("Key context: {}", k);
    }

    println!("\n{}", "[Chord identification coming soon...]".yellow());

    Ok(())
}
