pub enum roots {
    EXIT,
    HELP, 
    DB, 
    FIND,
    READ,
    GET,
    WRITE,
    PUT
}

pub enum help_subs {
    EXIT, 
    HELP,
    DB,
    FIND,
    READ,
    GET,
    WRITE,
    PUT
}

pub enum help_db_subs {
    CREATE,
    DELETE,
    SELECT
}

pub enum help_find_subs {
    NUM,
    ALL
}

pub enum help_read_subs {

}
//read <property>