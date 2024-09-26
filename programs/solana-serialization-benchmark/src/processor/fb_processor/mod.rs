
/// For new Serialization libraries, copy and paste the processor.template directory. Rename the directory
/// to the name of the serialization library + _processor. Replace the "FB" with the name of the
/// serialization library. Change the serialization and deserialization code as needed.
use solana_program::{
  account_info::AccountInfo, entrypoint::ProgramResult, program_memory::sol_memcpy, rent::Rent, sysvar::Sysvar
};

use crate::{error::SolanaSerializationBenchmarkError, instruction::accounts::{
      CreateBasicFBAccounts, CreateCollectionFBAccounts, ReadBasicFBAccounts, ReadCollectionFBAccounts, UpdateBasicFBAccounts, UpdateCollectionFBAccounts
  }};
use fb_types::{
  root_as_basic_types, root_as_collection_types, BasicTypesT, CollectionTypesT, ExampleEnum, ExampleStructT, ExampleVariantT, FlatBufferBuilder, OneST, PublicKeyHolderT, PublicKeyT, ThreeST, TwoST, ZeroST
};

pub(crate) fn create_basic<'a>(accounts: &'a [AccountInfo<'a>]) -> ProgramResult {
  // Accounts.
  let ctx = CreateBasicFBAccounts::context(accounts)?;
  let account = BasicTypesT{
    unsigned8: 1,
    unsigned16: 2,
    unsigned32: 3,
    unsigned64: 4,
    public_key: Some(PublicKeyT::default()),
    string: Some("Hello Solana".to_string()),
    example_struct: Some(Box::new(ExampleStructT{
      unsigned8: 1,
      unsigned16: 2,
      unsigned32: 3,
      unsigned64: 4,
      public_key: Some(PublicKeyT::default()),
      ..Default::default()
    })),
    example_enum: ExampleEnum::Zero,
    example_variant: ExampleVariantT::Zero(Box::new(ZeroST{
      zero: 0,
      ..Default::default()
    })),
    array8: Some(vec![1; 10]),
    array16: Some(vec![2; 10]),
    array32: Some(vec![3; 10]),
    array64: Some(vec![4; 10]),
    array_public_key: Some(vec![PublicKeyT::default(); 10]),
    array_string: Some(vec!["Hello Solana".to_string(); 10]),
    array_example_struct: Some(vec![ExampleStructT{
      unsigned8: 1,
      unsigned16: 2,
      unsigned32: 3,
      unsigned64: 4,
      public_key: Some(PublicKeyT::default()),
      ..Default::default()
    }; 10]),
    array_example_enum: Some(vec![ExampleEnum::Zero; 10]),
    array_example_variant1: ExampleVariantT::Zero(Box::new(ZeroST{
      zero: 0,
      ..Default::default()
    })),
    array_example_variant2:  ExampleVariantT::One(Box::new(OneST{
       one: 1,
       ..Default::default()
    })),
    array_example_variant3:  ExampleVariantT::Two(Box::new(TwoST{
      two: 2,
      ..Default::default()
    })),
    array_example_variant4: ExampleVariantT::Three(Box::new(ThreeST{
      three: 3,
      ..Default::default()
    })),
    array_example_variant5: ExampleVariantT::Four(Box::new(PublicKeyHolderT{
      key: Some(PublicKeyT::default()),
    })),
    ..Default::default()
  };
  // Serialize the the types to the account data.
  let mut fbb = FlatBufferBuilder::new();
  let packed = account.pack(&mut fbb);
  fbb.finish(packed, None);
  let account_data = fbb.finished_data();

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
  let ctx = ReadBasicFBAccounts::context(accounts)?;

  // Deserialize the account data to the types.
  let _account = root_as_basic_types(&ctx.accounts.address.try_borrow_data()?)
    .map_err(|_| SolanaSerializationBenchmarkError::DeserializationError)?;

  Ok(())
}

pub(crate) fn update_basic<'a>(accounts: &'a [AccountInfo<'a>]) -> ProgramResult {
  // Accounts.
  let ctx = UpdateBasicFBAccounts::context(accounts)?;

  // Deserialize the account data to the types.
  let data = ctx.accounts.address.try_borrow_data()?;
  let account = root_as_basic_types(&data)
    .map_err(|_| SolanaSerializationBenchmarkError::DeserializationError)?;
  let mut unpacked = account.unpack();
  drop(data);
  unpacked.mutate();
  // Serialize the types back to the account data.
  let mut fbb = FlatBufferBuilder::new();
  let packed = unpacked.pack(&mut fbb);
  fbb.finish(packed, None);
  let account_data = fbb.finished_data();


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
  let ctx = CreateCollectionFBAccounts::context(accounts)?;

  let account = CollectionTypesT{
    vec_public_key: Some(vec![PublicKeyT{
      b: [1; 32],
    }; 100]),
  };
  let mut fbb = FlatBufferBuilder::new();
  let packed = account.pack(&mut fbb);
  fbb.finish(packed, None);
  let account_data = fbb.finished_data();
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
  let ctx = ReadCollectionFBAccounts::context(accounts)?;

  // Deserialize the account data to the types.
  let _account = root_as_collection_types(&ctx.accounts.address.try_borrow_data()?)
    .map_err(|_| SolanaSerializationBenchmarkError::DeserializationError)?;

  Ok(())
}

pub(crate) fn update_collection<'a>(accounts: &'a [AccountInfo<'a>]) -> ProgramResult {
  // Accounts.
  let ctx = UpdateCollectionFBAccounts::context(accounts)?;

  // Deserialize the account data to the types.
  let data = ctx.accounts.address.try_borrow_data()?;
  let account = root_as_collection_types(&data)
    .map_err(|_| SolanaSerializationBenchmarkError::DeserializationError)?;
  let mut unpacked = account.unpack();
  drop(data);
  unpacked.mutate();
  // Serialize the types back to the account data.
  let mut fbb = FlatBufferBuilder::new();
  let packed = unpacked.pack(&mut fbb);
  fbb.finish(packed, None);
  let account_data = fbb.finished_data();

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
