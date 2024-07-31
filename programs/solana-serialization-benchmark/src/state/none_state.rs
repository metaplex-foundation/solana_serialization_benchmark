use solana_program::pubkey::Pubkey;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

#[repr(C)]
#[derive(Clone, Debug, Copy)]
pub struct ExampleStruct {
    pub unsigned8: u8,
    pub unsigned16: u16,
    pub unsigned32: u32,
    pub unsigned64: u64,
    pub public_key: Pubkey,
}

impl Default for ExampleStruct {
    fn default() -> Self {
        Self {
            unsigned8: 1,
            unsigned16: 2,
            unsigned32: 3,
            unsigned64: 4,
            public_key: Pubkey::default(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Debug, Copy)]
pub enum ExampleEnum {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl Default for ExampleEnum {
    fn default() -> Self {
        Self::Zero
    }
}

#[repr(C)]
#[derive(Clone, Debug, Copy)]
pub enum ExampleVariant {
    Zero(u8),
    One(u16),
    Two(u32),
    Three(u64),
    Four(Pubkey),
}

impl Default for ExampleVariant {
    fn default() -> Self {
        Self::Zero(0)
    }
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct BasicTypes {
    pub unsigned8: u8,
    pub unsigned16: u16,
    pub unsigned32: u32,
    pub unsigned64: u64,
    pub public_key: Pubkey,
    pub string: String,
    pub example_struct: ExampleStruct,
    pub example_enum: ExampleEnum,
    pub example_variant: ExampleVariant,
    pub array8: [u8; 10],
    pub array16: [u16; 10],
    pub array32: [u32; 10],
    pub array64: [u64; 10],
    pub array_public_key: [Pubkey; 10],
    pub array_string: [String; 10],
    pub array_example_struct: [ExampleStruct; 10],
    pub array_example_enum: [ExampleEnum; 10],
    pub array_example_variant: [ExampleVariant; 5],
}

impl Default for BasicTypes {
    fn default() -> Self {
        Self {
            unsigned8: 1,
            unsigned16: 2,
            unsigned32: 3,
            unsigned64: 4,
            public_key: Pubkey::default(),
            string: "Hello Solana".to_string(),
            example_struct: ExampleStruct::default(),
            example_enum: ExampleEnum::default(),
            example_variant: ExampleVariant::default(),
            array8: [1; 10],
            array16: [2; 10],
            array32: [3; 10],
            array64: [4; 10],
            array_public_key: [Pubkey::default(); 10],
            array_string: vec!["Hello Solana".to_string(); 10]
                .try_into()
                .unwrap_or_else(|_| panic!("Incorrect length")),
            array_example_struct: [ExampleStruct::default(); 10],
            array_example_enum: [
                ExampleEnum::Zero,
                ExampleEnum::One,
                ExampleEnum::Two,
                ExampleEnum::Three,
                ExampleEnum::Four,
                ExampleEnum::Five,
                ExampleEnum::Six,
                ExampleEnum::Seven,
                ExampleEnum::Eight,
                ExampleEnum::Nine,
            ],
            array_example_variant: [
                ExampleVariant::Zero(0),
                ExampleVariant::One(1),
                ExampleVariant::Two(2),
                ExampleVariant::Three(3),
                ExampleVariant::Four(Pubkey::default()),
            ],
        }
    }
}

impl BasicTypes {
    pub fn mutate(&mut self) {
        self.unsigned8 += 1;
        self.unsigned16 += 2;
        self.unsigned32 += 3;
        self.unsigned64 += 4;
        self.public_key = solana_program::vote::program::ID;
        self.string.push_str(", how are you?");
        self.example_struct.unsigned8 += 1;
        self.example_struct.unsigned16 += 2;
        self.example_struct.unsigned32 += 3;
        self.example_struct.unsigned64 += 4;
        self.example_struct.public_key = solana_program::vote::program::ID;
        self.example_enum = ExampleEnum::One;
        self.example_variant = ExampleVariant::One(1);
        self.array8[0] += 1;
        self.array16[0] += 2;
        self.array32[0] += 3;
        self.array64[0] += 4;
        self.array_public_key[0] = solana_program::vote::program::ID;
        self.array_string[0].push_str(", how are you?");
        self.array_example_struct[0].unsigned8 += 1;
        self.array_example_struct[0].unsigned16 += 2;
        self.array_example_struct[0].unsigned32 += 3;
        self.array_example_struct[0].unsigned64 += 4;
        self.array_example_struct[0].public_key = solana_program::vote::program::ID;
        self.array_example_enum[0] = ExampleEnum::One;
        self.array_example_variant[0] = ExampleVariant::One(1);
    }
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct CollectionTypes {
    pub vec_public_key: Vec<Pubkey>,
    pub hset_public_key: HashSet<Pubkey>,
    pub bset_public_key: BTreeSet<Pubkey>,
    pub hmap_public_key: HashMap<Pubkey, Pubkey>,
    pub bmap_public_key: BTreeMap<Pubkey, Pubkey>,
}

impl Default for CollectionTypes {
    fn default() -> Self {
        Self {
            vec_public_key: vec![Pubkey::default(); 10],
            hset_public_key: HashSet::from([Pubkey::default(); 10]),
            bset_public_key: BTreeSet::from([Pubkey::default(); 10]),
            hmap_public_key: HashMap::from([(Pubkey::default(), Pubkey::default()); 10]),
            bmap_public_key: BTreeMap::from([(Pubkey::default(), Pubkey::default()); 10]),
        }
    }
}

impl CollectionTypes {
    pub fn mutate(&mut self) {
        self.vec_public_key.push(solana_program::vote::program::ID);
        self.hset_public_key
            .insert(solana_program::vote::program::ID);
        self.bset_public_key
            .insert(solana_program::vote::program::ID);
        self.hmap_public_key.insert(
            solana_program::vote::program::ID,
            solana_program::vote::program::ID,
        );
        self.bmap_public_key.insert(
            solana_program::vote::program::ID,
            solana_program::vote::program::ID,
        );
    }
}
