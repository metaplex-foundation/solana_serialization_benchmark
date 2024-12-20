/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import {
  ClusterFilter,
  Context,
  Program,
  PublicKey,
} from '@metaplex-foundation/umi';
import {
  getSolanaSerializationBenchmarkErrorFromCode,
  getSolanaSerializationBenchmarkErrorFromName,
} from '../errors';

export const SOLANA_SERIALIZATION_BENCHMARK_PROGRAM_ID =
  'BENCHVr3SC7dVDMtKVpwctjFNPBMrqvXn9JVACJg3KEb' as PublicKey<'BENCHVr3SC7dVDMtKVpwctjFNPBMrqvXn9JVACJg3KEb'>;

export function createSolanaSerializationBenchmarkProgram(): Program {
  return {
    name: 'solanaSerializationBenchmark',
    publicKey: SOLANA_SERIALIZATION_BENCHMARK_PROGRAM_ID,
    getErrorFromCode(code: number, cause?: Error) {
      return getSolanaSerializationBenchmarkErrorFromCode(code, this, cause);
    },
    getErrorFromName(name: string, cause?: Error) {
      return getSolanaSerializationBenchmarkErrorFromName(name, this, cause);
    },
    isOnCluster() {
      return true;
    },
  };
}

export function getSolanaSerializationBenchmarkProgram<
  T extends Program = Program,
>(context: Pick<Context, 'programs'>, clusterFilter?: ClusterFilter): T {
  return context.programs.get<T>('solanaSerializationBenchmark', clusterFilter);
}

export function getSolanaSerializationBenchmarkProgramId(
  context: Pick<Context, 'programs'>,
  clusterFilter?: ClusterFilter
): PublicKey {
  return context.programs.getPublicKey(
    'solanaSerializationBenchmark',
    SOLANA_SERIALIZATION_BENCHMARK_PROGRAM_ID,
    clusterFilter
  );
}
