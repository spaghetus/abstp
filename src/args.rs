use std::fs::{File, OpenOptions};

use clap::{app_from_crate, Arg};

pub struct Args {
	pub dpres: (usize, usize),
	pub tpres: (usize, usize),
	pub modifier: String,
}

impl Args {
	pub fn get() -> Args {
		let matches = app_from_crate!()
			.arg(
				Arg::with_name("resolution")
					.short("r")
					.takes_value(true)
					.required(true)
					.help("The resolution of the screen, formatted like [x,y]. Used to convert from proportional screen-space to pixel-screen-space.")
			)
			.arg(
				Arg::with_name("tpresolution")
					.short("R")
					.takes_value(true)
					.required(true)
					.help("The resolution of the touchpad, formatted like [x,y]. Used to convert from touchpad-coordinate-space to proportional screen-space.")
			)
    .arg(Arg::with_name("modifier").short("m").takes_value(true).default_value("KEY_LEFTMETA").help("The modifier key used to enable abstp."))
			.get_matches();
		Args {
			dpres: serde_json::from_str(matches.value_of("resolution").unwrap())
				.expect("Couldn't interpret display resolution"),
			tpres: serde_json::from_str(matches.value_of("tpresolution").unwrap())
				.expect("Couldn't interpret touchpad resolution"),
			modifier: matches
				.value_of("modifier")
				.unwrap_or("KEY_LEFTMETA")
				.to_string(),
		}
	}
}
