mod animation;
mod utils;

use std::{env, path::Path};

use colored::Colorize;

fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();
    if args.len() != 3 {
        errorlog!(format!("[FATAL] expected 3 arguments found {}", args.len()));
        return;
    }

    let (animation, file, lines) = (args[0].as_str(), &args[1], &args[2]);
    if !Path::new(file).exists() {
        errorlog!(format!("[FATAL] file not found: '{}'", file));
        return;
    }
    if lines.is_empty() {
        errorlog!(format!("[FATAL] no windows size"));
        return;
    }

    let lines = lines
        .splitn(2, ':')
        .filter_map(|l| l.parse::<usize>().ok())
        .collect::<Vec<_>>();
    if lines.len() != 2 {
        errorlog!(format!("[FATAL] invalid windows size"));
        return;
    }
    let window_size = lines[0]..=lines[1];

    match animation {
        "make_it_rain" => {}
        "game_of_life" => {}
        _ => errorlog!(format!(
            "[ERROR] no implemented animation for '{}'",
            animation
        )),
    }
}
