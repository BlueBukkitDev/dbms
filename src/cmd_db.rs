use crate::help_msg::send_unknown_cmd;


pub fn process_db(args:&[&str]) {
    if args[0].eq_ignore_ascii_case("create") {

    }else if args[0].eq_ignore_ascii_case("delete") {

    }else if args[0].eq_ignore_ascii_case("select") {

    }else {
        let options = ["create", "delete", "select"];
        send_unknown_cmd("db", args, &options);
    }
}