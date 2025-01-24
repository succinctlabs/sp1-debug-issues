use std::{fs::File, io::Read};

use anyhow::{bail, Result};
use clap::{Parser, ValueEnum};
use hex_literal::hex;
use sha2::{Digest, Sha256};
use sp1_sdk::{Prover, ProverMode, SP1ProofMode, SP1Stdin};
use universal_prover::UniversalProver;

mod universal_prover;

const FIBONACCI_PROGRAM_HASH: [u8; 32] =
    hex!("45d1d3889583ac22808eb5be7316fdeb2a936a3da1ca4931d462e4706833fc2e");

const FIBONACCI_STDIN_HASH: [u8; 32] =
    hex!("47ff30c843f47b09affa3f52f5b0035959540cc9d523c718577bb5c29933e726");

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
    sp1_sdk::utils::setup_logger();
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

    // Verify the proof.
    client.verify(&proof, &vk).expect("failed to verify proof");
    println!("Successfully verified proof!");

    Ok(())
}

fn read_program() -> Result<Vec<u8>> {
    let mut file = File::open("program.bin")?;
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer)?;

    let mut hasher = Sha256::new();
    hasher.update(&buffer);
    let hash = hasher.finalize();

    if hash[..] == FIBONACCI_PROGRAM_HASH {
        bail!("Please replace the program.bin file with yours.");
    }

    Ok(buffer)
}

fn read_stdin() -> Result<SP1Stdin> {
    let mut file = File::open("stdin.bin")?;
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer)?;

    let mut hasher = Sha256::new();
    hasher.update(&buffer);
    let hash = hasher.finalize();

    if hash[..] == FIBONACCI_STDIN_HASH {
        bail!("Please replace the sdtin.bin file with yours.");
    }

    let stdin = bincode::deserialize(&buffer)?;

    Ok(stdin)
}
