use std::io::{self, BufRead};
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use qt_widgets::{self,
                QFileDialog,
                qt_core::QString,
                cpp_core::CppBox,
                QApplication};


enum Error {
    NotExist (String),
    Create (String),
}

fn read_filename() -> io::Result<CppBox<QString>> {
    unsafe{
        let filename =
            QFileDialog::get_open_file_name_0a();
        Ok(filename)
    }
}

fn press_key_to_continue() -> io::Result<()> {
    let mut key : String = String::new();
    println!("Premere Enter per chiudere il programma");
    io::stdin().read_line(&mut key)?;
    Ok(())
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
    QApplication::init(|_app| {

    print_message();
    loop {
        let filename = read_filename();
        match filename {
            Ok(a) => {
                unsafe {
                    if a.is_null() {
                        println!("Nessun File Selezionato");
                        press_key_to_continue();
                        std::process::exit(0)
                    }
                };

                let name = a.to_std_string();

                match process_file(name) {
                    Ok(_) => {
                        press_key_to_continue();
                        std::process::exit(0)
                    }
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
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fid() {
        let res = process_file(String::from("src/tests/test.fid"));
        match res {
            Ok(()) => (),
            Err(_) => assert!(false)
        }

        let file = match OpenOptions::new().read(true).open("src/tests/test.fid.out") {
            Ok(file) => file,
            Err(_) => return assert!(false)
        };

        let result : String  = String::from("N1 CENTRI VITI M20 FIX CASTELLO");
        let mut reader = io::BufReader::new(file);
        let mut line : String = String::new();
        let bytes_read = match reader.read_line(&mut line) {
            Ok(size) => size,
            Err(_) => return assert!(false)
        };

        assert_eq!(bytes_read-2, result.trim().len());

        assert_eq!(result.trim(), line.trim());
    }

    #[test]
    fn test_others() {
        let res = process_file(String::from("src/tests/test.txt"));
        match res {
            Ok(()) => (),
            Err(_) => assert!(false)
        }

        let file = match OpenOptions::new().read(true).open("src/tests/test.txt.out") {
            Ok(file) => file,
            Err(_) => return assert!(false)
        };

        let result : String  = String::from("N5 CENTRI VITI M20 FIX CASTELLO");
        let mut reader = io::BufReader::new(file);
        let mut line : String = String::new();
        let bytes_read = match reader.read_line(&mut line) {
            Ok(size) => size,
            Err(_) => return assert!(false)
        };

        assert_eq!(bytes_read-2, result.trim().len());

        assert_eq!(result.trim(), line.trim());
    }

    #[test]
    fn test_empty() {
        let res = process_file(String::from("src/tests/empty"));
        match res {
            Ok(()) => (),
            Err(_) => assert!(false)
        }

        let file = match OpenOptions::new().read(true).open("src/tests/empty.out") {
            Ok(file) => file,
            Err(_) => return assert!(false)
        };

        let result : String  = String::from("");
        let mut reader = io::BufReader::new(file);
        let mut line : String = String::new();
        let bytes_read = match reader.read_line(&mut line) {
            Ok(size) => size,
            Err(_) => return assert!(false)
        };

        assert_eq!(bytes_read, result.trim().len());

        assert_eq!(result.trim(), line.trim());
    }

    #[test]
    fn test_nonexisting() {
        let res = process_file(String::from("src/tests/something"));
        match res {
            Ok(()) => (),
            Err(e) => match e {
                Error::NotExist(e) => assert_eq!(e, String::from("src/tests/something")),
                Error::Create(e) => assert_eq!(e, String::from("src/tests/something"))
            }
        }
    }

    #[test]
    fn test_i() {
        let res = process_file(String::from("src/tests/test.i"));
        match res {
            Ok(()) => (),
            Err(_) => assert!(false)
        }

        let file = match OpenOptions::new().read(true).open("src/tests/test.i.out") {
            Ok(file) => file,
            Err(_) => return assert!(false)
        };

        let result : String  = String::from("CENTRI VITI M20 FIX CASTELLO");
        let mut reader = io::BufReader::new(file);
        let mut line : String = String::new();
        let bytes_read = match reader.read_line(&mut line) {
            Ok(size) => size,
            Err(_) => return assert!(false)
        };

        assert_eq!(bytes_read-2, result.trim().len());

        assert_eq!(result.trim(), line.trim());
    }

}
