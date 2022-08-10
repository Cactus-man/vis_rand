use clap::{value_parser, Arg, Command};
use dice::*;
use regex::{Regex, RegexSet};

fn main() {
    let is_die = Regex::new(r"^(\d+d)").unwrap();
    let patterns =
        RegexSet::new(&[r"^\d+$", r"^\[((?:\d+\s*:\s*\d+,)*\d+\s*:\s*\d+,?)\]$"]).unwrap();

    let args = Command::new("Dice Simulator")
        .args(&[
            Arg::new("rolls")
                .index(1)
                .required(true)
                .value_parser(value_parser!(usize)),
            Arg::new("dice").index(2).required(true).min_values(1),
        ])
        .get_matches();

    let rolls: usize = *args.get_one("rolls").expect("rolls is required");

    let dice: Vec<_> = args
        .get_many::<String>("dice")
        .unwrap()
        .filter(|s| is_die.is_match(s))
        .map(|s| s.split_once('d').unwrap())
        .map(|(amount, s)| (amount.parse::<u8>().unwrap(), s))
        .map(|(amount, s)| {
            let kind = match patterns.matches(s).into_iter().next().unwrap() {
                0 => DieKind::simple(s.parse().unwrap()),
                1 => {
                    let sides: Vec<(usize, u16)> = s
                        .strip_prefix('[')
                        .unwrap()
                        .strip_suffix(']')
                        .unwrap()
                        .split(',')
                        .map(|s| s.split_once(':').unwrap())
                        .map(|(w, d)| (d.parse().unwrap(), w.parse().unwrap()))
                        .collect();

                    DieKind::weighted(&sides)
                }
                _ => unreachable!(),
            };
            (kind, amount)
        })
        .collect();

    let mut counts: Vec<_> = roll_threaded(dice, rolls).into_iter().collect();
    counts.sort_unstable_by_key(|&(dots, _)| dots);

    for (dots, count) in counts {
        println!("{dots}: {count}")
    }
}
