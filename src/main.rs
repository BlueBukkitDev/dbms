use std::fs;
use colored::Colorize;
use crate::help_msg::*;

mod help_msg;

fn main(){
    send_base_help_msg();
    operate();
}

fn operate(){
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Did not enter a correct string");
    let pieces: Vec<&str> = input.trim().split_whitespace().collect();
    if process_command(pieces[0], &pieces[1..]) {
        return;
    }
    operate();
}

/**
 Takes in a command and an array of subcommands. Redirects to other functions depending on the series. 

 Returns true if the program should close. 
 */
fn process_command(cmd: &str, args: &[&str]) -> bool {
    if cmd == "stop" || cmd == "end" || cmd == "quit" || cmd == "close" || cmd == "die" {
        return true;
    }else if cmd == "help" {
        process_help(&args);
    }else if cmd == "db" {

    }else if cmd == "find" {

    }else if cmd == "read" {

    }else if cmd == "get" {
        
    }else if cmd == "write" {
        
    }else if cmd == "put" {
        
    }else {
        send_unknown_cmd(cmd);
    }

    return false;
}

fn send_unknown_cmd(cmd: &str) {
    println!("{} you wrote {}", "Unknown command!".red(), cmd.truecolor(241, 194, 50));
}

/**
 Processes help commands based on user input. 
 */
fn process_help(args: &[&str]) {
    if args.len() == 0 {
        send_base_help_msg();
    }else if args.len() == 1 {
        if args[0].to_lowercase() == "db" {
            send_db_help_msg();
        }
    }else if args.len() == 2 {
        if args[0].to_lowercase() == "db" {
            if args[1].to_lowercase() == "select" {
                send_db_select_help_msg();
            }
        }
    }
}

//This fn is here for future use
fn _create_folder(path: &str) { 
    match fs::create_dir(path) {
        Ok(_) => println!("Directory created successfully"),
        Err(e) => println!("Error creating directory: {:?}", e),
    }
}
