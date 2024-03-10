mod animation;
mod utils;

use std::{env, path::Path};

use animation::{gol::GameOfLife, mir::MakeItRain, Animation};
use colored::Colorize;
use utils::ErrorMsg;

fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();
    if args.len() != 3 {
        if args.len() == 1 && args[0] == "--help" || args[0] == "-h" {
            println!(
                "{}",
                "OPTIONS: [animation_type] [filepath] [lines]\nanimation_type: make_it_rain or game_of_life\nlines: inclusive range of lines (separated by ':') affected by the animation (e.g: '2:5', will only animate line 2 to 5 (inclusive)".blue()
            );
            return;
        }

        errlog!("[FATAL] expected 3 arguments found {}", args.len());
        return;
    }

    let (animation, file, lines) = (args[0].as_str(), &args[1], &args[2]);
    if !Path::new(file).exists() {
        errlog!("[FATAL] file not found: '{}'", file);
        return;
    }
    if lines.is_empty() {
        errlog!("[FATAL] no windows size");
        return;
    }

    let lines = lines
        .splitn(2, ':')
        .filter_map(|l| l.parse::<usize>().ok())
        .collect::<Vec<_>>();
    if lines.len() != 2 {
        errlog!("[FATAL] invalid windows size");
        return;
    }
    let window_size = lines[0]..=lines[1]; // line should begin at 0
    match animation {
        "make_it_rain" => MakeItRain::create_and_launch(Path::new(file), window_size).unwrap_app(),
        "game_of_life" => GameOfLife::create_and_launch(Path::new(file), window_size).unwrap_app(),
        _ => errlog!("[ERROR] no implemented animation for '{}'", animation),
    }
}
