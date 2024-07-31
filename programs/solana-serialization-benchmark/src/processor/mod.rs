pub mod borsh_processor;
pub mod none_processor;

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
    }
}
