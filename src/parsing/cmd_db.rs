use crate::parsing::help_msg::*;

pub fn process_db(args:&[&str]) {
    match args.len() {
        0 => return,//send incorrect or help message
        1 => process_one(args),
        _ => return,
    }
}

fn process_one(args: &[&str]) {
    match args[0] {
        "create" => process_create_cmd(args),
        "delete" => execute_delete_cmd(args),
        "select" => execute_select_cmd(args),
        _ => {
            let options = ["create", "delete", "select"];
            send_unknown_cmd("db", args, &options);},
    }
}

//Create should use a path, or at least have a path subcmd option to dictate where a database gets placed. 
fn process_create_cmd(args: &[&str]) {
    match args.len() {
        1 => return,//return incorrect usage, must at least have a name
        2 => execute_create(args),//has a name, but no path or option
        3 | 4 => process_create_options(args),//has either a path or option
        _ => return,//too many options; send help msg or smth
    }
}

fn process_create_options(args: &[&str]) {
    if args[2].eq_ignore_ascii_case("-c") {//3
        execute_create_custom_cmd(args);
    }else {//we have to assume it is a path. Maybe it is, maybe not. User error is gonna kick a few here. 
        
    }
    if args[1].eq_ignore_ascii_case("-c") || args[2].eq_ignore_ascii_case("-c") {//4

    }else {
        //incorrect usage; too many variables, since not using an option. Gonna need to not split text in quotes at whitespace, since that can screw up paths. 
    }
}

fn execute_create(args: &[&str]) {

}

fn execute_create_custom_cmd(_args: &[&str]) {

}

fn execute_delete_cmd(args: &[&str]) {
    if args.len() == 2 {
        execute_delete_cmd(args);//Should have a confirmation
    }else {
        //not enough or too many args
    }
}

fn execute_select_cmd(args: &[&str]) {
    //Select is going to have to include a path, unless we create a "db.ptr" file to point to locations of databases by name. Actually I like that. 
    if args.len() == 2 {
        execute_select_cmd(args);
    }else {
        //not enough or too many args
    }
}