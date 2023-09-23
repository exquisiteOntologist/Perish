use std::fs::OpenOptions;
use std::io;
use std::path::Path;
use std::{env, process};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if &args.len() < &2 {
        println!("To perish a file provide a valid file path");
        process::exit(1);
    };

    let path = Path::new(args[1].as_str());

    println!("Perishing file {:?}", path.to_str().unwrap());

    let f = OpenOptions::new().read(true).write(true).truncate(false).open(&path)?;
    let f_size = f.metadata()?.len();

    // erase the 2nd half of the file
    f.set_len(f_size / 2)?;
    // add empty 0s in place of 2nd half
    f.set_len(f_size)?;
   
    println!("file has now perished");

    Ok(())
}
