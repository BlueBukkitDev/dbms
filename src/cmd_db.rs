use crate::help_msg::*;

pub fn process_db(args:&[&str]) {
    if args.len() == 0 {
        //no args, return incorrect or help.
    }
    if args[0].eq_ignore_ascii_case("create") {
        if args.len() < 2 {
            //return incorrect usage, must at least have a name
        }else if args.len() == 2 {
            execute_create_cmd(args);
        }else if args.len() == 3 {
            if args[2].eq_ignore_ascii_case("-c") {
                execute_create_custom_cmd(args);
            }else {
                //incorrect usage; wrong arg
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