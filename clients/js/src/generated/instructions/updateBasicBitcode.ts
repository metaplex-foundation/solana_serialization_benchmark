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
  Signer,
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
export type UpdateBasicBitcodeInstructionAccounts = {
  /** The address of the new account */
  address: PublicKey | Pda;
  /** The account paying for the storage fees */
  payer?: Signer;
  /** The system program */
  systemProgram?: PublicKey | Pda;
};

// Data.
export type UpdateBasicBitcodeInstructionData = { discriminator: number };

export type UpdateBasicBitcodeInstructionDataArgs = {};

export function getUpdateBasicBitcodeInstructionDataSerializer(): Serializer<
  UpdateBasicBitcodeInstructionDataArgs,
  UpdateBasicBitcodeInstructionData
> {
  return mapSerializer<
    UpdateBasicBitcodeInstructionDataArgs,
    any,
    UpdateBasicBitcodeInstructionData
  >(
    struct<UpdateBasicBitcodeInstructionData>([['discriminator', u8()]], {
      description: 'UpdateBasicBitcodeInstructionData',
    }),
    (value) => ({ ...value, discriminator: 26 })
  ) as Serializer<
    UpdateBasicBitcodeInstructionDataArgs,
    UpdateBasicBitcodeInstructionData
  >;
}

// Instruction.
export function updateBasicBitcode(
  context: Pick<Context, 'payer' | 'programs'>,
  input: UpdateBasicBitcodeInstructionAccounts
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
      isWritable: true as boolean,
      value: input.address ?? null,
    },
    payer: {
      index: 1,
      isWritable: true as boolean,
      value: input.payer ?? null,
    },
    systemProgram: {
      index: 2,
      isWritable: false as boolean,
      value: input.systemProgram ?? null,
    },
  } satisfies ResolvedAccountsWithIndices;

  // Default values.
  if (!resolvedAccounts.payer.value) {
    resolvedAccounts.payer.value = context.payer;
  }
  if (!resolvedAccounts.systemProgram.value) {
    resolvedAccounts.systemProgram.value = context.programs.getPublicKey(
      'splSystem',
      '11111111111111111111111111111111'
    );
    resolvedAccounts.systemProgram.isWritable = false;
  }

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
  const data = getUpdateBasicBitcodeInstructionDataSerializer().serialize({});

  // Bytes Created On Chain.
  const bytesCreatedOnChain = 0;

  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
  ]);
}
