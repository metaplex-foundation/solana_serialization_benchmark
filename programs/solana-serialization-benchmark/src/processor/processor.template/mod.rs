/// For new Serialization libraries, copy and paste the processor.template directory. Rename the directory
/// to the name of the serialization library + _processor. Replace the "SERDESLIB" with the name of the
/// serialization library. Change the serialization and deserialization code as needed.
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_memory::sol_memcpy, rent::Rent,
    sysvar::Sysvar,
};

use crate::{
    instruction::accounts::{
        CreateBasicSERDESLIBAccounts, CreateCollectionSERDESLIBAccounts,
        ReadBasicSERDESLIBAccounts, ReadCollectionSERDESLIBAccounts, UpdateBasicSERDESLIBAccounts,
        UpdateCollectionSERDESLIBAccounts,
    },
    state::SERDESLIB_state::{BasicTypes, CollectionTypes},
};

pub(crate) fn create_basic<'a>(accounts: &'a [AccountInfo<'a>]) -> ProgramResult {
    // Accounts.
    let ctx = CreateBasicSERDESLIBAccounts::context(accounts)?;

    let account = BasicTypes::default();
    // Serialize the the types to the account data.
    // let account_data = account.try_to_vec()?;

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
    let ctx = ReadBasicSERDESLIBAccounts::context(accounts)?;

    // Deserialize the account data to the types.
    // let _account = BasicTypes::try_from_slice(&ctx.accounts.address.try_borrow_data()?)?;

    Ok(())
}

pub(crate) fn update_basic<'a>(accounts: &'a [AccountInfo<'a>]) -> ProgramResult {
    // Accounts.
    let ctx = UpdateBasicSERDESLIBAccounts::context(accounts)?;

    // Deserialize the account data to the types.
    // let mut account = BasicTypes::try_from_slice(&ctx.accounts.address.try_borrow_data()?)?;
    account.mutate();
    // Serialize the types back to the account data.
    // let account_data = account.try_to_vec()?;

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
    let ctx = CreateCollectionSERDESLIBAccounts::context(accounts)?;

    let account = CollectionTypes::default();
    // Serialize the the types to the account data.
    // let account_data = account.try_to_vec()?;

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
    let ctx = ReadCollectionSERDESLIBAccounts::context(accounts)?;

    // Deserialize the account data to the types.
    // let _account = CollectionTypes::try_from_slice(&ctx.accounts.address.try_borrow_data()?)?;

    Ok(())
}

pub(crate) fn update_collection<'a>(accounts: &'a [AccountInfo<'a>]) -> ProgramResult {
    // Accounts.
    let ctx = UpdateCollectionSERDESLIBAccounts::context(accounts)?;

    // Deserialize the account data to the types.
    // let mut account = CollectionTypes::try_from_slice(&ctx.accounts.address.try_borrow_data()?)?;
    account.mutate();
    // Serialize the types back to the account data.
    // let account_data = account.try_to_vec()?;

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
