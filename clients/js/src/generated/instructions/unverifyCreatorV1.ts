/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import {
  AccountMeta,
  Context,
  PublicKey,
  Serializer,
  Signer,
  WrappedInstruction,
  checkForIsWritableOverride as isWritable,
  mapSerializer,
  publicKey,
} from '@metaplex-foundation/umi';

// Accounts.
export type UnverifyCreatorV1InstructionAccounts = {
  /** Creator to verify, collection (or metadata if parent burned) update authority or delegate */
  authority?: Signer;
  /** Delegate record PDA */
  delegateRecord?: PublicKey;
  /** Metadata account */
  metadata: PublicKey;
  /** Mint of the Collection */
  collectionMint?: PublicKey;
  /** Metadata Account of the Collection */
  collectionMetadata?: PublicKey;
  /** System program */
  systemProgram?: PublicKey;
  /** Instructions sysvar account */
  sysvarInstructions?: PublicKey;
};

// Arguments.
export type UnverifyCreatorV1InstructionData = {
  discriminator: number;
  unverifyCreatorV1Discriminator: number;
};

export type UnverifyCreatorV1InstructionDataArgs = {};

export function getUnverifyCreatorV1InstructionDataSerializer(
  context: Pick<Context, 'serializer'>
): Serializer<
  UnverifyCreatorV1InstructionDataArgs,
  UnverifyCreatorV1InstructionData
> {
  const s = context.serializer;
  return mapSerializer<
    UnverifyCreatorV1InstructionDataArgs,
    UnverifyCreatorV1InstructionData,
    UnverifyCreatorV1InstructionData
  >(
    s.struct<UnverifyCreatorV1InstructionData>(
      [
        ['discriminator', s.u8()],
        ['unverifyCreatorV1Discriminator', s.u8()],
      ],
      { description: 'UnverifyCreatorV1InstructionData' }
    ),
    (value) =>
      ({
        ...value,
        discriminator: 53,
        unverifyCreatorV1Discriminator: 0,
      } as UnverifyCreatorV1InstructionData)
  ) as Serializer<
    UnverifyCreatorV1InstructionDataArgs,
    UnverifyCreatorV1InstructionData
  >;
}

// Instruction.
export function unverifyCreatorV1(
  context: Pick<Context, 'serializer' | 'programs' | 'identity'>,
  input: UnverifyCreatorV1InstructionAccounts
): WrappedInstruction {
  const signers: Signer[] = [];
  const keys: AccountMeta[] = [];

  // Program ID.
  const programId = context.programs.getPublicKey(
    'mplTokenMetadata',
    'metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s'
  );

  // Resolved accounts.
  const authorityAccount = input.authority ?? context.identity;
  const delegateRecordAccount = input.delegateRecord ?? {
    ...programId,
    isWritable: false,
  };
  const metadataAccount = input.metadata;
  const collectionMintAccount = input.collectionMint ?? {
    ...programId,
    isWritable: false,
  };
  const collectionMetadataAccount = input.collectionMetadata ?? {
    ...programId,
    isWritable: false,
  };
  const systemProgramAccount = input.systemProgram ?? {
    ...context.programs.getPublicKey(
      'splSystem',
      '11111111111111111111111111111111'
    ),
    isWritable: false,
  };
  const sysvarInstructionsAccount =
    input.sysvarInstructions ??
    publicKey('Sysvar1nstructions1111111111111111111111111');

  // Authority.
  signers.push(authorityAccount);
  keys.push({
    pubkey: authorityAccount.publicKey,
    isSigner: true,
    isWritable: isWritable(authorityAccount, false),
  });

  // Delegate Record.
  keys.push({
    pubkey: delegateRecordAccount,
    isSigner: false,
    isWritable: isWritable(delegateRecordAccount, false),
  });

  // Metadata.
  keys.push({
    pubkey: metadataAccount,
    isSigner: false,
    isWritable: isWritable(metadataAccount, true),
  });

  // Collection Mint.
  keys.push({
    pubkey: collectionMintAccount,
    isSigner: false,
    isWritable: isWritable(collectionMintAccount, false),
  });

  // Collection Metadata.
  keys.push({
    pubkey: collectionMetadataAccount,
    isSigner: false,
    isWritable: isWritable(collectionMetadataAccount, true),
  });

  // System Program.
  keys.push({
    pubkey: systemProgramAccount,
    isSigner: false,
    isWritable: isWritable(systemProgramAccount, false),
  });

  // Sysvar Instructions.
  keys.push({
    pubkey: sysvarInstructionsAccount,
    isSigner: false,
    isWritable: isWritable(sysvarInstructionsAccount, false),
  });

  // Data.
  const data = getUnverifyCreatorV1InstructionDataSerializer(context).serialize(
    {}
  );

  // Bytes Created On Chain.
  const bytesCreatedOnChain = 0;

  return {
    instruction: { keys, programId, data },
    signers,
    bytesCreatedOnChain,
  };
}
