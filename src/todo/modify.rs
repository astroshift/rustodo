use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::prelude::*;
use std::io::ErrorKind;

pub fn add_entry(_path: String, _item: &str, _date: String) {
    let dir_s = _path.split("/");
    let dir_v: Vec<&str> = dir_s.collect();    
    let mut dir = String::new();

    let mut x = 0;
    while x < dir_v.len() - 1 {
        dir.push_str(dir_v[x]);

        if x < dir_v.len() - 2 {
            dir.push('/');
        }

        x += 1;
    }

    let _cdir = fs::create_dir_all(dir);
    let mut _cdir = match _cdir {
        Ok(_cdir) => _cdir,
        Err(e) => match e.kind() {
            ErrorKind::AlreadyExists => print!(""),
            other_error => panic!("There was a problem creating the directory: {:?}", other_error),
        },
    };

    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&_path);

    let mut file = match file {
        Ok(file) => file,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => match File::create(&_path) {
                Ok(fc) => fc,
                Err(error) => panic!("Tried to create file but there was a problem: {:?}", error),
            },
            other_error => panic!("There was a problem opening/creating the file: {:?}", other_error),
        },
    };

    let mut line = String::new();
    line.push_str("(_): ");
    line.push_str(&_item);
    line.push_str(" (");
    line.push_str(&_date);
    line.push_str(")");

    if let Err(e) = writeln!(file, "{}", line) {
        eprintln!("Couldn't write to file {}", e);
    }
}

pub fn remove_entry(_path: String, _item: &str) {
    let _lines = fs::read_to_string(&_path)
        .expect("Something went wrong reading the file");

    let split = _lines.split("\n");

    let vec: Vec<&str> = split.collect();
    let mut _nvec: Vec<String> = Vec::new();

    for i in vec.iter() {
        if !i.contains(_item) {
            _nvec.push(i.to_string());
        }
        else {
            println!("Deleting: {}", i);
        }
    }

    if vec.len() == 2 {
        File::create(&_path)
            .expect("Error creating file");
    }
    else if vec.len() > 2 {
        let mut f = File::create(&_path)
            .expect("Error creating file");

        let mut j = 0;
        while j < _nvec.len() {
            if j < _nvec.len() - 1 {
                _nvec[j].push_str("\n");
            }

            f.write_all(_nvec[j].as_bytes())
                .expect("Unable to write data");
            
            j += 1;
        }
    }
}

pub fn ufin_entry(_path: String, _item: &str, foru: bool) {
    let _lines = fs::read_to_string(&_path)
        .expect("Something went wrong reading the file");   
    
    let _file = OpenOptions::new()
        .write(true)
        .open(&_path);

    let mut _file = match _file {
        Ok(file) => file,
        Err(e) => panic!("Error opening file: {}", e),    
    };

    let split = _lines.split("\n");

    let vec: Vec<&str> = split.collect();
    let mut _nvec: Vec<String> = Vec::new();

    let mut i = 0;
    if !foru {
        while i < vec.len() {
            if vec[i].contains(_item) {
                _nvec.push(vec[i].replace("x", "_").to_string());
            }
            else {
                _nvec.push(vec[i].to_string());
            }

            i += 1;
        }
    }
    else { 
        while i < vec.len() {
            if vec[i].contains(_item) {
                _nvec.push(vec[i].replace("_", "x").to_string());
            }
            else {
                _nvec.push(vec[i].to_string());
            }

            i += 1;
        }
    }

    let mut j = 0;
    while j < _nvec.len() {
        if j < _nvec.len() - 1 {
            writeln!(_file, "{}", _nvec[j]);
        }
        else {
            write!(_file, "{}", _nvec[j]);
        }

        j += 1;
    }
}

pub fn clear_entrys(_path: String) {
    print!("Are you sure you want to clear your todo list? (y/n): ");
    flushio();

    let mut yn = String::new();

    loop {
        io::stdin().read_line(&mut yn)
            .expect("Failed to read line");

        if yn.contains("Y") || yn.contains("y") {    
            File::create(&_path)
                .expect("Couldn't clear file");
            println!("File cleared");
            break;
        }
        else if yn.contains("N") || yn.contains("n") {
            println!("File clear cancelled");
            break;
        }
        else {
            print!("Error (enter y or n): ");
            flushio();
        }
    }
}

fn flushio() {
    io::Write::flush(&mut io::stdout())
        .expect("Flush failed");
}
