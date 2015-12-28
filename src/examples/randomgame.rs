extern crate rpslib;
extern crate rustc_serialize;
extern crate rand;
use rustc_serialize::{Decodable, Decoder};
use rustc_serialize::json::{self, ToJson, Json};

use rand::Rng;
use std::io::{self,BufRead};

use rpslib::*;

fn main() {
	let mut loop_index = 1;
	let mut wins = 0;
	let mut ties = 0;
	let mut lose = 0;
	println!("Pick 1,2, or 3 for:");
	println!("rock (1), paper(2), or scissors(3)");
	while (loop_index != 0) {
		// 1 = rock
		// 2 = paper
		// 3 = scissors
		let mut rng = rand::thread_rng();
		let mut a_move = String::new();
    	let stdin = io::stdin();
    	stdin.lock().read_line(&mut a_move).unwrap();

		let randomNumber = rng.gen_range(1, 3);
		if (randomNumber == 1) {
			if (a_move == "1\n") {
				println!("you selected rock");
				println!("computer selected rock");
				println!("rock vs. rock = its a tie");
				ties += 1;
			} else if (a_move == "2\n") {
				println!("you selected paper");
				println!("computer selected rock");
				println!("paper vs. rock = you win");
				wins += 1;
			} else if (a_move == "3\n") {
				println!("you selected scissors");
				println!("computer selected rock");
				println!("scissors vs. rock = you lose");
				lose += 1;
			}
		} else if (randomNumber == 2) {
			if (a_move == "1\n") {
				println!("you selected rock");
				println!("computer selected paper");
				println!("rock vs. paper = you lose");
				lose += 1;
			} else if (a_move == "2\n") {
				println!("you selected paper");
				println!("computer selected paper");
				println!("paper vs. paper = its a tie");
				ties += 1;
			} else if (a_move == "3\n") {
				println!("you selected scissors");
				println!("computer selected paper");
				println!("scissors vs. paper = you win");
				wins += 1;
			}

		} else if (randomNumber == 3) {
			if (a_move == "1\n") {
				println!("you selected rock");
				println!("computer selected scissors");
				println!("rock vs. scissors = you win");
				wins += 1;
			} else if (a_move == "2\n") {
				println!("you selected paper");
				println!("computer selected scissors");
				println!("paper vs. scissors = you lose");
				lose += 1;
			} else if (a_move == "3\n") {
				println!("you selected scissors");
				println!("computer selected scissors");
				println!("scissors vs. scissors = its a tie");
				ties += 1;
			}
		}

    	println!("Do you want to play again? yes(1)/no and exit(2)/scoreboard and restart(3)");
    
    	let mut input = String::new();
    	let stdin = io::stdin();
    	stdin.lock().read_line(&mut input).unwrap();

    	if(input=="1\n") {
      		println!("rock, paper, scissors");
      		loop_index = 1;	
    	}
		else if(input=="2\n") {
    		loop_index = 0;
    		println!("goodbye");
		}
		else if(input=="3\n") {
			println!("wins : {:?}", wins);
			println!("losses : {:?}", lose);
			println!("ties : {:?}", ties);
      		println!("rock, paper, scissors");
			loop_index = 1;
		}
	}	
}