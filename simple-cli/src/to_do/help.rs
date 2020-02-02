pub fn help() {
    println!();
    println!("Available commands: ");
    println!();
    //todo command
    println!("to-do module helps you manage to-do notes");
    println!("{:<4}todo get -> lists the created notes", "");
    println!("{:<4}todo add {{content}} -> creates a new to do note", "");
    println!("{:<4}todo complete {{index}} -> completes a note at the provided integer index", "");
    println!("{:<4}todo delete {{index}} -> deletes a note at the provided integer index", "");
    println!();
    //clr
    println!("clr -> clears the screen");
    println!();
    //touch
    println!("touch -> creates a new emtpy file");
    println!("{:<4}touch file.txt -> creates a an empty .txt file", "");
    println!();
    //ls
    println!("ls -> lists files from the directory");
    println!();
    //help
    println!("help -> lists the available commands.");
    println!();
    //quit
    println!("quit -> quits the operating system");
    println!();
}
