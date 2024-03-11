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

    fn display(&self, mut rolls: Vec<usize>, verbose: bool) {
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

        if self.keep < rolls.len() as isize {
            let mut keep = self.keep;
            while keep < rolls.len() as isize {
                let min = rolls.iter().min().unwrap();
                let min_index = rolls.iter().position(|&x| x == *min).unwrap();
                rolls.remove(min_index);
                keep += 1;
            }
        }

        println!(
            "You rolled a {}",
            ((rolls.into_iter().sum::<usize>() as isize) + self.modifier).max(0)
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

    let mut times = match match_result
        .get_one::<String>("times")
        .unwrap()
        .parse::<usize>()
    {
        Ok(x) => x,
        Err(_) => {
            println!("Times must be a positive integer");
            std::process::exit(1);
        }
    };

    let roll = Roll::new(
        match match_result
            .get_one::<String>("sides")
            .unwrap()
            .parse::<usize>()
        {
            Ok(x) => x,
            Err(_) => {
                println!("Side must be a positive integer");
                std::process::exit(1);
            }
        },
        match match_result
            .get_one::<String>("number")
            .unwrap()
            .parse::<usize>()
        {
            Ok(x) => x,
            Err(_) => {
                println!("Number must be a positive integer");
                std::process::exit(1);
            }
        },
        match_result.get_flag("advantage"),
        match match_result
            .get_one::<String>("modifier")
            .unwrap()
            .parse::<isize>()
        {
            Ok(x) => x,
            Err(_) => {
                println!("Modifier must be an integer");
                std::process::exit(1);
            }
        },
        match match_result
            .get_one::<String>("keep")
            .unwrap()
            .parse::<isize>()
        {
            Ok(x) => x,
            Err(_) => {
                println!("Keep must be a positive intiger");
                std::process::exit(1);
            }
        },
    );

    while times > 0 {
        let mut rolls: Vec<usize> = Vec::new();
        roll.roll(&mut rolls);

        roll.display(rolls, match_result.get_flag("verbose"));

        times -= 1;
    }
}
