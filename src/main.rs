use std::io::BufRead;

use args::Args;
use chrono::{DateTime, Duration, Local};
use regex::Regex;

mod args;

#[macro_use]
extern crate clap;

fn main() {
	let args = args::Args::get();
	let mut should_update = false;
	let mut position = (0usize, 0usize);
	let mut last_move = Local::now();
	let event_regex = Regex::new(
		r"^Event: time \d*\.\d*, type \w* \(([\w_]*)\), code \w* \(([\w_]*)\), value (\w*)",
	)
	.unwrap();
	for line in std::io::stdin().lock().lines() {
		let now = Local::now();
		if now - last_move < Duration::milliseconds(50) {
			continue;
		}
		let line = line.expect("Couldn't read stdin");
		if !(line.contains("ABS") || line.contains("KEY_")) {
			continue;
		}
		let matches = event_regex.captures(&line);
		match matches {
			None => {}
			Some(matches) => {
				let matches = matches
					.iter()
					.map(|v| v.map(|v| v.as_str()))
					.collect::<Vec<Option<&str>>>();
				match matches.as_slice() {
					[_, Some("EV_KEY"), Some(key), Some(state_str)] if key == &&args.modifier => {
						let state: u8 = state_str.parse().unwrap();
						should_update = state != 0;
					}
					[_, Some("EV_ABS"), Some("ABS_MT_POSITION_X"), Some(value_str)] => {
						if should_update {
							position.0 = value_str.parse().unwrap();
						}
					}
					[_, Some("EV_ABS"), Some("ABS_MT_POSITION_Y"), Some(value_str)] => {
						if should_update {
							position.1 = value_str.parse().unwrap();
							set_position(&mut last_move, position, &args);
						}
					}
					_ => {}
				}
			}
		}
	}
}

fn set_position(last_move: &mut DateTime<Local>, target: (usize, usize), args: &Args) {
	*last_move = Local::now();
	// Convert to relative space
	let target = (
		target.0 as f64 / args.tpres.0 as f64,
		target.1 as f64 / args.tpres.1 as f64,
	);
	// Convert to pixel space
	let target = (
		target.0 * args.dpres.0 as f64,
		target.1 * args.dpres.1 as f64,
	);
	// Convert to integer
	let target = (target.0 as isize, target.1 as isize);
	println!("mousemove --sync {} {}", target.0, target.1)
}
