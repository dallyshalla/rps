extern crate rustc_serialize;

use rustc_serialize::{Decodable, Decoder};
use rustc_serialize::json::{self, ToJson, Json};

use std::error::Error;
use std::f32;
use std::env;
use std::process::Command;
use std::io::prelude::*;
use std::io;
use std::fs;
use std::str;
use std::fs::File;
use std::fs::OpenOptions;
use std::path::Path;
use std::fs::Metadata;
use std::rc::Rc;
use std::cell::RefCell;

pub fn make_app_root_dir(rootname: &str) {
	let mut the_home_dir = String::new();

	match env::home_dir() {
   		Some(ref p) => the_home_dir = p.display().to_string(),
   		None => println!("Impossible to get your home dir!")
	}

	let the_other_part = rootname;
	let the_full_path = the_home_dir + the_other_part;
	match fs::create_dir(&the_full_path) {
		Err(why) => { 
			println!("{:?}", why.kind()); 
		},
		Ok(_) => { 	
			println!("make it"); 
		},
	}
} 

#[test]
fn it_works() {
}
