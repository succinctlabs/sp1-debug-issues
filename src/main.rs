use std::{fs::File, io::Read};

use clap::{Parser, ValueEnum};
use sp1_sdk::{Prover, ProverMode, SP1ProofMode, SP1Stdin};
use universal_prover::UniversalProver;

mod universal_prover;

/// The arguments for the command.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The mode to run the prover in.
    mode: ProverMode,

    /// The proof mode to use.
    #[arg(long, value_enum, default_value_t = ProofMode::Compressed)]
    proof_mode: ProofMode,
}

#[derive(Debug, Clone, ValueEnum)]
enum ProofMode {
    Core,
    Compressed,
    Plonk,
    Groth16,
}

impl From<ProofMode> for SP1ProofMode {
    fn from(mode: ProofMode) -> Self {
        match mode {
            ProofMode::Core => SP1ProofMode::Core,
            ProofMode::Compressed => SP1ProofMode::Compressed,
            ProofMode::Plonk => SP1ProofMode::Plonk,
            ProofMode::Groth16 => SP1ProofMode::Groth16,
        }
    }
}

fn main() -> Result<()> {
    dotenv::dotenv().ok();

    // Parse the command line arguments.
    let args = Args::parse();
    let client = UniversalProver::new(&args.mode);
    let program = read_program()?;
    let stdin = read_stdin()?;

    // Setup the program for proving.
    let (pk, vk) = client.setup(&program);

    let proof = client
        .prove(&pk, &stdin, args.proof_mode.into())
        .expect("failed to generate proof");
    println!("Successfully generated proof!");

    if !matches!(args.mode, ProverMode::Mock) {
        // Verify the proof.
        client.verify(&proof, &vk).expect("failed to verify proof");
        println!("Successfully verified proof!");
    }

    Ok(())
}

fn read_program() -> Result<Vec<u8>> {
    let mut file = File::open("program.bin")?;
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer)?;

    Ok(buffer)
}

fn read_stdin() -> Result<SP1Stdin> {
    let mut file = File::open("stdin.bin")?;
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer)?;

    let stdin = bincode::deserialize(&buffer)?;

    Ok(stdin)
}
