use anyhow::Result;
use sp1_prover::components::CpuProverComponents;
use sp1_sdk::{CpuProver, CudaProver, NetworkProver, Prover, ProverClient, ProverMode};

pub enum UniversalProver {
    Cpu(CpuProver),
    Cuda(CudaProver),
    Network(NetworkProver),
    Mock(CpuProver),
}

impl UniversalProver {
    pub fn new(mode: &ProverMode) -> Self {
        match mode {
            ProverMode::Cpu => UniversalProver::Cpu(ProverClient::builder().cpu().build()),
            ProverMode::Cuda => UniversalProver::Cuda(ProverClient::builder().cuda().build()),
            ProverMode::Network => {
                UniversalProver::Network(ProverClient::builder().network().build())
            }
            ProverMode::Mock => UniversalProver::Mock(ProverClient::builder().mock().build()),
        }
    }
}

impl Prover<CpuProverComponents> for UniversalProver {
    fn inner(&self) -> &sp1_sdk::SP1Prover<CpuProverComponents> {
        match self {
            UniversalProver::Cpu(prover) => prover.inner(),
            UniversalProver::Cuda(prover) => prover.inner(),
            UniversalProver::Network(prover) => prover.inner(),
            UniversalProver::Mock(prover) => prover.inner(),
        }
    }

    fn setup(&self, elf: &[u8]) -> (sp1_sdk::SP1ProvingKey, sp1_sdk::SP1VerifyingKey) {
        match self {
            UniversalProver::Cpu(prover) => prover.setup(elf),
            UniversalProver::Cuda(prover) => prover.setup(elf),
            UniversalProver::Network(prover) => prover.setup(elf),
            UniversalProver::Mock(prover) => prover.setup(elf),
        }
    }

    fn prove(
        &self,
        pk: &sp1_sdk::SP1ProvingKey,
        stdin: &sp1_sdk::SP1Stdin,
        mode: sp1_sdk::SP1ProofMode,
    ) -> Result<sp1_sdk::SP1ProofWithPublicValues> {
        match self {
            UniversalProver::Cpu(prover) => {
                Prover::<CpuProverComponents>::prove(prover, pk, stdin, mode)
            }
            UniversalProver::Cuda(prover) => {
                Prover::<CpuProverComponents>::prove(prover, pk, stdin, mode)
            }
            UniversalProver::Network(prover) => {
                Prover::<CpuProverComponents>::prove(prover, pk, stdin, mode)
            }
            UniversalProver::Mock(prover) => {
                Prover::<CpuProverComponents>::prove(prover, pk, stdin, mode)
            }
        }
    }
}
