#![no_std]
use gmeta::{In, InOut, Metadata};
use gstd::{prelude::*};
pub struct ProgramMetadata;

impl Metadata for ProgramMetadata {
   type Init = In<String>;
   type Reply = ();
   type Others = ();
   type Signal = ();
   type Handle = InOut<TmgAction, TmgEvent>;
   type State = Tamagotchi;
}

#[derive(Encode, Decode, TypeInfo)]
pub enum TmgAction {
   Name,
   Age,
}

#[derive(Encode, Decode, TypeInfo)]
pub enum TmgEvent {
   Name(String),
   Age(u64),
}

#[derive(Default, Encode, Decode, TypeInfo)]
pub struct Tamagotchi {
   pub name: String,
   pub date_of_birth: u64,
}