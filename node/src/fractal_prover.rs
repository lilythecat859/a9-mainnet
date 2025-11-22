use ark_ff::PrimeField;
use ark_stark::{prove, verify, ProverParams};
use blake3::Hasher;

pub fn prove_fractal<F: PrimeField>(input: &[F]) -> Vec<u8> {
    let params = ProverParams::new(256, 16, 20); // security, blow-up, fri_degree
    prove(input, params)
}

pub fn verify_fractal<F: PrimeField>(proof: &[u8], claimed: &[F]) -> bool {
    verify(proof, claimed, ProverParams::default())
}
