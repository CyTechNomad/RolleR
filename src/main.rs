use core::fmt;
use std::{str::FromStr, usize};

use clap::{command, Arg, ArgAction, ArgMatches};
use rand::prelude::*;

struct Roll {
    properties: Properties,
    values: Vec<usize>,
}
impl fmt::Debug for Roll {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "Rolled: {}D{}{:+}, Keeping: {}, Advantage: {}\nIndividual rolls: {}\nTotal: {}",
            self.properties.number,
            self.properties.sides,
            self.properties.modifier,
            self.properties.keep.unwrap_or(self.properties.number),
            self.properties.advantage,
            self.values
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(", "),
            self.values
                .iter()
                .rev()
                .take(self.properties.keep.unwrap_or(self.properties.number))
                .sum::<usize>() as isize
                + self.properties.modifier
        )
    }
}
impl fmt::Display for Roll {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "Total: {}",
            self.values
                .iter()
                .rev()
                .take(self.properties.keep.unwrap_or(self.properties.number))
                .sum::<usize>() as isize
                + self.properties.modifier
        )
    }
}
#[derive(Copy, Clone)]
struct Properties {
    sides: usize,
    number: usize,
    advantage: bool,
    modifier: isize,
    keep: Option<usize>,
}
impl Properties {
    fn new(
        sides: usize,
        number: usize,
        advantage: bool,
        modifier: isize,
        keep: Option<usize>,
    ) -> Properties {
        Properties {
            sides,
            number,
            advantage,
            modifier,
            keep,
        }
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

fn def_commands() -> ArgMatches {
    command!()
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
        .get_matches()
}

fn main() {
    let match_result: ArgMatches = def_commands();

    let properties = Properties::new(
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
        match_result
            .get_one::<String>("keep")
            .map(|x| x.parse().ok())
            .flatten(),
    );

    let mut times = handle_error(
        match_result
            .get_one::<String>("times")
            .unwrap()
            .parse::<usize>(),
        "Times must be a positive integer",
    );

    while times > 0 {
        times -= 1;
        let mut rolls: Vec<usize> = Vec::new();
        properties.roll(&mut rolls);

        if match_result.get_flag("verbose") {
            println!(
                "{:?}",
                Roll {
                    properties,
                    values: rolls
                }
            );
            continue;
        }

        println!(
            "{}",
            Roll {
                properties,
                values: rolls
            }
        );
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
