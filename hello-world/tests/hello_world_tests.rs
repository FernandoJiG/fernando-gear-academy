use gtest::{Log, Program, System};
use gstd::{prelude::*, msg, debug, ActorId};

#[derive(Encode, Decode, TypeInfo, Debug)]
pub enum InputMessage {
    SendHelloTo(ActorId),
    SendHelloReply,
}

#[test]
fn hello_test() {
    let sys = System::new();//The system emulates the node behaviour
    sys.init_logger();// to initialize printing logs into stdout

    /***The uploaded program has its own id. You can specify the program id manually using the current_with_id constructor.
    If you don’t specify the program id, the id of the first initialized program will be 1, and the next program initialized without an id specification will have the second id and so on.
    ***/
    //let hello_program = Program::current(&system);// we initialize our program.
    let hello_program = Program::current_with_id(&sys, 100);


    /***
    The next step, we’ll send messages to our program.
    The first argument in the send is a sender id, the second one is a message payload.
    ***/
    let greeting = String::from("Hello World!");
    let res = hello_program.send(10, greeting.clone());
    assert!(!res.main_failed());

    let recipient = 200;
    let res = hello_program.send(10, InputMessage::SendHelloTo(recipient.into()));
    let expected_log = Log::builder().dest(recipient).payload(greeting.clone());
    assert!(res.contains(&expected_log));//Make sure the message was sent

    let expected_msg = res.decoded_log::<String>();
    println!("{:#?}", expected_msg);

    let res = hello_program.send(10, InputMessage::SendHelloReply);//This message will be processed through th ehandle function.
    let expected_log = Log::builder().dest(10).payload(greeting.clone());
    assert!(res.contains(&expected_log));

    let expected_msg = res.decoded_log::<String>();
    println!("{:#?}", expected_msg);

}