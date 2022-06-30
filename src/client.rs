mod form_data;

use std::net::TcpStream;
use std::io::{Read, Write};
use std::str::from_utf8;
use std::{env, thread, time};

use serde_json::{Value};
use crate::form_data::PublicPlayer;

fn main() {
    let user_name = get_username();
    let stream = TcpStream::connect("localhost:7878");

// loop
    match stream {
        Ok(mut stream) => {
            //Connect to server
            send_message(&mut stream, "\"Hello\"".to_string());

            //Exec
            loop {
                let size = len_stream(&mut stream);
                if size !=0 {
                    let buff: form_data::MessageResponse = return_value(&mut stream, size);
                    match &buff {
                        form_data::MessageResponse::Welcome(res) => welcome(&mut stream, res, user_name.clone()),
                        form_data::MessageResponse::SubscribeResult(res) => subscribe_result(res),
                        form_data::MessageResponse::Challenge(_) => println!("here game"),
                        form_data::MessageResponse::PublicLeaderBoard(res) => leader_board(res),
                        form_data::MessageResponse::EndOfGame(_) => {
                            println!("Close connection...");
                            break;
                        },
                        form_data::MessageResponse::RoundSummary(value) => round_summary(value)
                    }
                }

                let ten_millis = time::Duration::from_millis(1000);
                thread::sleep(ten_millis);
            }

            //close connection
            // TcpStream::shutdown(&stream, Shutdown::Read);
            drop(stream);
        }

        Err(err) => panic!("Cannot connect: {err}")
    }
}


//-------- Message Traitment --------
fn welcome(stream: &mut TcpStream, res: &Value, user_name: String) {
    println!("here welcome, version: {}", res["version"]);
    send_message(stream, "{\"Subscribe\":{\"name\":\"".to_owned() + &user_name +"\"}}");
}

fn subscribe_result(res: &form_data::SubscribeResult) {
    match &res {
        form_data::SubscribeResult::Ok => println!("Subscribed !"),
        form_data::SubscribeResult::Error(val) => println!("Message :  {}", val),
        form_data::SubscribeResult::Err(val) => println!("Message : {}", val),
    }

}

fn leader_board(res: &Vec<PublicPlayer>) {
    println!("Leaderboard : ");
    for player in res {
        println!("Player : {}", player.name);
    }
}

fn round_summary(res : &Value) {
    println!("-----------------");
    println!("Round summary : ");
    println!("{}", res);
    println!("-----------------");
}

//-------- End Message Traitment --------


//-------- TCP Traitment --------

fn return_value(stream: &mut TcpStream, size: u32) -> form_data::MessageResponse {
    let mut data = Vec::new();
    data.resize(size.try_into().unwrap(), 0);
    match stream.read(&mut data) {
        Ok(_) => {
            let v: form_data::MessageResponse = serde_json::from_str(from_utf8(&data).unwrap()).unwrap();
            return v;
        },
        Err(e) => {
            println!("Failed to receive data: {}", e);
        }
    }
    return serde_json::from_str(from_utf8((&"").as_ref()).unwrap()).unwrap();
}

fn len_stream(stream: &mut TcpStream) -> u32 {
    let mut number = [0 as u8; 4];
    match stream.read_exact(&mut number) {
        Ok(_) => {
            let s = u32::from_be_bytes(number);
            return s;
        },
        Err(e) => {
            println!("Failed to get length: {}", e);
        }
    }
    return 0;
}

fn send_message(stream: &mut TcpStream, message: String) {
    // let message = "\"Hello\""; //big Endian
    println!("{}", message.len() as u32);
    let n : u32 = message.len() as u32;
    stream.write_all(&n.to_be_bytes()).unwrap(); //.unwrap()
    stream.write_all(&message.as_bytes()).unwrap(); //.unwrap()
}
//------- End TCP Traitment --------

fn get_username() -> String {
    let args: Vec<String> = env::args().collect();
    if args.len() < 1 {
        return args[1].to_string();
    }
    return "fred".to_string();
}