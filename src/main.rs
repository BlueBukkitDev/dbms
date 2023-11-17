use std::fs;
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
        let options = ["stop", "help", "db", "find", "read", "get", "write", "put"];
        send_unknown_cmd(cmd, args, &options);
    }

    return false;
}

//This fn is here for future use
fn _create_folder(path: &str) { 
    match fs::create_dir(path) {
        Ok(_) => println!("Directory created successfully"),
        Err(e) => println!("Error creating directory: {:?}", e),
    }
}
