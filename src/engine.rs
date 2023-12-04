use std::env;
use std::path::Path;
use std::path::PathBuf;
use std::fs;
use crate::help_msg::*;
use crate::cmd_db::*;

//const tags: &[u8] = include_bytes!("tags.yml");//Need to be able to write existing files into the program folder on startup; save as arrays of bytes, compile, when executing, will
//write to the file at the desired destination. EZPZ. 

/**
 Takes in a command and an array of subcommands. Redirects to other functions depending on the series. 

 Returns true if the program should close. 
 */
pub fn process_command(cmd: &str, args: &[&str]) -> bool {
    match cmd {
        "exit" => return true,
        "help" => {process_help(&args); return false},
        "db" => {process_db(&args); return false},
        "find" => return false,
        "read" => return false,
        "get" => return false,
        "write" => return false,
        "put" => return false,
        _ => {
            let options = ["exit", "help", "db", "find", "read", "get", "write", "put"];
            send_unknown_cmd(cmd, args, &options); 
            return false},
    }
    /*else if cmd == "find" {

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

    return false;*/
}

fn get_root() -> Result<PathBuf, std::io::Error> {
    let current_dir = env::current_dir()?;
    Ok(current_dir.ancestors().last().unwrap_or_else(|| Path::new("/")).to_path_buf())
}

pub fn setup_program_files() {
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