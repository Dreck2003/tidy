use std::{
    env,
    fs::{self, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
    process, writeln,
};

pub fn get_exe_path() -> PathBuf {
    let res = env::current_exe();
    if let Err(e) = &res {
        eprintln!("{}", e.to_string());
        process::exit(1);
    }
    let mut binding = res.unwrap();
    binding.pop();
    binding.push("proyects.txt");
    return binding;
}

pub fn write_to_proyects_db(line: &str) {
    if Path::new(&get_exe_path()).exists() {
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
    let res = fs::File::create(get_exe_path());

    if let Err(e) = res {
        return Err(e.to_string());
    }

    return Ok(());
}

pub fn read_proyects() -> String {
    if Path::new(&get_exe_path()).exists() {
        match fs::read_to_string(get_exe_path()) {
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
        .open(get_exe_path());

    if let Err(e) = res {
        return Err(e.to_string());
    }
    let mut file = res.unwrap();

    if let Err(e) = writeln!(file, "{}", text) {
        return Err(e.to_string());
    }
    return Ok(());
}
