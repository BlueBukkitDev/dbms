//This file will contain structs and impls for any operation needed. //zettlekasten query language, ZQL

use std::env;
use std::path::Path;
use std::path::PathBuf;
use std::fs;
use crate::help_msg::*;
use crate::cmd_db::*;

//const tags: &[u8] = include_bytes!("tags.yml");//Need to be able to write existing files into the program folder on startup; save as arrays of bytes, compile, when executing, will
//write to the file at the desired destination. EZPZ. 

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
        setup_program_files();
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
    if cmd == "stop" || cmd == "quit" || cmd == "close" {
        return true;
    }else if cmd == "help" {
        process_help(&args);
    }else if cmd == "db" {
        process_db(&args);
    }else if cmd == "find" {

    }else if cmd == "read" {

    }else if cmd == "get" {
        
    }else if cmd == "write" {
        if args.len() == 1 {
            //create_folder(&args[0].to_string());
            
        }else {
            setup_std_db("Rando", "C:\\Users\\be\\Desktop")
            //send wrong args msg
        }
    }else if cmd == "put" {
        
    }else {
        let options = ["stop", "help", "db", "find", "read", "get", "write", "put"];
        send_unknown_cmd(cmd, args, &options);
    }

    return false;
}

fn get_root() -> Result<PathBuf, std::io::Error> {
    let current_dir = env::current_dir()?;
    Ok(current_dir.ancestors().last().unwrap_or_else(|| Path::new("/")).to_path_buf())
}

fn setup_program_files() {
    if program_is_setup() {
        return;
    }
    //if not, create a bunch of files with configs and whatnot. 
    let root = get_program_folder_path();
    create_folder(&root);

}

fn program_is_setup () -> bool {
    Path::new(&get_program_folder_path()).exists()
}

fn get_program_folder_path () -> String {
    let root = get_root().expect("Error finding root folder");
    let path = root.join("EZQL");
    let folder = path.to_str().expect("Failed to convert path to string");
    folder.to_string()
}

fn setup_std_db(name:&str, path:&str) {
    let db_path = Path::new(path).join(name);
    let db_folder = db_path.to_str().expect("Failed to convert path to string");
    create_folder(&db_folder);
    create_folder(&format!("{}\\Person", db_folder));
    create_folder(&format!("{}\\Location", db_folder));
    create_folder(&format!("{}\\Concept", db_folder));
    create_folder(&format!("{}\\Technology", db_folder));
    create_folder(&format!("{}\\Event", db_folder));
    create_folder(&format!("{}\\Creature", db_folder));
    create_folder(&format!("{}\\Object", db_folder));
    create_folder(&format!("{}\\Structure", db_folder));
    println!("Database has been created.");
}

//This fn is here for future use
fn create_folder(path: &str) {
    match fs::create_dir(path) {
        Ok(_) => return,
        Err(e) => println!("Error creating directory: {:?}", e),
    }
}
