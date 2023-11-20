//This file will contain structs and impls for any operation needed. //zettlekasten query language, ZQL

use std::fs;
use crate::help_msg::*;
use crate::cmd_db::*;

/**
 Session's purpose is to store relevant persistent data. For now this only stores the selected database, but may be expanded to 
 include previous operations, login data, and more. 
 */
pub struct Session {
    selected_db : String
}

impl Session {
    /**
     Creates a new session with a null database
     */
    pub fn new() -> Session {
        Session {
            selected_db : String::new()
        }
    }
    pub fn select_db(&mut self, name: &str) {
        self.selected_db = name.to_string();
    }
    pub fn get_selected_db(self) -> String {
        self.selected_db
    }
}

pub struct API {
    session: Session
}

impl API {
    /**
     */
    pub fn new(&self) -> API {
        API {
            session: Session::new()
        }
    }
    /**
    Takes in a command and an array of subcommands. Redirects to other functions depending on the series. 

    Returns true if the program should close. 
    */
    pub fn run_command(cmd: &str, args:&[&str]) -> bool {
        process_command(cmd, args)
    }

    pub fn select_db (&mut self, name: &str) {
        self.session.select_db(name)
    }
    pub fn create_db (&mut self, name: &str) {

    }
    pub fn create_db_at_path (&mut self, name: &str, path: String) {

    }
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
        process_db(&args);
    }else if cmd == "find" {

    }else if cmd == "read" {

    }else if cmd == "get" {
        
    }else if cmd == "write" {
        create_folder(args[0]);
    }else if cmd == "put" {
        
    }else {
        let options = ["stop", "help", "db", "find", "read", "get", "write", "put"];
        send_unknown_cmd(cmd, args, &options);
    }

    return false;
}

//This fn is here for future use
pub fn create_folder(path: &str) {
    match fs::create_dir(path) {
        Ok(_) => println!("Directory created successfully"),
        Err(e) => println!("Error creating directory: {:?}", e),
    }
}
