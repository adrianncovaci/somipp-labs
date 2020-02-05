use std::{thread, time};
use crossterm::{
    ExecutableCommand,
};
use std::io::stdout;

pub fn print_os_load() {
    print!("\x1B[2J");
    stdout().execute(crossterm::cursor::MoveTo(0, 0)).expect("something went wrong");
    println!("Power On Self Test... DONE");
    thread::sleep(time::Duration::from_secs(1));
    println!("Accessing, loading, and running the operating system...");
    thread::sleep(time::Duration::from_secs(1));
    println!("Starting up daemons");
    thread::sleep(time::Duration::from_secs(1));
    println!("                           ______");
    println!("        |\\_______________ (_____\\______________");
    println!("HH======#H###############H#######################");
    println!("        '\\ ~\"\"\"\"\"\"\"\"\"\"\"\"\"\"`##(_))#H\\\"\"\"\"\"Y########");
    println!("                          ))    \\#H\\       `\"Y###");
    println!("                          \"      }}#H");
    println!("");
    thread::sleep(time::Duration::from_secs(1));
}
