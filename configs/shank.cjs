const path = require("path");
const { generateIdl } = require("@metaplex-foundation/shank-js");

const idlDir = path.join(__dirname, "..", "idls");
const binaryInstallDir = path.join(__dirname, "..", ".crates");
const programDir = path.join(__dirname, "..", "programs");

generateIdl({
  generator: "shank",
  programName: "solana_serialization_benchmark_program",
  programId: "BENCHVr3SC7dVDMtKVpwctjFNPBMrqvXn9JVACJg3KEb",
  idlDir,
  binaryInstallDir,
  programDir: path.join(programDir, "solana-serialization-benchmark"),
});
