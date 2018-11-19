mod todo;
use todo::modify::{add_entry, remove_entry, ufin_entry, clear_entrys};
use todo::list::list_entrys;
use todo::help::help;

extern crate chrono;
use chrono::prelude::*;

use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let _conf = run(&args).unwrap_or_else(|err|    {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
}

fn run(args: &[String]) -> Result<(), &'static str> {
    let mut home_path = String::new();
    let path = String::from("/.config/todo/entrys.sav");

    let _hv = match env::var("HOME") {
        Ok(_hv) => home_path.push_str(&_hv),
        Err(_e) => panic!("Error: {}", _e),
    };

    home_path.push_str(&path);

    if args.len() < 2 || args.len() > 3 {
        return Err("Usage error: 'todo -h' for help");
    }
    else {
        let date = Local::now().format("%H:%M:%S %Y-%m-%d").to_string();

        if args.len() == 2 {
            match args[1].as_ref() {
                "-l" => list_entrys(home_path),
                "-c" => clear_entrys(home_path),
                "-h" => help(), 
                _ => return Err("Option not recognized or program misuse, read -h for help"),
            }
        }
        else if args.len() == 3 {
            let item = &args[2];
            match args[1].as_ref() {
                "-a" => add_entry(home_path, item, date),
                "-f" => ufin_entry(home_path, item, true),
                "-u" => ufin_entry(home_path, item, false),
                "-r" => remove_entry(home_path, item),
                _ => return Err("Option not recognized or program misuse, read-h for help"),
            }
        }
    }

    Ok(())
}
