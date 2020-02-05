mod to_do;
use std::io;
use std::io::{Write, stdout};
use crate::to_do::{note, loados, help};
use crossterm::{
    ExecutableCommand,
};
use std::fs::{File, read_dir};
use std::env;
use std::path::Path;
use std::{thread, time};

enum Functions {
    Quit,
    NotesCmd(note::Command),
    Clear,
    Help,
    Touch(String),
    Ls,
}

fn main() {
    loados::print_os_load();
    let mut pass = String::new();
    loop {
        println!("Please log in as root:");
        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut pass).expect("Something went wrong");
        pass = pass.trim().to_string();
        match pass.as_str() {
            "admin" => { println!("Welcome, root!"); break; },
            "quit" => {return;}
            _ => { println!("Wrong, please try again!"); pass = String::new(); }
        }
    }
    let mut to_do_list = note::ToDoList::new();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut inp = String::new();
        io::stdin().read_line(&mut inp).expect("something went wrong");
        let args: Vec<&str>;
        if inp.contains("todo") {
            args = inp.trim().splitn(3, ' ').collect();
        } else {
            args = inp.trim().splitn(2, ' ').collect();
        }
        let command = match args[0]{
            "todo" => {
                match args[1] {
                    "get" => Functions::NotesCmd(note::Command::Get),
                    "add" => {
                        if args.len() < 3 {
                            help::help();
                            continue;
                        } else {
                        Functions::NotesCmd(note::Command::Add(args[2].to_string()))
                        }
                    },
                    "delete" => {
                        if args.len() < 3 {
                            help::help();
                            continue;
                        } else {
                        Functions::NotesCmd(note::Command::Delete(args[2].parse().expect("Please provide an integer index")))
                        }
                    },
                    "complete" => {
                        if args.len() < 3 {
                            help::help();
                            continue;
                        } else {
                        Functions::NotesCmd(note::Command::Complete(args[2].parse().expect("Please provide an integer index")))
                        }
                    },
                    _ => panic!("You need to provide an available command, use /help to get a list of commands"),
                }
            },
            "quit" => Functions::Quit,
            "clr" => Functions::Clear,
            "touch" => Functions::Touch(args[1].to_string()),
            "ls" => Functions::Ls,
            "help" => Functions::Help,
            _ => { println!("You need to provide an available command"); Functions::Help },
        };

        match command {
            Functions::NotesCmd(note::Command::Get) => to_do_list.display_list(),
            Functions::NotesCmd(note::Command::Add(task)) => {
                to_do_list.create_to_do(task);
                to_do_list.display_list();
            },
            Functions::NotesCmd(note::Command::Delete(index)) => {
                to_do_list.delete_to_do(index);
                to_do_list.display_list();
            },
            Functions::NotesCmd(note::Command::Complete(index)) => {
                to_do_list.complete_to_do(index);
                to_do_list.display_list();
            },
            Functions::Clear => {
                print!("\x1B[2J");
                stdout().execute(crossterm::cursor::MoveTo(0, 0)).expect("something went wrong");
            },
            Functions::Touch(task) => {
                let file_dir = Path::new("files/");
                env::set_current_dir(&file_dir).expect("There is no files folder");
                File::create(task).expect("Something went wrong creating the file");
                env::set_current_dir(Path::new("../")).expect("There is no such folder");
            },
            Functions::Ls => {
                let paths = read_dir("files/").unwrap();
                for path in paths {
                    println!("{}", path.unwrap().path().display());
                }
            },
            Functions::Help => {
                help::help();
            },
            Functions::Quit => {
                println!("Goodbye!");
                thread::sleep(time::Duration::from_secs(1));
                break;
            }
        }
    }
}
