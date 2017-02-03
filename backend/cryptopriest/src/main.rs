#![feature(custom_derive)]
#![feature(proc_macro)]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

extern crate bitcoin;
extern crate secp256k1;
extern crate rand;
extern crate futures;
extern crate env_logger;
extern crate tokio_minihttp;
extern crate tokio_proto;
extern crate tokio_service;
extern crate time;

use secp256k1::{Secp256k1, ContextFlag};
use secp256k1::key::{PublicKey, SecretKey};
use bitcoin::blockdata::block::{Block};
use bitcoin::util::address::{Address, Privkey};
use bitcoin::util::hash::{Hash160};
use bitcoin::network::constants::Network;
use bitcoin::network::serialize::{deserialize, serialize};
use bitcoin::util::base58::{ToBase58, FromBase58};
use rand::{thread_rng, Rng};
use std::io;
use futures::future;
use tokio_minihttp::{Request, Response, Http};
use tokio_proto::TcpServer;
use tokio_service::Service;

#[derive(Serialize, Deserialize, Debug)]
pub struct WeddingRequest {
    alice_pk : String,
    bob_pk : String,
    public_message : String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WeddingEvent {
    priest_address : String,
    //timestamp : time::Tm,
}

struct WeddingService;

impl Service for WeddingService {
    type Request = Request;
    type Response = Response;
    type Error = io::Error;
    type Future = future::Ok<Response, io::Error>;

    fn call(&self, _request: Request) -> Self::Future {
        // Deserialize request and get addresses of participants
        //let deserialized_request: WeddingRequest = serde_json::from_str(&_request.data).unwrap();
        let pk_alice: Address =
        FromBase58::from_base58check("2N3zXjbwdTcPsJiy8sUK9FhWJhqQCxA8Jjr").unwrap();
        let pk_bob: Address =
        FromBase58::from_base58check("2N3zXjbwdTcPsJiy8sUK9FhWJhqQCxA8Jjr").unwrap();

        let wedding_request = WeddingRequest{
            alice_pk : pk_alice.to_base58check(),
            bob_pk : pk_bob.to_base58check(),
            public_message : "MyFirstOne".to_string()
         };

        // Create new wedding event and priest credentials
        // generate keypair from secp256k1 elliptic curve
        let secp = Secp256k1::with_caps(ContextFlag::SignOnly);
        let network = Network::Bitcoin;
        let compressed = false;
        let sk = SecretKey::new(&secp, &mut thread_rng());
        let pk = PublicKey::from_secret_key(&secp, &sk).expect("Failed to create public key");

        // convert public key to Bitcoin address and private key to base58
        let address = Address::from_key(network, &pk, compressed);
        let privkey = Privkey::from_key(network, sk, compressed);
        let wedding = WeddingEvent{
            priest_address : address.to_base58check(),
            //timestamp : time::now_utc()
        };

        // Prepare response
        let mut resp = Response::new();
        let serialized_response = serde_json::to_string(&wedding).unwrap();
        //let serialized_response = r#"{ priest_address:"1Er1kCVPzyXxhogYr7biLSPQ9ZC4nL1gNw", expiration_timestamp:"0" }"#.to_string();
        resp.body(&serialized_response);
        future::ok(resp)
    }
}

fn main() {
    //Start HTTP-service
    drop(env_logger::init());
    let addr = "0.0.0.0:3000".parse().unwrap();
    TcpServer::new(Http, addr).serve(|| Ok(WeddingService));
}
