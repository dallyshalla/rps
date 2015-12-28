extern crate rpslib;
extern crate rustc_serialize;
extern crate sodiumoxide;
#[macro_use] 
extern crate t_bang;

use t_bang::*;

use sodiumoxide::crypto::box_;

use rustc_serialize::{Decodable, Decoder};
use rustc_serialize::json::{self, ToJson, Json};

use std::io::{self,BufRead};

use rpslib::*;
#[derive(Clone, Debug, RustcDecodable, RustcEncodable)]
struct Player {
	name: String,
    player_pk: sodiumoxide::crypto::box_::curve25519xsalsa20poly1305::PublicKey,
    player_sk: sodiumoxide::crypto::box_::curve25519xsalsa20poly1305::SecretKey,
    wins: i64,
    ties: i64,
    lose: i64,
    tournament_wins: i64,
}

#[derive(Clone, Debug, RustcDecodable, RustcEncodable)]
struct TournamentRoll {
    roll: 
}






#[derive(Clone, Debug, RustcDecodable, RustcEncodable)]
struct Tournament {
	tournament_rolls: Vec<TournamentRoll>,
}


fn main() {
    println!("generating game files");
    println!("---------------------");

    make_app_root_dir("/rockgame");

    println!("---------------");
    println!("register a name");
    let mut a_name = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut a_name).unwrap();

    let (ourpk, oursk) = box_::gen_keypair();

    let new_player = Player {
        name: a_name,
        player_pk: ourpk,
        player_sk: oursk,
        wins: 0,
        ties: 0,
        lose: 0,
        tournament_wins: 0,
    }
    
    println!("register a move");
    println!("Pick 1,2, or 3 for:");
    println!("rock (1), paper(2), or scissors(3)");
    let mut a_move = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut a_move).unwrap();

// normally theirpk is sent by the other party
let (theirpk, theirsk) = box_::gen_keypair();
let nonce = box_::gen_nonce();
let plaintext = b"some data";
let ciphertext = box_::seal(plaintext, &nonce, &theirpk, &oursk);
let their_plaintext = box_::open(&ciphertext, &nonce, &ourpk, &theirsk).unwrap();
assert!(plaintext == &their_plaintext[..]);


    println!("register a move");
    println!("Pick 1,2, or 3 for:");
	println!("rock (1), paper(2), or scissors(3)");
	let mut a_move = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut a_move).unwrap();

    println!("{:?}", a_name);
    println!("{:?}", a_move);

	
}