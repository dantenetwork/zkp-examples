## Circom and SnarkJS Examples - Solidity

Example of circuit for [circom](https://github.com/iden3/circom), a Javascript zk-SNARKs library.

`privateKeyProof.circom` contains a circuit for the statement "I know the `private key` corresponding to this `public key`".

#### [1.Installing the circom ecosystem](https://docs.circom.io/getting-started/installation/)

#### [2.Review circuits](./ethereum/privateKeyProof/privateKeyProof.circom)

#### [3.Generate trusted setup](https://docs.circom.io/getting-started/proving-circuits/)

3.1 start a new "powers of tau" ceremony
```
mkdir setup
cd setup
snarkjs powersoftau new bn128 12 pot12_0000.ptau -v
```

3.2 contribute to the ceremony
```
snarkjs powersoftau contribute pot12_0000.ptau pot12_0001.ptau --name="First contribution" -v
```

3.3 start the generation of phase 2
```
snarkjs powersoftau prepare phase2 pot12_0001.ptau pot12_final.ptau -v
```

3.4 start a new zkey
```
snarkjs groth16 setup ../privateKeyProof.r1cs pot12_final.ptau privateKeyProof_0000.zkey
```

3.5 Contribute to the phase 2 of the ceremony
```
snarkjs zkey contribute privateKeyProof_0000.zkey privateKeyProof_0001.zkey --name="1st Contributor Name" -v
```

3.6 Export the verification key
```
snarkjs zkey export verificationkey privateKeyProof_0001.zkey verification_key.json
```

#### 4.Compiling circuit
```
cd ethereum/privateKeyProof
circom privateKeyProof.circom --r1cs --wasm --sym --c
```

#### 5.Create input.json

```
{
  "in": 8763488322167937039616325905516046217694264098671987087929565332380420898361
}
```

#### 6.Computing witness

```
node privateKeyProof_js/generate_witness.js privateKeyProof_js/privateKeyProof.wasm input.json witness.wtns
```

#### 7.Generating Proof
```
snarkjs groth16 prove setup/privateKeyProof_0001.zkey witness.wtns proof.json public.json
```

#### 8.Verifying Proof
```
node privateKeyProof_js/generate_witness.js privateKeyProof_js/privateKeyProof.wasm input.json witness.wtns
```

## Verifying from a Smart Contract

#### 1.generate the Solidity code
```
snarkjs zkey export solidityverifier setup/privateKeyProof_0001.zkey verifier.sol
```

#### 2.generate the parameters of the call 
```
snarkjs generatecall
```
