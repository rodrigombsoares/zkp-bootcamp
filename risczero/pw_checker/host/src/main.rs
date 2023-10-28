use methods::{PW_CHECKER_ELF, PW_CHECKER_ID};
use risc0_zkvm::sha::Digest;
use risc0_zkvm::{default_prover, ExecutorEnv};
use risc0_zkvm::serde::{from_slice, to_vec};

fn main() {
    // First, we construct an executor environment
    // let env = ExecutorEnv::builder().build().unwrap();

    // TODO: add guest input to the executor environment using
    // ExecutorEnvBuilder::add_input().
    // To access this method, you'll need to use the alternate construction
    // ExecutorEnv::builder(), which creates an ExecutorEnvBuilder. When you're
    // done adding input, call ExecutorEnvBuilder::build().

    // For example:
    let pw = String::from("MyPas!s");
    let env = ExecutorEnv::builder().add_input(&to_vec(&pw).unwrap()).build().unwrap();

    // Obtain the default prover.
    let prover = default_prover();

    // Produce a receipt by proving the specified ELF binary.
    let receipt = prover.prove_elf(env, PW_CHECKER_ELF).unwrap();

    // Code for transmitting or serializing the receipt for other parties (print)
    let digest: Digest = from_slice(&receipt.journal).unwrap();
    println!("This hash has a special char: {}", digest);

    // Optional: Verify receipt to confirm that recipients will also be able to
    // verify your receipt
    receipt.verify(PW_CHECKER_ID).unwrap();
}
