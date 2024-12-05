use clap::Parser;
use days::Days;

mod day5;
mod days;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct AyoCli {
    #[arg(short, long)]
    day: Option<i32>,
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    real_data: Option<bool>,
    #[arg(long, action = clap::ArgAction::SetTrue)]
    debug: Option<bool>,
}

fn main() {
    let args = AyoCli::parse();

    let selected = if let Some(d) = args.day {
        if d < 1 {
            println!("Invalid day {d:0>2}!");
            return;
        }
        println!("Selected day {d:0>2}!");
        d
    } else {
        // if no day is specified, just run all days
        println!("Selected all days!");
        0
    };

    let day = Days::from(selected);
    match day {
        Days::All => todo!(),
        Days::Day5 => day5::execute(&day, args.real_data.unwrap_or(false)),
        _ => todo!("{day:?}"),
    }
}
