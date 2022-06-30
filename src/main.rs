mod utils;

use std::fmt::Formatter;
use std::net::TcpListener;
use std::ops::Sub;
use serde::{Serialize, Deserialize};
use serde_json::Error;
use std::io::Read;


/*
----------              --------------
server:80 | <-      -- | client:62458 |
----------              --------------
    ^
    |
 ------------    
|client:62459|
 ------------

TCP: Connecté (lettre avec accusé de réception)
UDP: Par paquet (lettre simple)
 */

// Exemple de framework pour le web
// - Rocket
// - Actix (bâti sur Tokio)

#[derive(Debug, Serialize, Deserialize)]
struct Welcome {
    version: u8,
}

#[derive(Debug, Serialize, Deserialize)]
struct Subscribe {
    name: String
}

#[derive(Debug, Serialize, Deserialize)]
enum Message {
    Hello,
    Welcome(Welcome),
    // Welcome { version: u8 },
    Subscribe(Subscribe),
}


// impl std::fmt::Debug for Welcome {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         todo!()
//     }
// }

// Welcome { version: 1 }


fn main() {
    utils::f();
    
    let listener = TcpListener::bind("127.0.0.1:7676");
    let listener = match listener {
        Ok(l) => l,
        Err(err) => panic!("Cannot bind: {err}")
    };
    
    for message in listener.incoming() {
        println!("{message:?}");
        let mut message = message.unwrap();
        let mut data = Vec::<u8>::new();
        message.read_to_end(&mut data);
        println!("{data:?}");
        let str = String::from_utf8_lossy(&data);
        println!("{str}");
    }
    
    
    let serialized = "";
    
    
    
    
    
    ////// Echange du message entre client et serveur
    
    let message: Result<Message, _> = serde_json::from_str(&serialized);
    
    match message {
        Ok(m) => println!("message={m:?}"), 
        Err(err) => println!("error={err:?}")
    }
    
    
    // let record = serde_json::from_string(&record_str);
    
    // match message {
    //     Message::Welcome(_) => {}
    //     Message::Hello => {}
    //     Message::Subscribe(_) => {}
    // }
    
    
    // cible:           {"Welcome":{"version":1}}
    // résultat actuel: {"Welcome":{"version":1}}
}
