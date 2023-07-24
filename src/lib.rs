// #![warn(missing_docs)]   // TODO: re-enable

mod message;
mod server;

pub use server::RendezvousServer;

use message::{Message, MessageError, PingRes};
use std::error::Error;

// TODO: remove dummy()
pub fn dummy() {
    // let msg_id = Message::random_msg_id();

    // let message = Message::PingReq(PingReq {
    //     msg_id,
    //     from_peer_id: String::from("abc-1"),
    //     to_peer_id: String::from("ddd-2"),
    // });
    // let output: Vec<u8> = message.to_allocvec().unwrap();
    // println!("{output:?}");
    // let message: Message = Message::from_bytes(output.as_slice()).unwrap();
    // println!("{message:?}");

    // let message = Message::PingRes(PingRes {
    //     _dummy: false,
    //     msg_id,
    // });
    // let mut output = message.to_allocvec().unwrap();

    // output[1] = 11;

    // println!("{output:?}");
    // let message = Message::from_bytes(output.as_slice());

    // match &message {
    //     Ok(m) => println!("message {m:?}"),
    //     Err(e) => {
    //         println!("error: {e}");
    //         println!("error: {e:?}");
    //         message.unwrap();
    //     }
    // }

    let message = Message::PingRes(PingRes {
        _dummy: false,
        msg_id: 0,
    });
    let mut data_invalid = message.to_allocvec().unwrap();
    data_invalid[1] = 11;
    let message = Message::from_bytes(data_invalid.as_slice());
    match &message {
        Ok(m) => println!("message {:?}", m),
        Err(e) => {
            println!("DISPLAY...");
            println!("error: {}", e);
            println!();
            println!("DEBUG...");
            println!("error: {:?}", e);
            println!();

            match e {
                MessageError::FromBytes { .. } => {
                    println!("Expected error...");
                    println!("error: {:?}", e);
                    let mut source: Option<&(dyn Error + 'static)> = e.source();

                    while let Some(s) = source {
                        println!("source: {}, {:?}", s, s);
                        source = s.source();
                    }
                }
                e => {
                    println!("Unexpected error...");
                    println!("error: {:?}", e);
                    let mut source: Option<&(dyn Error + 'static)> = e.source();

                    while let Some(s) = source {
                        println!("source: {}, {:?}", s, s);
                        source = s.source();
                    }
                }
            }
        }
    }
}
