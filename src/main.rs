use std::fs::{File, OpenOptions};
use std::io;
use std::io::prelude::*;
use std::os::unix::prelude::FileExt;
use std::path::Path;
use std::{env, fs, process};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if &args.is_empty() == &true {
        println!("To perish a file provide a valid file path");
        process::exit(1);
    };

    let path = Path::new(args[1].as_str());
    let path_full = fs::canonicalize(&path).unwrap();

    println!("Perishing file {:?}", &path_full);

    let fo = File::open(&path).unwrap();
    let f_size = fo.metadata()?.len();
    let cut_point = f_size / 2;

    let f = OpenOptions::new().read(true).write(true).truncate(false).open(&path)?;

    // erase the 2nd half of the file
    f.set_len(cut_point)?;
    // add empty 0s in place of 2nd half
    f.set_len(f_size)?;

    f.sync_all()?;
   
    println!("file has now perished");

    Ok(())
}
