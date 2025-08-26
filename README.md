# RolleR

RolleR is a command-line dice rolling application written in Rust. It is designed for tabletop gamers, RPG enthusiasts, and anyone who needs flexible dice rolling from their terminal.

## Features

- **Standard Dice Rolls**: Roll any number of dice with any number of sides.
- **Advantage**: Simulate rolling with advantage.
- **Modifiers**: Add or subtract modifiers to your rolls.
- **Keep**: Roll multiple dice and keep only the highest or lowest results.
- **Verbose**: Show all individual dice rolls.
- **Multiple Rolls**: Repeat a roll multiple times.

## Usage

Run the application from the command line with your preferred flags. Example:

```sh
roller -d 6 -n 4 -k 3 -a -m 2 -v -t 5
```

This rolls four 6-sided dice, keeps the highest three, rolls with advantage, adds a +2 modifier, prints individual rolls, and repeats the whole process 5 times.

### Command-Line Options

- `-d`, `--sides <SIDES>` **(required)**  
  How many sides the die has (e.g., 6 for a d6).

- `-n`, `--number <NUMBER>`  
  How many dice to roll. Default: `1`.

- `-a`, `--advantage`  
  Roll with advantage.

- `-m`, `--mod <MODIFIER>`  
  Modifier to add (or subtract) to the roll. Default: `0`. Negative numbers allowed.

- `-k`, `--keep <KEEP>`  
  How many dice to keep (e.g., keep highest 3 out of 4 rolls).

- `-v`, `--verbose`  
  Print the individual rolls.

- `-t`, `--times <TIMES>`  
  How many times to roll this set. Default: `1`.

### Example Commands

- Roll a single d20:  
  `roller -d 20`
- Roll 4d6 and keep the highest 3:  
  `roller -d 6 -n 4 -k 3`
- Roll with a +5 modifier:  
  `roller -d 8 -n 2 -m 5`
- Roll with advantage:  
  `roller -d 20 -a`
- Show all individual rolls:  
  `roller -d 10 -n 6 -v`
- Repeat the roll 10 times:  
  `roller -d 4 -n 2 -t 10`

## Installation

1. Make sure you have [Rust](https://www.rust-lang.org/tools/install) installed.
2. Clone the repository:

   ```sh
   git clone https://github.com/CyTechNomad/RolleR.git
   cd RolleR
   ```

3. Build the project:

   ```sh
   cargo build --release
   ```

4. Run the executable from `target/release/roller`.

## Contributing

Contributions, suggestions, and bug reports are welcome! Please open issues or pull requests to help improve RolleR.

## Author

- CyTechNomad
