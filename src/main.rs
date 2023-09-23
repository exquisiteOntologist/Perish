use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::{env, fs, process};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if &args.is_empty() == &true {
        println!("To perish a file provide a valid file path");
        process::exit(1);
    };

    let path = Path::new(args[1].as_str());

    println!("Perishing file: {:?}", fs::canonicalize(&path).unwrap());

    let mut f = File::open(&path)?;
    // let f_length = f.
    let mut buffer = [0; 10000];
    let mut file_done = false;
    while &file_done == &false {
        // read up to buffer [,x] bytes
        let n = f.read(&mut buffer[..])?;
        if n == 0 {
            file_done = true;
        }
        println!("The bytes: {:?}", &buffer[..n]);
    }
    Ok(())
}
