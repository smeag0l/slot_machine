extern crate rand;

use std::io;
use std::io::Write;
use rand::Rng;

const SKULL: char = '\u{1F480}';
const BOMB: char = '\u{1F4A3}';
const OCTOPUS: char = '\u{1F991}';

struct Wallet{
    cash: i32
}

fn main() {    
    instructions();
    play();
}

fn instructions() {
    println!("Slot Machine");
    println!("~~~~~~~~~~~~");
    println!("{} {} {} = $5", SKULL, SKULL, SKULL);
    println!("{} {} {} = $20", BOMB, BOMB, BOMB);
    println!("{} {} {} = $40", OCTOPUS,OCTOPUS,OCTOPUS);
    println!("Stake = $5");
    println!("~~~~~~~~~~~~");
}

fn play(){
    let mut wallet = Wallet {
        cash: 100
    };
    println!("You have ${}",wallet.cash);
    while wallet.cash > 0 && prompt() {
        wallet.cash -= 5;
        spin(&mut wallet);
        println!("You have ${}",wallet.cash);
    }
    cash_out(wallet);
}

fn prompt() -> bool{
    println!("play again or (c)ash out?");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice)
            .expect("Failed to read line");
    if choice.trim() == "c" {
        false
    }
    else {
        true
    }
}

fn spin (wallet : &mut Wallet) {
    let reels: [char; 3] = [get_reel(),
                            get_reel(),
                            get_reel()];
    print!("{} ", reels[0]);
    print!("{} ", reels[1]);
    print!("{} ", reels[2]);
    io::stdout().flush().unwrap();
    println!("");

    let payout = calculate_payout(reels[0],reels[1],reels[2]);

    println!("You made ${}", payout);
    wallet.cash += payout;

}

fn get_reel() -> char {
    let spin = rand::thread_rng().gen_range(1, 4);
    if spin == 1 {
        SKULL
    } else if spin == 2 {
        BOMB
    } else {
        OCTOPUS
    }
}

fn calculate_payout(reel_1:char, reel_2: char, reel_3: char) -> i32{
    let mut payout = 0;
    if reel_1 == reel_2 && reel_2 == reel_3 {
        if reel_1 == SKULL {
            payout = 5;
        } else if reel_2 == BOMB {
            payout = 20;
        } else if reel_3 == OCTOPUS {
            payout = 40;
        }
    }
    payout
}

fn cash_out(wallet :Wallet){
    println!("You finished with ${}", wallet.cash);
}

