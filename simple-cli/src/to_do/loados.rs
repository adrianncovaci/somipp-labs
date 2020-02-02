use std::{thread, time};

pub fn print_os_load() {
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
