use colored::Colorize;

const Y: (u8, u8, u8) = (241, 194, 50);
const C: (u8, u8, u8) = (0, 198, 254);

/**
 Processes help commands based on user input. 
 */
pub fn process_help(args: &[&str]) {
    if args.len() == 0 {
        send_base_help_msg();
    }else if args.len() == 1 {
        process_one(args);
    }else if args.len() == 2 {
        process_two(args);
    }
}

fn process_one(args: &[&str]) {
    if args[0].eq_ignore_ascii_case("db") {
        send_db_help_msg();
    }else if args[0].eq_ignore_ascii_case("find") {

    }else if args[0].eq_ignore_ascii_case("read") {

    }else if args[0].eq_ignore_ascii_case("get") {

    }else if args[0].eq_ignore_ascii_case("write") {

    }else if args[0].eq_ignore_ascii_case("put") {

    }else{
        let options = ["db", "find", "read", "get", "write", "put"];
        send_unknown_cmd("help", args, &options);
    }
}

fn process_two(args: &[&str]) {
    if args[0].to_lowercase() == "db" {
        if args[1].eq_ignore_ascii_case("select") {
            send_db_select_help_msg();
        }else if args[1].eq_ignore_ascii_case("create") {

        }else if args[1].eq_ignore_ascii_case("delete") || args[1].eq_ignore_ascii_case("destroy") {
            
        }else {

        }
    }
}
////////////////////////////////////////////////////0-1 args
pub fn send_base_help_msg() {
    println!("DBMS {} help", "command".truecolor(C.0, C.1, C.2));
    println!("{} - used to provide help for a given command", "help".truecolor(Y.0, Y.1, Y.2));
    println!("{} - used to interact with a database and it's properties", "db".truecolor(Y.0, Y.1, Y.2));
    println!("{} - used to search for an entry in a selected database", "get".truecolor(Y.0, Y.1, Y.2));
    println!("{} - used to input a new entry into a selected database", "put".truecolor(Y.0, Y.1, Y.2));
}

fn send_db_help_msg() {
    println!("DBMS {} help", "db".truecolor(C.0, C.1, C.2));
    println!("db {} - used to create a new database", "create".truecolor(Y.0, Y.1, Y.2));
    println!("db {} - used to delete an existing database", "destroy".truecolor(Y.0, Y.1, Y.2));
    println!("db {} - used to select a database for future queries", "select <name>".truecolor(Y.0, Y.1, Y.2));
}

fn send_find_help_msg() {
    
}

fn send_read_help_msg() {

}

fn send_get_help_msg() {

}

fn send_write_help_msg() {

}

fn send_put_help_msg() {

}
/////////////////////////////////////////////////////2 args
fn send_db_select_help_msg() {
    println!("DBMS {} help", "db select".truecolor(C.0, C.1, C.2));
    println!("db select {} - input the name of the parent folder containing generated db files", "<name-of-database>".truecolor(Y.0, Y.1, Y.2));
}

fn send_db_create_help_msg() {
    println!("DBMS {} help", "db create".truecolor(C.0, C.1, C.2));
    println!("db create {} - input the name you wish to use to reference your new database", "<name-of-database>".truecolor(Y.0, Y.1, Y.2));
}

fn send_db_destroy_help_msg() {
    println!("DBMS {} help", "db destroy".truecolor(C.0, C.1, C.2));
    println!("db destroy {} - input the name of the database you wish to destroy", "<name-of-database>".truecolor(Y.0, Y.1, Y.2));
}

pub fn send_unknown_cmd(cmd:&str, args: &[&str], options: &[&str]) {//Unknown command! You typed "&7cmd sub &esub"; valid options are "&a<list>"
    let mut op_string = String::new();
    for option in options {
        op_string = op_string+option+", ";
    }
    op_string.truncate(op_string.len() - 2);

    let mut args_string = String::new();
    args_string = args_string+cmd+" ";
    for arg in args {
        args_string = args_string+arg+" "
    }
    args_string.trim();
    println!("{} you typed {}; valid options are {}", "Unknown command!".red(), args_string.truecolor(Y.0, Y.1, Y.2), op_string.truecolor(C.0, C.1, C.2));
}