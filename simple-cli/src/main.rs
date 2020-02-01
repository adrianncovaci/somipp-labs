mod to_do;
use std::io;
use std::io::Write;
use crate::to_do::{note, loados};

enum Functions {
    Quit,
    NotesCmd(note::Command),
    Clear,
}

fn main() {
    loados::print_os_load();
    let mut pass = String::new();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut pass).expect("Something went wrong");
        pass = pass.trim().to_string();
        match pass.as_str() {
            "admin" => { println!("Welcome, root!"); break; },
            _ => println!("Wrong, please try again!"),
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
                    "add" => Functions::NotesCmd(note::Command::Add(args[2].to_string())),
                    "delete" => Functions::NotesCmd(note::Command::Delete(args[2].parse().expect("Please provide an integer index"))),
                    "complete" => Functions::NotesCmd(note::Command::Complete(args[2].parse().expect("Please provide an integer index"))),
                    _ => panic!("You need to provide an available command, use /help to get a list of commands"),
                }
            },
            "quit" => Functions::Quit,
            "clr" => Functions::Clear,
            _ => panic!("You need to provide an available command"),
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
                println!("{}[2J", 27 as char);
            },
            Functions::Quit => {
                break;
            }
        }
    }
}
