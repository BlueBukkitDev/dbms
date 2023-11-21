use crate::help_msg::*;

pub fn process_db(args:&[&str]) {
    if args.len() == 0 {
        //no args, return incorrect or help.
    }
    if args[0].eq_ignore_ascii_case("create") {
        //Create should use a path, or at least have a path subcmd option to dictate where a database gets placed. 
        if args.len() < 2 {
            //return incorrect usage, must at least have a name
        }else if args.len() == 2 {//has a name, but no path or option
            execute_create_cmd(args);
        }else if args.len() == 3 {//has either a path or option
            if args[2].eq_ignore_ascii_case("-c") {
                execute_create_custom_cmd(args);
            }else {//we have to assume it is a path. Maybe it is, maybe not. User error is gonna kick a few here. 
                
            }
        }else if args.len() == 4 {//has both a path and an option
            if args[1].eq_ignore_ascii_case("-c") || args[2].eq_ignore_ascii_case("-c") {

            }else {
                //incorrect usage; too many variables, since not using an option. Gonna need to not split text in quotes at whitespace, since that can screw up paths. 
            }
        }else {
            //incorrect usage, too many args
        }
    }else if args[0].eq_ignore_ascii_case("delete") {
        if args.len() == 2 {
            execute_delete_cmd(args);//Should have a confirmation
        }else {
            //not enough or too many args
        }
    }else if args[0].eq_ignore_ascii_case("select") {
        //Select is going to have to include a path, unless we create a "db.ptr" file to point to locations of databases by name. Actually I like that. 
        if args.len() == 2 {
            execute_select_cmd(args);
        }else {
            //not enough or too many args
        }
    }else {
        let options = ["create", "delete", "select"];
        send_unknown_cmd("db", args, &options);
    }
}

fn execute_create_cmd(_args: &[&str]) {
    
}

fn execute_create_custom_cmd(_args: &[&str]) {

}

fn execute_delete_cmd(_args: &[&str]) {

}

fn execute_select_cmd(_args: &[&str]) {

}