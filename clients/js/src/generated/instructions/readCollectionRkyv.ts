/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import {
  Context,
  Pda,
  PublicKey,
  TransactionBuilder,
  transactionBuilder,
} from '@metaplex-foundation/umi';
import {
  Serializer,
  mapSerializer,
  struct,
  u8,
} from '@metaplex-foundation/umi/serializers';
import {
  ResolvedAccount,
  ResolvedAccountsWithIndices,
  getAccountMetasAndSigners,
} from '../shared';

// Accounts.
export type ReadCollectionRkyvInstructionAccounts = {
  /** The address of the new account */
  address: PublicKey | Pda;
};

// Data.
export type ReadCollectionRkyvInstructionData = { discriminator: number };

export type ReadCollectionRkyvInstructionDataArgs = {};

export function getReadCollectionRkyvInstructionDataSerializer(): Serializer<
  ReadCollectionRkyvInstructionDataArgs,
  ReadCollectionRkyvInstructionData
> {
  return mapSerializer<
    ReadCollectionRkyvInstructionDataArgs,
    any,
    ReadCollectionRkyvInstructionData
  >(
    struct<ReadCollectionRkyvInstructionData>([['discriminator', u8()]], {
      description: 'ReadCollectionRkyvInstructionData',
    }),
    (value) => ({ ...value, discriminator: 22 })
  ) as Serializer<
    ReadCollectionRkyvInstructionDataArgs,
    ReadCollectionRkyvInstructionData
  >;
}

// Instruction.
export function readCollectionRkyv(
  context: Pick<Context, 'programs'>,
  input: ReadCollectionRkyvInstructionAccounts
): TransactionBuilder {
  // Program ID.
  const programId = context.programs.getPublicKey(
    'solanaSerializationBenchmark',
    'BENCHVr3SC7dVDMtKVpwctjFNPBMrqvXn9JVACJg3KEb'
  );

  // Accounts.
  const resolvedAccounts = {
    address: {
      index: 0,
      isWritable: false as boolean,
      value: input.address ?? null,
    },
  } satisfies ResolvedAccountsWithIndices;

  // Accounts in order.
  const orderedAccounts: ResolvedAccount[] = Object.values(
    resolvedAccounts
  ).sort((a, b) => a.index - b.index);

  // Keys and Signers.
  const [keys, signers] = getAccountMetasAndSigners(
    orderedAccounts,
    'programId',
    programId
  );

  // Data.
  const data = getReadCollectionRkyvInstructionDataSerializer().serialize({});

  // Bytes Created On Chain.
  const bytesCreatedOnChain = 0;

  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
  ]);
}