pub mod bincode_processor;
pub mod borsh_processor;
pub mod none_processor;
pub mod rkyv_processor;
pub mod fb_processor;

use borsh::BorshDeserialize;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

use crate::instruction::SolanaSerializationBenchmarkInstruction;

pub fn process_instruction<'a>(
    _program_id: &Pubkey,
    accounts: &'a [AccountInfo<'a>],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction: SolanaSerializationBenchmarkInstruction =
        SolanaSerializationBenchmarkInstruction::try_from_slice(instruction_data)?;
    match instruction {
        SolanaSerializationBenchmarkInstruction::CreateBasicNone => {
            none_processor::create_basic(accounts)
        }
        SolanaSerializationBenchmarkInstruction::ReadBasicNone => {
            none_processor::read_basic(accounts)
        }
        SolanaSerializationBenchmarkInstruction::UpdateBasicNone => {
            none_processor::update_basic(accounts)
        }
        SolanaSerializationBenchmarkInstruction::CreateCollectionNone => {
            none_processor::create_collection(accounts)
        }
        SolanaSerializationBenchmarkInstruction::ReadCollectionNone => {
            none_processor::read_collection(accounts)
        }
        SolanaSerializationBenchmarkInstruction::UpdateCollectionNone => {
            none_processor::update_collection(accounts)
        }
        SolanaSerializationBenchmarkInstruction::CreateBasicBorsh => {
            borsh_processor::create_basic(accounts)
        }
        SolanaSerializationBenchmarkInstruction::ReadBasicBorsh => {
            borsh_processor::read_basic(accounts)
        }
        SolanaSerializationBenchmarkInstruction::UpdateBasicBorsh => {
            borsh_processor::update_basic(accounts)
        }
        SolanaSerializationBenchmarkInstruction::CreateCollectionBorsh => {
            borsh_processor::create_collection(accounts)
        }
        SolanaSerializationBenchmarkInstruction::ReadCollectionBorsh => {
            borsh_processor::read_collection(accounts)
        }
        SolanaSerializationBenchmarkInstruction::UpdateCollectionBorsh => {
            borsh_processor::update_collection(accounts)
        }
        SolanaSerializationBenchmarkInstruction::CreateBasicBincode => {
            bincode_processor::create_basic(accounts)
        }
        SolanaSerializationBenchmarkInstruction::ReadBasicBincode => {
            bincode_processor::read_basic(accounts)
        }
        SolanaSerializationBenchmarkInstruction::UpdateBasicBincode => {
            bincode_processor::update_basic(accounts)
        }
        SolanaSerializationBenchmarkInstruction::CreateCollectionBincode => {
            bincode_processor::create_collection(accounts)
        }
        SolanaSerializationBenchmarkInstruction::ReadCollectionBincode => {
            bincode_processor::read_collection(accounts)
        }
        SolanaSerializationBenchmarkInstruction::UpdateCollectionBincode => {
            bincode_processor::update_collection(accounts)
        }
        SolanaSerializationBenchmarkInstruction::CreateBasicRkyv => {
            rkyv_processor::create_basic(accounts)
        }
        SolanaSerializationBenchmarkInstruction::ReadBasicRkyv => {
            rkyv_processor::read_basic(accounts)
        }
        SolanaSerializationBenchmarkInstruction::UpdateBasicRkyv => {
            rkyv_processor::update_basic(accounts)
        }
        SolanaSerializationBenchmarkInstruction::CreateCollectionRkyv => {
            rkyv_processor::create_collection(accounts)
        }
        SolanaSerializationBenchmarkInstruction::ReadCollectionRkyv => {
            rkyv_processor::read_collection(accounts)
        }
        SolanaSerializationBenchmarkInstruction::UpdateCollectionRkyv => {
            rkyv_processor::update_collection(accounts)
        }
        SolanaSerializationBenchmarkInstruction::CreateBasicFB => {
            fb_processor::create_basic(accounts)
        }
        SolanaSerializationBenchmarkInstruction::ReadBasicFB => {
            fb_processor::read_basic(accounts)
        }
        SolanaSerializationBenchmarkInstruction::UpdateBasicFB => {
            fb_processor::update_basic(accounts)
        }
        SolanaSerializationBenchmarkInstruction::CreateCollectionFB => {
            fb_processor::create_collection(accounts)
        }
        SolanaSerializationBenchmarkInstruction::ReadCollectionFB => {
            fb_processor::read_collection(accounts)
        }
        SolanaSerializationBenchmarkInstruction::UpdateCollectionFB => {
            fb_processor::update_collection(accounts)
        }
    }
}
