mod basic_generated;
mod collection_generated;
mod state_generated;

pub use basic_generated::*;
pub use collection_generated::*;
pub use state_generated::*;

pub use flatbuffers::FlatBufferBuilder;

impl CollectionTypesT {
  pub fn mutate(&mut self) {
    self.vec_public_key.as_mut().map(|x| x.push(PublicKeyT{
      b: [1; 32],
    }));
  }
}


impl BasicTypesT {
  pub fn mutate(&mut self) {
    self.unsigned8 += 1;
    self.unsigned16 += 2;
    self.unsigned32 += 3;
    self.unsigned64 += 4;
    self.public_key = Some(PublicKeyT{
      b: [1; 32],
    });
    self.string.as_mut().map(|x| x.push_str(", how are you?"));
    self.example_struct.as_mut().map(|x| {
      x.unsigned8 += 1;
      x.unsigned16 += 2;
      x.unsigned32 += 3;
      x.unsigned64 += 4;
      x.public_key = Some(PublicKeyT{
        b: [1; 32],
      });
    });
    
    self.example_enum = ExampleEnum::One;
    self.example_variant = ExampleVariantT::One(Box::new(OneST{
      one: 1,
      ..Default::default()
    }));
    self.array_example_struct.as_mut().map(|x| x[0].unsigned8 += 1);
    self.array_example_struct.as_mut().map(|x| x[0].unsigned16 += 2);
    self.array_example_struct.as_mut().map(|x| x[0].unsigned32 += 3);
    self.array_example_struct.as_mut().map(|x| x[0].unsigned64 += 4);
    self.array_example_struct.as_mut().map(|x| x[0].public_key = Some(PublicKeyT{
      b: [1; 32],
    }));
    self.array_example_enum.as_mut().map(|x| x[0] = ExampleEnum::One);
    self.array_example_variant1.as_zero_mut().map(|x| x.zero += 1);
    self.array_example_variant2.as_one_mut().map(|x| x.one += 1);
    self.array_example_variant3.as_two_mut().map(|x| x.two += 1);
    self.array_example_variant4.as_three_mut().map(|x| x.three += 1);
    self.array_example_variant5.as_four_mut().map(|x| x.key = Some(PublicKeyT{
      b: [1; 32],
    }));
  }
}