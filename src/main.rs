use std::fs;
use colored::Colorize;

fn main(){
    send_base_help_msg();
    operate();
}

fn operate(){
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Did not enter a correct string");
    let cmd = input.trim();
    let args: Vec<&str> = cmd.split_whitespace().collect();
    if args[0].to_lowercase() == "help" {
        process_help(&args);
        operate();
    }else if cmd.to_lowercase() == "stop" {
        println!("{}", "Bye!".green());
    }else {
        println!("Invalid Input! You typed {}, but options are only 'Go' and 'Stop'", cmd.red());
        operate();
    }
}

fn process_help(args: &[&str]) {
    if args.len() == 1 {
        send_base_help_msg();
    }else if args.len() == 2 {
        if args[1].to_lowercase() == "db" {
            send_db_help_msg();
        }
    }else if args.len() == 3 {
        if args[1].to_lowercase() == "db" {
            if args[2].to_lowercase() == "select" {
                send_db_select_help_msg();
            }
        }
    }
}

fn send_base_help_msg() {
    println!("DBMS {} help", "command".cyan());
    println!("{} - used to provide help for a given command", "help".yellow());
    println!("{} - used to interact with a database and it's properties", "db".yellow());
    println!("{} - used to search for an entry in a selected database", "get".yellow());
    println!("{} - used to input a new entry into a selected database", "put".yellow());
}

fn send_db_help_msg() {
    println!("DBMS {} help", "db".cyan());
    println!("db {} - used to create a new database", "create".yellow());
    println!("db {} - used to delete an existing database", "destroy".yellow());
    println!("db {} - used to select a database for future queries", "select <name>".yellow());
}

fn send_db_select_help_msg() {
    println!("DBMS {} help", "db select".cyan());
    println!("db select {} - the name of the parent folder containing generated db files", "<name-of-database>".yellow());
}

//This fn is here for future use
fn _create_folder(path: &str) { 
    match fs::create_dir(path) {
        Ok(_) => println!("Directory created successfully"),
        Err(e) => println!("Error creating directory: {:?}", e),
    }
}
