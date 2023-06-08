#![no_std]

//use gstd::{prelude::*,exec ,msg, debug, ActorId};
use gstd::{prelude::*,exec ,msg, debug};

#[derive(Default, Encode, Decode, TypeInfo)]
pub struct Tamagotchi {
   pub name: String,
   pub date_of_birth: u64,
}

static mut TAMAGOTCHI_STATE : Tamagotchi = Tamagotchi{name: String::new(), date_of_birth : 0};

#[no_mangle]
extern "C" fn init(){
   let init_message: String = msg::load().expect("Can´t load the incoming message");

   unsafe{TAMAGOTCHI_STATE.name = init_message.clone()};
   unsafe{TAMAGOTCHI_STATE.date_of_birth=exec::block_timestamp()};
   debug!("Program initialized {:?}", init_message.clone());
   msg::reply("The initialization succeeded",0).expect("Unable to reply init.");
}

#[no_mangle]
extern "C" fn handle(){
   let message : TmgAction = msg::load().expect("Can't load the incoming message");
   match message{
      TmgAction::Name=>{
         let name = unsafe{&TAMAGOTCHI_STATE.name};
         let _payload = TmgEvent::Name(name.to_string());
         debug!("Message with name sent: {:?}", name.to_string());
         msg::reply(_payload, 0).expect("Can't send a `TmgAction::Name` message");
      },
      TmgAction::Age=>{
         let date = unsafe{TAMAGOTCHI_STATE.date_of_birth};
         let _payload = TmgEvent::Age(date);
         debug!("Message with age sent: {:?}", date);
         msg::reply(_payload, 0).expect("Can't send a `TmgAction::Age` message");
      }
   }

}

#[no_mangle]
extern "C" fn state(){
   let name_ = unsafe{&TAMAGOTCHI_STATE.name};
   let date = unsafe{TAMAGOTCHI_STATE.date_of_birth};
   let state : Tamagotchi = Tamagotchi{name: name_.to_string(), date_of_birth: date};
   msg::reply(state, 0).expect("Unable to share the state");
}

#[no_mangle]
extern "C" fn metahash() {
    let metahash: [u8; 32] = include!("../.metahash");
    msg::reply(metahash, 0).expect("Unable to share the metahash");
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


