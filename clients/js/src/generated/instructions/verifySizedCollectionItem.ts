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
export type VerifySizedCollectionItemInstructionAccounts = {
  /** Metadata account */
  metadata: PublicKey | Pda;
  /** Collection Update authority */
  collectionAuthority: Signer;
  /** payer */
  payer?: Signer;
  /** Mint of the Collection */
  collectionMint: PublicKey | Pda;
  /** Metadata Account of the Collection */
  collection: PublicKey | Pda;
  /** MasterEdition2 Account of the Collection Token */
  collectionMasterEditionAccount: PublicKey | Pda;
  /** Collection Authority Record PDA */
  collectionAuthorityRecord?: PublicKey | Pda;
};

// Data.
export type VerifySizedCollectionItemInstructionData = {
  discriminator: number;
};

export type VerifySizedCollectionItemInstructionDataArgs = {};

export function getVerifySizedCollectionItemInstructionDataSerializer(): Serializer<
  VerifySizedCollectionItemInstructionDataArgs,
  VerifySizedCollectionItemInstructionData
> {
  return mapSerializer<
    VerifySizedCollectionItemInstructionDataArgs,
    any,
    VerifySizedCollectionItemInstructionData
  >(
    struct<VerifySizedCollectionItemInstructionData>(
      [['discriminator', u8()]],
      { description: 'VerifySizedCollectionItemInstructionData' }
    ),
    (value) => ({ ...value, discriminator: 30 })
  ) as Serializer<
    VerifySizedCollectionItemInstructionDataArgs,
    VerifySizedCollectionItemInstructionData
  >;
}

// Instruction.
export function verifySizedCollectionItem(
  context: Pick<Context, 'payer' | 'programs'>,
  input: VerifySizedCollectionItemInstructionAccounts
): TransactionBuilder {
  // Program ID.
  const programId = context.programs.getPublicKey(
    'mplTokenMetadata',
    'metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s'
  );

  // Accounts.
  const resolvedAccounts = {
    metadata: { index: 0, isWritable: true, value: input.metadata ?? null },
    collectionAuthority: {
      index: 1,
      isWritable: false,
      value: input.collectionAuthority ?? null,
    },
    payer: { index: 2, isWritable: true, value: input.payer ?? null },
    collectionMint: {
      index: 3,
      isWritable: false,
      value: input.collectionMint ?? null,
    },
    collection: { index: 4, isWritable: true, value: input.collection ?? null },
    collectionMasterEditionAccount: {
      index: 5,
      isWritable: false,
      value: input.collectionMasterEditionAccount ?? null,
    },
    collectionAuthorityRecord: {
      index: 6,
      isWritable: false,
      value: input.collectionAuthorityRecord ?? null,
    },
  } satisfies ResolvedAccountsWithIndices;

  // Default values.
  if (!resolvedAccounts.payer.value) {
    resolvedAccounts.payer.value = context.payer;
  }

  // Accounts in order.
  const orderedAccounts: ResolvedAccount[] = Object.values(
    resolvedAccounts
  ).sort((a, b) => a.index - b.index);

  // Keys and Signers.
  const [keys, signers] = getAccountMetasAndSigners(
    orderedAccounts,
    'omitted',
    programId
  );

  // Data.
  const data =
    getVerifySizedCollectionItemInstructionDataSerializer().serialize({});

  // Bytes Created On Chain.
  const bytesCreatedOnChain = 0;

  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
  ]);
}
