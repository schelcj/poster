extern crate getopts;

use getopts::Options;
use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use walkdir::WalkDir;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn set_category(category: &str) -> std::io::Result<()> {
    let mut file = File::create("/home/schelcj/.wallpapers/category")?;
    file.write_all(category.as_bytes())?;

    Ok(())
}

fn clear_category() -> std::io::Result<()> {
    if !Path::new("/home/schelcj/.wallpapers/category").exists() {
        fs::remove_file("/home/schelcj/.wallpapers/category")?;
    }

    Ok(())
}

fn find_posters() -> std::io::Result<()> {
    let walker = WalkDir::new("/home/schelcj/.wallpapers/Wallpapers").into_iter();

    for entry in walker
        .filter_map(|e| e.ok())
        .filter(|e| !e.file_type().is_dir())
    {
        println!("{}", entry.path().display());
    }

    Ok(())
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
        // TODO: [2023-05-02 schelcj] - write category to file
        // TODO: [2023-05-02 schelcj] - set poster to one from this category/directory

        let category = matches.opt_str("c");
        match category {
            Some(x) => {
                set_category(&x).expect("failed to set category");
            }
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

    if matches.opt_present("f") {
        // TODO: [2023-05-02 schelcj] - flush whatever cache there is
    }

    if matches.opt_present("d") {
        // TODO: [2023-05-02 schelcj] - dump the cache to STDOUT
    }

    if matches.opt_present("l") {
        // TODO: [2023-05-02 schelcj] - set a lock file
    }

    if matches.opt_present("u") {
        // TODO: [2023-05-02 schelcj] - remove lock file
    }

    if matches.opt_present("clear") {
        clear_category().expect("failed to clear category");
    }

    if matches.opt_present("p") {
        // TODO: [2023-05-02 schelcj] - set poster to previous image from previous file
    }

    find_posters().expect("failed to scan wallpapers directory");
}
