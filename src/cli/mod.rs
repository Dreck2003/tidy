mod constants;

use crate::utils::{read_proyects, write_to_proyects_db};

use super::cli::constants::{ADD, FIRST_LEVEL_ARGS, HELP, HELP_SHORT, LIST, NEW, OPEN};
use super::colors::utils::Color;
use super::messages::principal::PRINCIPAL;
use std::fs;
use std::io::{self, stdout, Write};
use std::{
    env,
    fmt::Debug,
    process::{self, Command},
};

pub struct CommandArg {
    query: String,
    type_query: TypeQuery,
}

impl Debug for CommandArg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CommandArg")
            .field("query", &self.query)
            .field("type_query", &self.type_query)
            .finish()
    }
}

impl CommandArg {
    pub fn new(query: &str, type_query: TypeQuery) -> Self {
        Self {
            query: query.to_string(),
            type_query,
        }
    }
}

#[derive(Debug)]
pub enum TypeQuery {
    Flag,
    Text,
}

#[derive(Debug)]
pub enum CommandError {
    EmptyCommands,
}

pub fn read_commands() -> Result<Vec<CommandArg>, CommandError> {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        return Err(CommandError::EmptyCommands);
    }
    let params = &args[1..];
    let mut command_args = vec![];

    for param in params {
        let type_query = if let Some(text) = param.get(0..1) {
            if text == "-" {
                TypeQuery::Flag
            } else {
                TypeQuery::Text
            }
        } else {
            TypeQuery::Text
        };
        command_args.push(CommandArg::new(&param, type_query));
    }
    Ok(command_args)
}

fn verify_first_arg(first_query: &CommandArg) -> bool {
    FIRST_LEVEL_ARGS.contains(&first_query.query.as_str())
}

pub fn run() {
    let commands = read_commands();
    // print!("\x1B[2J\x1B[1;1H");
    let _ = stdout().flush();
    if let Err(cmd) = commands {
        let text = match cmd {
            CommandError::EmptyCommands => PRINCIPAL,
        };
        eprintln!("{}", text);
        process::exit(0)
    }
    let commands = commands.unwrap();
    let first_query = &commands[0];

    if !verify_first_arg(first_query) {
        let msg = Color::red(&first_query.query).build();
        eprintln!(
            "The {} not is a command or option valid option\n {}",
            msg, PRINCIPAL
        );
        process::exit(0)
    }
    clear_screen();
    match first_query.query.as_str() {
        LIST => list_proyects(),
        NEW => {
            Color::green("?").print();
            Color::white(" Name of proyect: ").bold().print();
            stdout().flush().unwrap();
            new_proyect();
        }
        ADD => {
            add_proyect();
        }
        OPEN => select_proyect(),
        HELP | HELP_SHORT => {
            eprintln!("{}", PRINCIPAL);
            process::exit(0);
        }
        _ => {
            println!("Not exist this command.")
        }
    }
}

fn new_proyect() {
    let stdin = io::stdin();
    let mut input = "".to_string();

    if let Err(e) = stdin.read_line(&mut input) {
        eprintln!("{}", e.to_string());
        process::exit(1)
    }
    input = input.trim_end().trim().to_string();

    let res = env::current_dir();
    if let Err(e) = &res {
        eprintln!("{}", e.to_string());
        process::exit(1)
    }
    let mut path_to_new_folder = res.unwrap();
    path_to_new_folder.push(input);

    let path_to_save = path_to_new_folder.to_str();
    if let None = path_to_save {
        eprintln!("Ocurred a problem getting the path.");
        process::exit(1)
    }

    let binding = read_proyects();
    let content: Vec<&str> = binding.lines().collect();
    if content.contains(&path_to_save.unwrap()) {
        eprintln!("The folder already exist!. Choose other name.");
        process::exit(1)
    }

    if let Err(e) = fs::create_dir(path_to_new_folder.clone()) {
        eprintln!("{}", e.to_string());
        process::exit(1)
    }

    Color::green(&format!("{}", path_to_save.unwrap())).println();

    write_to_proyects_db(path_to_save.unwrap());
}

fn list_proyects() {
    let content = read_proyects();
    let proyects: Vec<&str> = content.lines().collect();
    if proyects.len() <= 0 {
        println!("No proyects!");
        process::exit(0);
    }

    let joined = join_proyect_to_print(&proyects, |_| "-".to_string());
    println!("List of proyects: ");
    println!("{}", joined);
}

fn add_proyect() {
    let res = env::current_dir();
    if let Err(e) = res {
        eprintln!("{}", e.to_string());
        process::exit(1)
    }
    let binding = res.unwrap();
    let path = binding.to_str();

    if let None = &path {
        eprintln!("Cannot parse the current folder.");
        process::exit(1)
    }

    let binding = read_proyects();
    let content: Vec<&str> = binding.lines().collect();
    if content.contains(&path.unwrap()) {
        eprintln!("The folder already exist!. Choose other name.");
        process::exit(1)
    }
    write_to_proyects_db(path.unwrap());
    Color::yellow(path.unwrap()).println();
}

fn select_proyect() {
    let content = read_proyects();
    let proyects: Vec<&str> = content.lines().collect();

    if proyects.len() <= 0 {
        println!("Has no proyect");
        process::exit(0);
    }

    Color::green("?").print();
    Color::white(" Choose the proyect: ").println();
    let joined = join_proyect_to_print(&proyects, |c| c.to_string() + ". ");

    println!("{}", joined); //Print all proyects
    let _ = stdout().flush();

    let mut line = "".to_string();
    let _ = io::stdin().read_line(&mut line);

    let index = line.trim().parse::<i32>();

    if let Err(e) = index {
        eprintln!("{}", e.to_string());
        process::exit(1)
    }
    let index = index.unwrap();
    if index > proyects.len() as i32 || index <= 0 {
        eprintln!("That proyect does not exist with {} index", index);
        process::exit(1)
    }
    select_proyect_and_open(proyects[(index - 1) as usize]);
    let res = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/c", "cd", proyects[(index - 1) as usize]])
            .output()
    } else {
        Command::new("sh")
            .args(["-c", "cd ", proyects[(index - 1) as usize]])
            .output()
    };
    if let Err(e) = res {
        eprintln!("{}", e.to_string());
    }
    Color::green(&format!("{}", proyects[(index - 1) as usize])).println();
}

fn select_proyect_and_open(proyect: &str) {
    let res = if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/c", "code ", proyect]).output()
    } else {
        Command::new("sh").args(["-c", "code ", proyect]).output()
    };
    if let Err(e) = res {
        eprintln!("{}", e.to_string());
        process::exit(1)
    }
}

fn join_proyect_to_print<T>(proyects: &[&str], f: T) -> String
where
    T: Fn(i32) -> String,
{
    let mut count = 0;
    return proyects.iter().fold("".to_string(), |init, s| {
        count += 1;
        if count == 1 {
            format!("{init}{}{s}", f(count))
        } else {
            return format!("{init}\n{}{s}", f(count));
        }
    });
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    println!("{}", env::current_dir().unwrap().to_str().unwrap());
}
