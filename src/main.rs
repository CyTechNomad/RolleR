use std::str::FromStr;

use clap::{command, Arg, ArgAction, ArgMatches};
use rand::prelude::*;

struct Roll {
    sides: usize,
    number: usize,
    advantage: bool,
    modifier: isize,
    keep: isize,
}
impl Roll {
    fn new(sides: usize, number: usize, advantage: bool, modifier: isize, mut keep: isize) -> Roll {
        if keep < 1 {
            keep = number as isize;
        }

        if keep > number as isize {
            println!("You can't keep more dice than you rolled");
            std::process::exit(1);
        }

        Roll {
            sides,
            number,
            advantage,
            modifier,
            keep,
        }
    }

    fn display(&self, rolls: Vec<usize>, verbose: bool) {
        if verbose {
            println!(
                "Rolled: {}D{}{:+}, Keeping: {}, Advantage: {}",
                self.number, self.sides, self.modifier, self.keep, self.advantage
            );

            println!(
                "Individual rolls: {}",
                rolls
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            );
        }

        println!(
            "You rolled a {}",
            ((rolls.iter().rev().take(self.keep as usize).sum::<usize>() as isize) + self.modifier)
                .max(0)
        );
    }

    fn roll(&self, rolls: &mut Vec<usize>) {
        let mut rng = rand::thread_rng();
        let mut number = self.number;
        while number > 0 {
            let droll = rng.gen_range(1..=self.sides) as usize;
            if self.advantage {
                let droll2 = rng.gen_range(1..=self.sides) as usize;
                rolls.push(droll.max(droll2));
                number -= 1;
                continue;
            }

            rolls.push(droll);
            number -= 1;
        }
        rolls.sort();
    }
}

fn main() {
    let match_result: ArgMatches = command!()
        .about("Rolls dice of a given number of sides and adds modifiers")
        .arg(
            Arg::new("sides")
                .short('d')
                .long("sides")
                .required(true)
                .help("how many sides the die has"),
        )
        .arg(
            Arg::new("number")
                .short('n')
                .long("number")
                .required(false)
                .default_value("1")
                .help("how many dice to roll"),
        )
        .arg(
            Arg::new("advantage")
                .short('a')
                .long("advantage")
                .required(false)
                .action(ArgAction::SetTrue)
                .help("roll with advantage"),
        )
        .arg(
            Arg::new("modifier")
                .value_name("MOD")
                .short('m')
                .long("mod")
                .required(false)
                .default_value("0")
                .default_missing_value("0")
                .allow_negative_numbers(true)
                .help("modifiers to add to the roll"),
        )
        .arg(
            Arg::new("keep")
                .short('k')
                .long("keep")
                .required(false)
                .default_value("-1")
                .help("how many dice to keep"),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .required(false)
                .action(ArgAction::SetTrue)
                .help("print the individual rolls"),
        )
        .arg(
            Arg::new("times")
                .short('t')
                .long("times")
                .required(false)
                .default_value("1")
                .help("how many times to roll"),
        )
        .get_matches();

    let mut times = handle_error(
        match_result
            .get_one::<String>("times")
            .unwrap()
            .parse::<usize>(),
        "Times must be a positive integer",
    );

    let roll = Roll::new(
        handle_error(
            match_result
                .get_one::<String>("sides")
                .unwrap()
                .parse::<usize>(),
            "Side must be a positive integer",
        ),
        handle_error(
            match_result
                .get_one::<String>("number")
                .unwrap()
                .parse::<usize>(),
            "Number must be a positive integer",
        ),
        match_result.get_flag("advantage"),
        handle_error(
            match_result
                .get_one::<String>("modifier")
                .unwrap()
                .parse::<isize>(),
            "Modifier must be an integer",
        ),
        handle_error(
            match_result
                .get_one::<String>("keep")
                .unwrap()
                .parse::<isize>(),
            "Keep must be a positive intiger",
        ),
    );

    while times > 0 {
        let mut rolls: Vec<usize> = Vec::new();
        roll.roll(&mut rolls);

        roll.display(rolls, match_result.get_flag("verbose"));

        times -= 1;
    }
}

fn handle_error<T>(result: Result<T, T::Err>, msg: &str) -> T
where
    T: FromStr,
{
    match result {
        Ok(x) => x,
        Err(_) => {
            println!("{}", msg);
            std::process::exit(1);
        }
    }
}
