mod FormData;

use std::borrow::Borrow;
use std::net::TcpStream;
use std::io::{Read, Write};
use std::str::from_utf8;
use std::{env, thread, time};
use std::thread::sleep;
use std::time::Duration;

use serde::de::Unexpected::Str;
use serde_json::{Result, Value};
use serde_json::Value::Null;
use serde::{Serialize, Deserialize};
use crate::FormData::PublicPlayer;
// use crate::Toto::SubscribeResult;

#[macro_use]
extern crate serde_json;


fn main() {
    let userName = get_username();
    let mut stream = TcpStream::connect("localhost:7878");

// loop
    match stream {
        Ok(mut stream) => {
            //Connect to server
            send_message(&mut stream, "\"Hello\"".to_string());

            //Exec
            loop {
                let size = len_stream(&mut stream);
                if size !=0 {
                    let buff: FormData::MessageResponse = return_value(&mut stream, size);
                    match &buff {
                        FormData::MessageResponse::Welcome(res) => welcome(&mut stream, res, userName.clone()),
                        FormData::MessageResponse::SubscribeResult(res) => subscribe_result(res),
                        FormData::MessageResponse::Challenge(_) => println!("here subscribe"),
                        FormData::MessageResponse::PublicLeaderBoard(res) => leader_board(res),
                        FormData::MessageResponse::EndOfGame(_) => {
                            println!("Close connection...");
                            break;
                        },
                        FormData::MessageResponse::RoundSummary(_) => {
                            println!("Round summary");

                        },
                        None => continue
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
fn welcome(stream: &mut TcpStream, res: &Value, userName: String) {
    println!("here welcome, version: {}", res["version"]);
    send_message(stream, "{\"Subscribe\":{\"name\":\"".to_owned() + &userName +"\"}}");
}

fn subscribe_result(res: &FormData::SubscribeResult) {
    match &res {
        FormData::SubscribeResult::Ok => println!("Subscribed !"),
        FormData::SubscribeResult::Error(val) => println!("here subscribe : inio {}", val),
    }

}

fn leader_board(res: &Vec<PublicPlayer>) {
    println!("Leaderboard : ");
    for player in res {
        println!("Player : {}", player.name);
    }
}

//-------- End Message Traitment --------


//-------- TCP Traitment --------

fn return_value(stream: &mut TcpStream, mut size: u32) -> FormData::MessageResponse {
    let mut data = Vec::new();
    data.resize(size.try_into().unwrap(), 0);
    match stream.read(&mut data) {
        Ok(_) => {
            let v: FormData::MessageResponse = serde_json::from_str(from_utf8(&data).unwrap()).unwrap();
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
    let mut n : u32 = message.len() as u32;
    stream.write_all(&n.to_be_bytes()).unwrap(); //.unwrap()
    stream.write_all(&message.as_bytes()).unwrap(); //.unwrap()
}
//------- End TCP Traitment --------

fn get_username() -> String {
    let mut args: Vec<String> = env::args().collect();
    if args.len() < 1 {
        return args[1].to_string();
    }
    return "fred".to_string();
}