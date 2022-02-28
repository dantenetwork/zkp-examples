#![allow(unused_imports)]
#![allow(unused_variables)]
extern crate bellman;
extern crate pairing;
extern crate rand;
// We'll use these interfaces to construct our circuit.
use bellman::{Circuit, ConstraintSystem, SynthesisError};

// We're going to use the BLS12-381 pairing-friendly elliptic curve.
use pairing::{Engine, Field, PrimeField};

mod cube;

fn main() {
  // We're going to use the Groth16 proving system.
  use bellman::groth16::{
    create_random_proof, generate_random_parameters, prepare_verifying_key, verify_proof, Proof,
  };

  // We're going to use the BLS12-381 pairing-friendly elliptic curve.
  use pairing::bls12_381::{Bls12, Fr};

  // For randomness (during paramgen and proof generation)
  use rand::thread_rng;

  println!("Prove that I know x such that x^3 + x + 8 == 38.");

  let rng = &mut thread_rng();

  println!("Creating parameters...");

  // Create parameters for our circuit
  let params = {
    let c = cube::Cube::<Bls12> { x: None };

    generate_random_parameters(c, rng).unwrap()
  };
  println!("Params: {}", &params.vk.alpha_g1);

  // Prepare the verification key (for proof verification)
  let pvk = prepare_verifying_key(&params.vk);

  println!("Creating proofs...");

  // Create an instance of circuit
  let circuit = cube::Cube::<Bls12> {
    x: Fr::from_str("3"),
  };

  // Create a groth16 proof with our parameters.
  let proof = create_random_proof(circuit, &params, rng).unwrap();

  assert!(verify_proof(&pvk, &proof, &[Fr::from_str("38").unwrap()]).unwrap());
}
