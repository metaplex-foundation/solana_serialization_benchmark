use borsh::{BorshDeserialize, BorshSerialize};
use shank::{ShankContext, ShankInstruction};

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, ShankContext, ShankInstruction)]
#[rustfmt::skip]
pub enum SolanaSerializationBenchmarkInstruction {
    /// Create an empty account with basic types as a baseline.
    #[account(0, writable, signer, name="address", desc = "The address of the new account")]
    #[account(1, writable, signer, name="payer", desc = "The account paying for the storage fees")]
    #[account(2, name="system_program", desc = "The system program")]
    CreateBasicNone,

    /// Read an empty account with basic types as a baseline.
    #[account(0, name="address", desc = "The address of the new account")]
    ReadBasicNone,

    /// Update an empty account with basic types as a baseline.
    #[account(0, writable, name="address", desc = "The address of the new account")]
    #[account(1, writable, signer, name="payer", desc = "The account paying for the storage fees")]
    #[account(2, name="system_program", desc = "The system program")]
    UpdateBasicNone,

    /// Create an empty account with collection types as a baseline.
    #[account(0, writable, signer, name="address", desc = "The address of the new account")]
    #[account(1, writable, signer, name="payer", desc = "The account paying for the storage fees")]
    #[account(2, name="system_program", desc = "The system program")]
    CreateCollectionNone,

    /// Read an empty account with collection types as a baseline.
    #[account(0, name="address", desc = "The address of the new account")]
    ReadCollectionNone,

    /// Update an empty account with collection types as a baseline.
    #[account(0, writable, name="address", desc = "The address of the new account")]
    #[account(1, writable, signer, name="payer", desc = "The account paying for the storage fees")]
    #[account(2, name="system_program", desc = "The system program")]
    UpdateCollectionNone,

    /// Create a Borsh serialized data account with basic types.
    #[account(0, writable, signer, name="address", desc = "The address of the new account")]
    #[account(1, writable, signer, name="payer", desc = "The account paying for the storage fees")]
    #[account(2, name="system_program", desc = "The system program")]
    CreateBasicBorsh,

    /// Read a Borsh serialized data account with basic types.
    #[account(0, name="address", desc = "The address of the new account")]
    ReadBasicBorsh,

    /// Update a Borsh serialized data account with basic types.
    #[account(0, writable, name="address", desc = "The address of the new account")]
    #[account(1, writable, signer, name="payer", desc = "The account paying for the storage fees")]
    #[account(2, name="system_program", desc = "The system program")]
    UpdateBasicBorsh,

    /// Create a Borsh serialized data account with collection types.
    #[account(0, writable, signer, name="address", desc = "The address of the new account")]
    #[account(1, writable, signer, name="payer", desc = "The account paying for the storage fees")]
    #[account(2, name="system_program", desc = "The system program")]
    CreateCollectionBorsh,

    /// Read a Borsh serialized data account with collection types.
    #[account(0, name="address", desc = "The address of the new account")]
    ReadCollectionBorsh,

    /// Update a Borsh serialized data account with collection types.
    #[account(0, writable, name="address", desc = "The address of the new account")]
    #[account(1, writable, signer, name="payer", desc = "The account paying for the storage fees")]
    #[account(2, name="system_program", desc = "The system program")]
    UpdateCollectionBorsh,

    /// Create an Bincode account with basic types as a baseline.
    #[account(0, writable, signer, name="address", desc = "The address of the new account")]
    #[account(1, writable, signer, name="payer", desc = "The account paying for the storage fees")]
    #[account(2, name="system_program", desc = "The system program")]
    CreateBasicBincode,

    /// Read an Bincode account with basic types as a baseline.
    #[account(0, name="address", desc = "The address of the new account")]
    ReadBasicBincode,

    /// Update an Bincode account with basic types as a baseline.
    #[account(0, writable, name="address", desc = "The address of the new account")]
    #[account(1, writable, signer, name="payer", desc = "The account paying for the storage fees")]
    #[account(2, name="system_program", desc = "The system program")]
    UpdateBasicBincode,

    /// Create an Bincode account with collection types as a baseline.
    #[account(0, writable, signer, name="address", desc = "The address of the new account")]
    #[account(1, writable, signer, name="payer", desc = "The account paying for the storage fees")]
    #[account(2, name="system_program", desc = "The system program")]
    CreateCollectionBincode,

    /// Read an Bincode account with collection types as a baseline.
    #[account(0, name="address", desc = "The address of the new account")]
    ReadCollectionBincode,

    /// Update an Bincode account with collection types as a baseline.
    #[account(0, writable, name="address", desc = "The address of the new account")]
    #[account(1, writable, signer, name="payer", desc = "The account paying for the storage fees")]
    #[account(2, name="system_program", desc = "The system program")]
    UpdateCollectionBincode,

    /// Create an rkyv account with basic types as a baseline.
    #[account(0, writable, signer, name="address", desc = "The address of the new account")]
    #[account(1, writable, signer, name="payer", desc = "The account paying for the storage fees")]
    #[account(2, name="system_program", desc = "The system program")]
    CreateBasicRkyv,

    /// Read an rkyv account with basic types as a baseline.
    #[account(0, name="address", desc = "The address of the new account")]
    ReadBasicRkyv,

    /// Update an rkyv account with basic types as a baseline.
    #[account(0, writable, name="address", desc = "The address of the new account")]
    #[account(1, writable, signer, name="payer", desc = "The account paying for the storage fees")]
    #[account(2, name="system_program", desc = "The system program")]
    UpdateBasicRkyv,

    /// Create an rkyv account with collection types as a baseline.
    #[account(0, writable, signer, name="address", desc = "The address of the new account")]
    #[account(1, writable, signer, name="payer", desc = "The account paying for the storage fees")]
    #[account(2, name="system_program", desc = "The system program")]
    CreateCollectionRkyv,

    /// Read an rkyv account with collection types as a baseline.
    #[account(0, name="address", desc = "The address of the new account")]
    ReadCollectionRkyv,

    /// Update an rkyv account with collection types as a baseline.
    #[account(0, writable, name="address", desc = "The address of the new account")]
    #[account(1, writable, signer, name="payer", desc = "The account paying for the storage fees")]
    #[account(2, name="system_program", desc = "The system program")]
    UpdateCollectionRkyv,

    // For new Serialization libaries, copy and paste the CreateBasicNone, ReadBasicNone, UpdateBasicNone,
    // CreateCollectionNone, ReadCollectionNone, UpdateCollectionNone instructions here. Replace the "None"
    // with the name of the serialization library.
}
