# Bellman examples 

Example of circuit for [bellman](https://github.com/zkcrypto/bellman/), a Rust zk-SNARKs library.

Bellman provides a `Circuit` trait which you can use to synthesize the constraints in your program.
 
 `cube.rs` contains a circuit for the statement "I know `x` such that `x^3 + x + 8 == 38`"
 
### Constructing a circuit  

* 1.first flatten your program into its constituent steps. 

* 2.Allocate the variables, then enforce the constraints. 

* 3.Enforcing the constraint takes the form of `A * B = C`. (is a linear combination, vectors of all your variables)

* 4.The `lc` in the `cs.enforce` function stands for "linear combination", and is an inner product of all the variables with some vector of coefficients.

### Generating Parameters 

* This example use the function `generate_random_parameters` to generate a random set of parameters for testing. For real use cases, these parameters would have to be generated securely, through a multi-party computation. 

### Creating a proof

* To create a proof, instantiate a version of the struct that is passed into the circuit, with the inputs to the circuit. 

* In this example, the function `create_random_proof` is used to create a random groth16 proof. 

### Verifying a proof

* To verify a proof, prepare the verifying key by passing in `params.vk` to `prepare_verifying_key`. This gives you the prepared viewing key, `pvk`.

* The function `verify_proof` takes the prepared viewing key `pvk`, the `proof`, and the output as an array.

## Running 

`cargo build`

`cargo test` runs test proofs using both example circuits. Tests are located at the bottom of their source files.

`cargo run` runs the `cube.rs` example proof in the main file.


- This is the same example used in [bellman-examples](https://github.com/arcalinea/bellman-examples).