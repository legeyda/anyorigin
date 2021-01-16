
extern crate anyorigin;
extern crate log;
extern crate docopt;
extern crate rustc_serialize;

use docopt::Docopt;
use std::ascii::AsciiExt;
use std::process::exit;


static USAGE: &'static str="
Usage: anyorigin [options]

Options:
	-a, --address ADDRESS  bind to specified address [default: 0.0.0.0:80]
	-l, --log-level LOGLEVEL  log level (one of error, warn, info, debug, trace) [default: warn]
	-h, --help  display this help and exit
	-v, --version  output version information and exit
";

#[derive(RustcDecodable)]
#[allow(dead_code)]
struct Args {
	flag_address: String,
	flag_log_level: String,
	flag_help: bool,
	flag_version: bool
}

fn parse_log_level(name: &str) -> Option<log::LogLevel> {
	if "error".eq_ignore_ascii_case(name) {
		Some(log::LogLevel::Error)
	} else if "warn".eq_ignore_ascii_case(name) {
		Some(log::LogLevel::Warn)
	} else if "info".eq_ignore_ascii_case(name) {
		Some(log::LogLevel::Info)
	} else if "debug".eq_ignore_ascii_case(name) {
		Some(log::LogLevel::Debug)
	} else if "trace".eq_ignore_ascii_case(name) {
		Some(log::LogLevel::Trace)
	} else {
		None
	}
}


fn main() {
	let args: Args = Docopt::new(USAGE).and_then(|d| d.decode()).unwrap_or_else(|e| e.exit());
	
	if true == args.flag_version {
		println!("{}", env!("CARGO_PKG_VERSION"));
		exit(0);
	}
	
	match parse_log_level(&args.flag_log_level) {
		Some(log_level) => {
			anyorigin::simple_log::init(log_level);
			anyorigin::start(&args.flag_address);
		},
		None => {
			println!("unable to parse log level {}, one of error, warn, info, debug, trace expected", args.flag_log_level);			
		}
	}
}
