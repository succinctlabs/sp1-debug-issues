use std::{fs::File, io::Read};

use anyhow::Result;
use clap::Parser;
use sp1_sdk::{Prover, ProverMode, SP1ProofMode, SP1Stdin};
use universal_prover::UniversalProver;

mod universal_prover;

/// The arguments for the command.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    pub mode: ProverMode,
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
        .prove(&pk, &stdin, SP1ProofMode::Core)
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
