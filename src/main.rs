extern crate getopts;

use getopts::Options;
use std::env;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("c", "category", "Wallpaper category", "CATEGORY");
    opts.optopt("i", "image", "Use image as current wallpaper", "IMAGE");

    opts.optflag("f", "flush-cache", "Flush wallpaper cache");
    opts.optflag("d", "dump-cache", "Dump cache to STDOUT");
    opts.optflag("l", "lock", "Lock the current wallpaper");
    opts.optflag("u", "unlock", "Unlock the current wallpaper");
    opts.optflag("", "clear", "Clear previous wallpaper category");
    opts.optflag("p", "previous", "Set wallpaper to the previous paper");
    opts.optflag("h", "help", "Print usage info");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            panic!("{}", f.to_string())
        }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    if matches.opt_present("c") {
        let category = matches.opt_str("c");
        match category {
            Some(x) => println!("{}", x),
            None => panic!("what now?"),
        }
        return;
    }

    if matches.opt_present("i") {
        let image = matches.opt_str("i");
        match image {
            Some(x) => println!("{}", x),
            None => panic!("where it go?"),
        }
    }
}
