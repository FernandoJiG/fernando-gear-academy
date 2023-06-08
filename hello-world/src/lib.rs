#![no_std]

use gstd::{prelude::*, msg, debug, ActorId};
static mut GREETING: Option<String> = None; //Option can be String or None

#[derive(Encode, Decode, TypeInfo, Debug)]//Encode and decode to be able to serialize and deserialize the structure, debug to use code lines of debug type.
pub enum InputMessage {
    SendHelloTo(ActorId),
    SendHelloReply,
}

#[no_mangle]
extern "C" fn init(){
    let init_message: String = msg::load().expect("Can´t load the incoming message");
    debug!("Program initialized {:?}", init_message);

    unsafe{GREETING=Some(init_message)}

    unsafe{debug!{"GREETING value: {:?}", GREETING}};
}

use InputMessage::*;

#[no_mangle]
extern "C" fn handle(){
    debug!("Handle");
    //let init_mes: String = msg::load().expect("Can´t load the incoming message");
    let message: InputMessage = msg::load().expect("Can´t load the incoming message");
    debug!("Incoming message {:?}", message);
    //I get a reference from GREETING instead of sending a string with String::from(GREETING)
    let greeting = unsafe{GREETING.as_ref().expect("The contract is not initialized")};
    match message{
        SendHelloTo(account)=>{//I have another destiny to which I want to send the message.
            msg::send(account, greeting, 0).expect("Can't send a `SendHelloTo` message");
        },
        SendHelloReply => {
            msg::reply(greeting, 0).expect("Can't send a `SendHelloReply` message");
        }
    }
    /*
    debug!(init_mes);
    msg::reply(String::from("Hello people"),0).expect("Error while trying to send reply");*/


}

#[no_mangle]
extern "C" fn state() {
    let state = unsafe {GREETING.as_ref().expect("The contract is not initialized")};
    msg::reply(state, 0).expect("Unable to share the state");
}

#[no_mangle]
extern "C" fn metahash() {
    // It returns the hash of metadata.
    // .metahash is generating automatically
    // while you are using build.rs
    let metahash: [u8; 32] = include!("../.metahash");
    msg::reply(metahash, 0).expect("Unable to share the metahash");
}




