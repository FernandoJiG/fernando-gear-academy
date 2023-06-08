use gtest::{Log, Program, System};
use gstd::{prelude::*, msg, debug, ActorId};

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

#[test]
fn hello_test() {
    let sys = System::new();//The system emulates the node behaviour
    sys.init_logger();// to initialize printing logs into stdout

    /***The uploaded program has its own id. You can specify the program id manually using the current_with_id constructor.
    If you don’t specify the program id, the id of the first initialized program will be 1, and the next program initialized without an id specification will have the second id and so on.
    ***/
    //let tamagotchi_program = Program::current(&system);// we initialize our program.
    let tamagotchi_program = Program::current_with_id(&sys, 100);


    /***
    The next step, we’ll send messages to our program.
    The first argument in the send is a sender id, the second one is a message payload.
    ***/
    let name = String::from("Toby");
    let res = tamagotchi_program.send(10, name.clone());
    assert!(!res.main_failed());

    let expected_msg = res.decoded_log::<String>();
    println!("{:#?}", expected_msg);

    let res = tamagotchi_program.send(10, TmgAction::Name);
    let expected_log = Log::builder().dest(10).payload(TmgEvent::Name(name.to_string()));
    assert!(res.contains(&expected_log));//Make sure the message was sent

    /*
    let res = tamagotchi_program.send(10, TmgAction::Age);//This message will be processed through th ehandle function.
    let expected_log = Log::builder().dest(10).payload(TmgEvent::Age(15));
    assert!(res.contains(&expected_log));

    
    let expected_msg = res.decoded_log::<String>();
    println!("{:#?}", expected_msg);*/

}