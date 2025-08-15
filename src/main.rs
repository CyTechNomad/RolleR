use std::{str::FromStr, usize, fmt};

use clap::{command, Arg, ArgAction, ArgMatches};
use rand::prelude::*;

#[derive(Copy, Clone, Debug)]
struct Properties {
    sides: usize,
    number: usize,
    advantage: bool,
    modifier: isize,
    keep: Option<usize>,
}

impl Properties {
    fn roll(&self) -> Vec<usize> {
        let mut rng = rand::thread_rng();
        let mut rolls = Vec::with_capacity(self.number);
        for _ in 0..self.number {
            let roll = rng.gen_range(1..=self.sides);
            if self.advantage {
                let roll2 = rng.gen_range(1..=self.sides);
                rolls.push(roll.max(roll2));
            } else {
                rolls.push(roll);
            }
        }
        rolls.sort_unstable();
        rolls
    }
}

struct Roll {
    properties: Properties,
    values: Vec<usize>,
}

impl Roll {
    fn total(&self) -> isize {
        self.values
            .iter()
            .rev()
            .take(self.properties.keep.unwrap_or(self.properties.number))
            .sum::<usize>() as isize
            + self.properties.modifier
    }
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
                .collect::<Vec<_>>()
                .join(", "),
            self.total()
        )
    }
}

impl fmt::Display for Roll {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Total: {}", self.total())
    }
}

fn def_commands() -> ArgMatches {
    command!()
        .about("Rolls dice of a given number of sides and adds modifiers")
        .arg(Arg::new("sides").short('d').long("sides").required(true).help("How many sides the die has"))
        .arg(Arg::new("number").short('n').long("number").default_value("1").help("How many dice to roll"))
        .arg(Arg::new("advantage").short('a').long("advantage").action(ArgAction::SetTrue).help("Roll with advantage"))
        .arg(Arg::new("modifier").short('m').long("mod").default_value("0").allow_negative_numbers(true).help("Modifiers to add to the roll"))
        .arg(Arg::new("keep").short('k').long("keep").help("How many dice to keep"))
        .arg(Arg::new("verbose").short('v').long("verbose").action(ArgAction::SetTrue).help("Print the individual rolls"))
        .arg(Arg::new("times").short('t').long("times").default_value("1").help("How many times to roll"))
        .get_matches()
}

fn main() {
    let matches = def_commands();

    let sides = parse_arg::<usize>(&matches, "sides", "Side must be a positive integer");
    let number = parse_arg::<usize>(&matches, "number", "Number must be a positive integer");
    let advantage = matches.get_flag("advantage");
    let modifier = parse_arg::<isize>(&matches, "modifier", "Modifier must be an integer");
    let keep = matches.get_one::<String>("keep").and_then(|x| x.parse().ok());
    let mut times = parse_arg::<usize>(&matches, "times", "Times must be a positive integer");

    let properties = Properties { sides, number, advantage, modifier, keep };

    while times > 0 {
        times -= 1;
        let values = properties.roll();
        let roll = Roll { properties, values };

        if matches.get_flag("verbose") {
            println!("{:?}", roll);
        } else {
            println!("{}", roll);
        }
    }
}

fn parse_arg<T: FromStr>(matches: &ArgMatches, key: &str, msg: &str) -> T {
    matches
        .get_one::<String>(key)
        .and_then(|v| v.parse::<T>().ok())
        .unwrap_or_else(|| {
            eprintln!("{}", msg);
            std::process::exit(1);
        })
}
