use crate::help_msg::send_unknown_cmd;


pub fn process_db(args:&[&str]) {
    if(args.len() == 0){
        //no args, return incorrect or help.
    }
    if args[0].eq_ignore_ascii_case("create") {
        if args.len() < 2 {
            //return incorrect usage, must at least have a name
        }
    }else if args[0].eq_ignore_ascii_case("delete") {

    }else if args[0].eq_ignore_ascii_case("select") {

    }else {
        let options = ["create", "delete", "select"];
        send_unknown_cmd("db", args, &options);
    }
}

fn execute_create_cmd(args: &[&str]) {
    if args[2].eq_ignore_ascii_case("-c"){

    }
}