use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_memory::sol_memcpy, rent::Rent,
    sysvar::Sysvar,
};

use crate::{
    instruction::accounts::{
        CreateBasicBorshAccounts, CreateCollectionBorshAccounts, ReadBasicNoneAccounts,
        ReadCollectionNoneAccounts, UpdateBasicBorshAccounts, UpdateCollectionBorshAccounts,
    },
    state::none_state::{BasicTypes, CollectionTypes},
};

pub(crate) fn create_basic<'a>(accounts: &'a [AccountInfo<'a>]) -> ProgramResult {
    // Accounts.
    let ctx = CreateBasicBorshAccounts::context(accounts)?;

    let _account = BasicTypes::default();

    // CPI to the System Program.
    solana_program::program::invoke(
        &solana_program::system_instruction::create_account(
            ctx.accounts.payer.key,
            ctx.accounts.address.key,
            Rent::get()?.minimum_balance(0),
            0,
            &crate::id(),
        ),
        &[
            ctx.accounts.payer.clone(),
            ctx.accounts.address.clone(),
            ctx.accounts.system_program.clone(),
        ],
    )?;

    sol_memcpy(&mut ctx.accounts.address.try_borrow_mut_data()?, &[], 0);

    Ok(())
}

pub(crate) fn read_basic<'a>(accounts: &'a [AccountInfo<'a>]) -> ProgramResult {
    // Accounts.
    let _ctx = ReadBasicNoneAccounts::context(accounts)?;

    Ok(())
}

pub(crate) fn update_basic<'a>(accounts: &'a [AccountInfo<'a>]) -> ProgramResult {
    // Accounts.
    let ctx = UpdateBasicBorshAccounts::context(accounts)?;

    // Resize the account to fit any changes in data size.
    let new_size = 0;
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
    ctx.accounts.address.realloc(0, false)?;

    sol_memcpy(&mut ctx.accounts.address.try_borrow_mut_data()?, &[], 0);

    Ok(())
}

pub(crate) fn create_collection<'a>(accounts: &'a [AccountInfo<'a>]) -> ProgramResult {
    // Accounts.
    let ctx = CreateCollectionBorshAccounts::context(accounts)?;

    let _account = CollectionTypes::default();

    // CPI to the System Program.
    solana_program::program::invoke(
        &solana_program::system_instruction::create_account(
            ctx.accounts.payer.key,
            ctx.accounts.address.key,
            Rent::get()?.minimum_balance(0),
            0,
            &crate::id(),
        ),
        &[
            ctx.accounts.payer.clone(),
            ctx.accounts.address.clone(),
            ctx.accounts.system_program.clone(),
        ],
    )?;

    sol_memcpy(&mut ctx.accounts.address.try_borrow_mut_data()?, &[], 0);

    Ok(())
}

pub(crate) fn read_collection<'a>(accounts: &'a [AccountInfo<'a>]) -> ProgramResult {
    // Accounts.
    let _ctx = ReadCollectionNoneAccounts::context(accounts)?;

    Ok(())
}

pub(crate) fn update_collection<'a>(accounts: &'a [AccountInfo<'a>]) -> ProgramResult {
    // Accounts.
    let ctx = UpdateCollectionBorshAccounts::context(accounts)?;

    // Resize the account to fit any changes in data size.
    let new_size = 0;
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
    ctx.accounts.address.realloc(0, false)?;

    sol_memcpy(&mut ctx.accounts.address.try_borrow_mut_data()?, &[], 0);

    Ok(())
}
