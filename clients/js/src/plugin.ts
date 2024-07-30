import { UmiPlugin } from '@metaplex-foundation/umi';
import { createSolanaSerializationBenchmarkProgram } from './generated';

export const solanaSerializationBenchmark = (): UmiPlugin => ({
  install(umi) {
    umi.programs.add(createSolanaSerializationBenchmarkProgram(), false);
  },
});
