use std::io::{self, BufRead};
use std::fs::{File, OpenOptions};
use std::io::prelude::*;

enum Error {
    NotExist (String),
    Create (String),
}

fn read_filename() -> io::Result<String> {
    let mut filename = String::new();
    println!("Inserire il nome del file:");
    io::stdin().read_line(&mut filename)?;
    Ok(filename)
}

fn process_file(filename: String) -> Result<(), Error> {
    let comment_start : Vec<&str> = vec!["[ ", "* - ", "("];
    let comment_end : Vec<&str> = vec![" ]", "* _ ", ")"];

    let filename = filename.trim();

    let input_file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => return Err(Error::NotExist(String::from(filename)))
    };

    let output_filename = format!("{}.out", filename);

    let output_filename = output_filename.trim();

    let mut output_file : File = match OpenOptions::new()
                                        .create(true)
                                        .write(true)
                                        .truncate(true)
                                        .append(false)
                                        .open(output_filename) {
        Ok(file) => file,
        Err(_) => return Err(Error::Create(String::from(output_filename)))
    };

    let mut reader = io::BufReader::new(input_file);
    let mut line = String::new();

    println!("\n");

    loop {
        line.clear();

        let bytes_read = reader.read_line(&mut line)
            .expect("There is always a line to read");

        if bytes_read == 0 {
            break;
        }

        for comment in &comment_start {
            if !(line.contains(comment)) {
                continue;
            }
        }

        if line.contains("CENTRI ")
            || line.contains("PROFILI ") {
            let mut output_line = line.clone();
            for comment in &comment_start {
                output_line = output_line.replace(comment, "");
            }
            for comment in &comment_end {
                output_line = output_line.replace(comment, "");
            }
            print!("{}", output_line);
            output_file.write_all(output_line.as_bytes())
                .expect("Always able to append to the file");
        }
    }
    Ok(())
}

fn print_message() {
    println!("\n\n########## mambretti.exe v2.0 ##############\n");
    println!("\tAuthor: Andrea Mambretti");
    println!("\tEmail: m4mbr3@gmail.com\n");
    println!("###########################################\n\n")
}

fn main() {
    print_message();
    loop {
        let filename = read_filename();
        match filename {
            Ok(name) => {
                match process_file(name) {
                    Ok(_) => std::process::exit(0),
                    Err(e) => match e {
                        Error::NotExist(name) => println!("Impossibile aprire il file {}!\n", name),
                        Error::Create(name) => println!("Impossibile creare il file in uscita {}\n", name)
                    }
                }
            }
            Err(_) => {
                println!("Error reading filename");
                std::process::exit(0)
            }
        };
    }
}
