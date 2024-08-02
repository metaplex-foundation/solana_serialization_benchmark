use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_memory::sol_memcpy, rent::Rent,
    sysvar::Sysvar,
};

use crate::{
    error::SolanaSerializationBenchmarkError,
    instruction::accounts::{
        CreateBasicBincodeAccounts, CreateCollectionBincodeAccounts, ReadBasicBincodeAccounts,
        ReadCollectionBincodeAccounts, UpdateBasicBincodeAccounts, UpdateCollectionBincodeAccounts,
    },
    state::bincode_state::{BasicTypes, CollectionTypes},
};

pub(crate) fn create_basic<'a>(accounts: &'a [AccountInfo<'a>]) -> ProgramResult {
    // Accounts.
    let ctx = CreateBasicBincodeAccounts::context(accounts)?;

    let account = BasicTypes::default();
    // Serialize the the types to the account data.
    let account_data = bincode::serialize(&account)
        .map_err(|_| SolanaSerializationBenchmarkError::SerializationError)?;

    // CPI to the System Program.
    solana_program::program::invoke(
        &solana_program::system_instruction::create_account(
            ctx.accounts.payer.key,
            ctx.accounts.address.key,
            Rent::get()?.minimum_balance(account_data.len()),
            account_data.len() as u64,
            &crate::id(),
        ),
        &[
            ctx.accounts.payer.clone(),
            ctx.accounts.address.clone(),
            ctx.accounts.system_program.clone(),
        ],
    )?;

    sol_memcpy(
        &mut ctx.accounts.address.try_borrow_mut_data()?,
        &account_data,
        account_data.len(),
    );

    Ok(())
}

pub(crate) fn read_basic<'a>(accounts: &'a [AccountInfo<'a>]) -> ProgramResult {
    // Accounts.
    let ctx = ReadBasicBincodeAccounts::context(accounts)?;

    // Deserialize the account data to the types.
    let _account = bincode::deserialize::<BasicTypes>(&ctx.accounts.address.try_borrow_data()?)
        .map_err(|_| SolanaSerializationBenchmarkError::DeserializationError)?;

    Ok(())
}

pub(crate) fn update_basic<'a>(accounts: &'a [AccountInfo<'a>]) -> ProgramResult {
    // Accounts.
    let ctx = UpdateBasicBincodeAccounts::context(accounts)?;

    // Deserialize the account data to the types.
    let mut account = bincode::deserialize::<BasicTypes>(&ctx.accounts.address.try_borrow_data()?)
        .map_err(|_| SolanaSerializationBenchmarkError::DeserializationError)?;
    account.mutate();
    // Serialize the types back to the account data.
    let account_data = bincode::serialize(&account).map_err(|error| {
        solana_program::msg!("Error serializing account data: {:?}", error);
        SolanaSerializationBenchmarkError::SerializationError
    })?;

    // Resize the account to fit any changes in data size.
    let new_size = account_data.len();
    let rent = Rent::get()?;
    let new_minimum_balance = rent.minimum_balance(new_size);

    let lamports_diff = new_minimum_balance.saturating_sub(ctx.accounts.address.lamports());
    solana_program::program::invoke(
        &solana_program::system_instruction::transfer(
            ctx.accounts.payer.key,
            ctx.accounts.address.key,
            lamports_diff,
        ),
        &[ctx.accounts.payer.clone(), ctx.accounts.address.clone()],
    )?;
    ctx.accounts.address.realloc(account_data.len(), false)?;

    sol_memcpy(
        &mut ctx.accounts.address.try_borrow_mut_data()?,
        &account_data,
        account_data.len(),
    );

    Ok(())
}

pub(crate) fn create_collection<'a>(accounts: &'a [AccountInfo<'a>]) -> ProgramResult {
    // Accounts.
    let ctx = CreateCollectionBincodeAccounts::context(accounts)?;

    let account = CollectionTypes::default();
    // Serialize the the types to the account data.
    let account_data = bincode::serialize(&account).map_err(|error| {
        solana_program::msg!("Error serializing account data: {:?}", error);
        SolanaSerializationBenchmarkError::SerializationError
    })?;

    // CPI to the System Program.
    solana_program::program::invoke(
        &solana_program::system_instruction::create_account(
            ctx.accounts.payer.key,
            ctx.accounts.address.key,
            Rent::get()?.minimum_balance(account_data.len()),
            account_data.len() as u64,
            &crate::id(),
        ),
        &[
            ctx.accounts.payer.clone(),
            ctx.accounts.address.clone(),
            ctx.accounts.system_program.clone(),
        ],
    )?;

    sol_memcpy(
        &mut ctx.accounts.address.try_borrow_mut_data()?,
        &account_data,
        account_data.len(),
    );

    Ok(())
}

pub(crate) fn read_collection<'a>(accounts: &'a [AccountInfo<'a>]) -> ProgramResult {
    // Accounts.
    let ctx = ReadCollectionBincodeAccounts::context(accounts)?;

    // Deserialize the account data to the types.
    let _account =
        bincode::deserialize::<CollectionTypes>(&ctx.accounts.address.try_borrow_data()?)
            .map_err(|_| SolanaSerializationBenchmarkError::DeserializationError)?;

    Ok(())
}

pub(crate) fn update_collection<'a>(accounts: &'a [AccountInfo<'a>]) -> ProgramResult {
    // Accounts.
    let ctx = UpdateCollectionBincodeAccounts::context(accounts)?;

    // Deserialize the account data to the types.
    let mut account =
        bincode::deserialize::<CollectionTypes>(&ctx.accounts.address.try_borrow_data()?)
            .map_err(|_| SolanaSerializationBenchmarkError::DeserializationError)?;
    account.mutate();
    // Serialize the types back to the account data.
    let account_data = bincode::serialize(&account)
        .map_err(|_| SolanaSerializationBenchmarkError::SerializationError)?;

    // Resize the account to fit any changes in data size.
    let new_size = account_data.len();
    let rent = Rent::get()?;
    let new_minimum_balance = rent.minimum_balance(new_size);

    let lamports_diff = new_minimum_balance.saturating_sub(ctx.accounts.address.lamports());
    solana_program::program::invoke(
        &solana_program::system_instruction::transfer(
            ctx.accounts.payer.key,
            ctx.accounts.address.key,
            lamports_diff,
        ),
        &[ctx.accounts.payer.clone(), ctx.accounts.address.clone()],
    )?;
    ctx.accounts.address.realloc(account_data.len(), false)?;

    sol_memcpy(
        &mut ctx.accounts.address.try_borrow_mut_data()?,
        &account_data,
        account_data.len(),
    );

    Ok(())
}
