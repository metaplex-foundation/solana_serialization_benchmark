use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_memory::sol_memcpy, rent::Rent,
    sysvar::Sysvar,
};

use crate::{
    instruction::accounts::{
        CreateBasicBorshAccounts, CreateCollectionBorshAccounts, ReadBasicBorshAccounts,
        ReadCollectionBorshAccounts, UpdateBasicBorshAccounts, UpdateCollectionBorshAccounts,
    },
    state::borsh_state::{BasicTypes, CollectionTypes},
};

pub(crate) fn create_basic<'a>(accounts: &'a [AccountInfo<'a>]) -> ProgramResult {
    // Accounts.
    let ctx = CreateBasicBorshAccounts::context(accounts)?;

    let account = BasicTypes::default();
    let account_data = account.try_to_vec()?;

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
    let ctx = ReadBasicBorshAccounts::context(accounts)?;

    let _account = BasicTypes::try_from_slice(&ctx.accounts.address.try_borrow_data()?)?;

    Ok(())
}

pub(crate) fn update_basic<'a>(accounts: &'a [AccountInfo<'a>]) -> ProgramResult {
    // Accounts.
    let ctx = UpdateBasicBorshAccounts::context(accounts)?;
    solana_program::msg!("Updating basic account");

    let mut account = BasicTypes::try_from_slice(&ctx.accounts.address.try_borrow_data()?)?;
    solana_program::msg!("Mutating account");
    account.mutate();
    solana_program::msg!("Serializing account");
    let account_data = account.try_to_vec()?;
    solana_program::msg!("Serialized account");

    // Resize the account to fit any changes in data size.
    let new_size = account_data.len();
    solana_program::msg!("new_size: {}", new_size);
    let rent = Rent::get()?;
    let new_minimum_balance = rent.minimum_balance(new_size);
    solana_program::msg!("new_minimum_balance: {}", new_minimum_balance);

    let lamports_diff = new_minimum_balance.saturating_sub(ctx.accounts.address.lamports());
    solana_program::msg!("lamports_diff: {}", lamports_diff);
    solana_program::program::invoke(
        &solana_program::system_instruction::transfer(
            ctx.accounts.payer.key,
            ctx.accounts.address.key,
            lamports_diff,
        ),
        &[ctx.accounts.payer.clone(), ctx.accounts.address.clone()],
    )?;
    solana_program::msg!("Transferred lamports");
    ctx.accounts.address.realloc(account_data.len(), false)?;
    solana_program::msg!("Reallocated account");

    sol_memcpy(
        &mut ctx.accounts.address.try_borrow_mut_data()?,
        &account_data,
        account_data.len(),
    );

    Ok(())
}

pub(crate) fn create_collection<'a>(accounts: &'a [AccountInfo<'a>]) -> ProgramResult {
    // Accounts.
    let ctx = CreateCollectionBorshAccounts::context(accounts)?;

    let account = CollectionTypes::default();
    let account_data = account.try_to_vec()?;

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
    let ctx = ReadCollectionBorshAccounts::context(accounts)?;

    let _account = CollectionTypes::try_from_slice(&ctx.accounts.address.try_borrow_data()?)?;

    Ok(())
}

pub(crate) fn update_collection<'a>(accounts: &'a [AccountInfo<'a>]) -> ProgramResult {
    // Accounts.
    let ctx = UpdateCollectionBorshAccounts::context(accounts)?;

    let mut account = CollectionTypes::try_from_slice(&ctx.accounts.address.try_borrow_data()?)?;
    account.mutate();
    let account_data = account.try_to_vec()?;

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
