pragma circom 2.0.0;

include "../circomlib/circuits/bitify.circom";
include "../circomlib/circuits/escalarmulfix.circom";

template privateKeyProof() {
  signal input in;
  signal output out[2];

  var BASE8[2] = [
    5299619240641551281634865583518297030282874472190772894086521144482721001553,
    16950150798460657717958625567821834550301663161624707787222815936182638968203
  ];

  component privBits = Num2Bits(253);
  privBits.in <== in;

  component mulFix = EscalarMulFix(253, BASE8);

  for (var i = 0; i < 253; i++) {
    mulFix.e[i] <== privBits.out[i];
  }

  out[0] <== mulFix.out[0];
  out[1] <== mulFix.out[1];
}

component main = privateKeyProof();