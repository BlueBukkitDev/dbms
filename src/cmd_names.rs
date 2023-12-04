pub enum Roots {
    EXIT, HELP, DB, FIND, READ, GET, WRITE, PUT
}

pub enum HelpSubs {
    EXIT, HELP, DB, FIND, READ, GET, WRITE, PUT
}

pub enum HelpDbSubs {
    CREATE, DELETE, SELECT
}

pub enum HelpFindSubs {
    NUM, ALL
}

pub enum HelpReadSubs {
    INDEX, PROPERTY, ALL
}

pub enum HelpWriteSubs {
    PROPERTY
}