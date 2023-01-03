use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::Path,
    writeln,
};

pub fn write_to_proyects_db(line: &str) {
    if Path::new("proyects.txt").exists() {
        if let Err(e) = add_proyect_path(line) {
            eprintln!("{}", e);
        }
    } else {
        let r = create_proyects_db();
        if let Err(e) = r {
            eprintln!("{}", e);
        }
        if let Err(e) = add_proyect_path(line) {
            eprintln!("{}", e);
        }
    }
}

fn create_proyects_db() -> Result<(), String> {
    let res = fs::File::create("proyects.txt");

    if let Err(e) = res {
        return Err(e.to_string());
    }

    return Ok(());
}

pub fn read_proyects() -> String {
    if Path::new("proyects.txt").exists() {
        match fs::read_to_string("proyects.txt") {
            Ok(s) => return s,
            Err(_) => "".to_string(),
        }
    } else {
        let r = create_proyects_db();
        if let Err(e) = r {
            eprintln!("{}", e);
        }
        return "".to_string();
    }
}

fn add_proyect_path(text: &str) -> Result<(), String> {
    let res = OpenOptions::new()
        .write(true)
        .append(true)
        .open("proyects.txt");

    if let Err(e) = res {
        return Err(e.to_string());
    }
    let mut file = res.unwrap();

    if let Err(e) = writeln!(file, "{}", text) {
        return Err(e.to_string());
    }
    return Ok(());
}
