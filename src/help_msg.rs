use colored::Colorize;

const Y: (u8, u8, u8) = (241, 194, 50);
const C: (u8, u8, u8) = (0, 198, 254);

pub fn send_base_help_msg() {
    println!("DBMS {} help", "command".truecolor(C.0, C.1, C.2));
    println!("{} - used to provide help for a given command", "help".truecolor(Y.0, Y.1, Y.2));
    println!("{} - used to interact with a database and it's properties", "db".truecolor(Y.0, Y.1, Y.2));
    println!("{} - used to search for an entry in a selected database", "get".truecolor(Y.0, Y.1, Y.2));
    println!("{} - used to input a new entry into a selected database", "put".truecolor(Y.0, Y.1, Y.2));
}

pub fn send_db_help_msg() {
    println!("DBMS {} help", "db".truecolor(C.0, C.1, C.2));
    println!("db {} - used to create a new database", "create".truecolor(Y.0, Y.1, Y.2));
    println!("db {} - used to delete an existing database", "destroy".truecolor(Y.0, Y.1, Y.2));
    println!("db {} - used to select a database for future queries", "select <name>".truecolor(Y.0, Y.1, Y.2));
}

pub fn send_db_select_help_msg() {
    println!("DBMS {} help", "db select".truecolor(C.0, C.1, C.2));
    println!("db select {} - the name of the parent folder containing generated db files", "<name-of-database>".truecolor(Y.0, Y.1, Y.2));
}