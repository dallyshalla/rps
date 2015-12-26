extern crate rpslib;
extern crate rustc_serialize;

use rustc_serialize::{Decodable, Decoder};
use rustc_serialize::json::{self, ToJson, Json};

use std::io::{self,BufRead};

use rpslib::*;

struct Tournament {
	size: i64,
	deadline: i64,
}

fn main() {

	make_app_root_dir("/rockgame");
	println!("register a name");
	let mut a_name = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut a_name).unwrap();

    println!("register a move");
	let mut a_move = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut a_move).unwrap();

    println!("{:?}", a_name);
    println!("{:?}", a_move);

	
}