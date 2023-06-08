#![no_std]//To delete the duplicate lang item in create 'gstd' error
use gmeta::{InOut, Metadata};
use gstd::{prelude::*, msg, debug, ActorId};

pub struct HelloMetadata;

#[derive(Encode, Decode, TypeInfo, Debug)]
pub enum InputMessage {
    SendHelloTo(ActorId),
    SendHelloReply,
}

impl Metadata for HelloMetadata {
    type Init = InOut<String, ()>; //The input is a String and the output is nothing
    type Handle = InOut<InputMessage, String>;//The input is of InputMessage enum type and the putput is a String
    type State = String;//The state will return a String
    type Reply = (); 
    type Signal = ();
    type Others = (); 
 }
