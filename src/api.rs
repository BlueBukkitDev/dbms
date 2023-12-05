use crate::parsing::entry_parse::*;

///
/// Session stores data related to the use and access of this library.
/// 
/// This relevant persistent data may include the selected database, username/password/pattern, previous commands for selecting tags,
/// or any host of other information. For now this only includes the selected database and authorization information, which is still
/// unimplemented. 
///
pub struct Session {
    selected_db : String,
    auth : Auth,
}

impl Session {
    ///
    /// Creates a new session with no stored memory.
    ///
    pub fn new() -> Session {
        setup_program_files();
        Session {
            selected_db : String::new(),
            auth : Auth::new()
        }
    }
    pub fn select_db(&mut self, name: &str) {
        self.selected_db = name.to_string();
    }
    pub fn get_selected_db(self) -> String {
        self.selected_db
    }
    pub fn set_auth_user(&mut self, user : String) {
        self.auth.add_user(user);
    }
    pub fn set_auth_pass(&mut self, pass : String) {
        self.auth.add_pass(pass);
    }
    pub fn set_auth_patern(&mut self, pattern : Vec<i64>) {
        self.auth.add_pattern(pattern);
    }
    pub fn get_auth_user(&self) -> &String {
        &self.auth.user
    }
    pub fn get_auth_pass(&self) -> &String {
        &self.auth.pass
    }
    pub fn get_auth_pattern(&self) -> &Vec<i64> {
        &self.auth.pattern
    }
}

pub struct API {
    session: Session
}

impl API {
    ///
    /// Creates a new API instance from which to perform internal operations. 
    /// 
    /// Creates program files at the default location (system root). 
    ///
    pub fn new_default(&self) -> API {
        API {
            session: Session::new()
        }
    }
    ///
    /// Creates a new API instance from which to perform internal operations.
    /// 
    /// The `path` variable is the path to the location where you want program files stored. This can be anywhere, by default it is at
    /// system root (C:/ in Windows). These files will determine and record how this library should be used to create databases. If you
    /// will be having multiple databases with different tags in each, simply create a new instance of this API with a different path
    /// specific to that database. You can do this for as many databases as you wish. 
    ///
    pub fn new(&self, path: &str) -> API {
        API {
            session: Session::new()
        }
    }
    /**
    Takes in a command and an array of subcommands. Redirects to other functions depending on the series.
    */
    pub fn run_command(cmd: &str, args:&[&str]) {
        process_command(cmd, args)
    }

    pub fn select_db (&mut self, name: &str) {
        self.session.select_db(name)
    }

    pub fn create_db (&mut self, _name: &str) {

    }

    pub fn create_db_at_path (&mut self, _name: &str, _path: String) {

    }
}

pub struct Auth {
    user : String,
    pass : String,
    pattern : Vec<i64>
}

impl Auth {
    pub fn new() -> Auth {
        Auth {
            user : String::new(),
            pass : String::new(),
            pattern : Vec::new()
        }
    }

    pub fn add_user (&mut self, user : String) {
        self.user = user;
    }

    pub fn add_pass (&mut self, pass : String) {
        self.pass = pass;
    }

    pub fn add_pattern (&mut self, pattern : Vec<i64>) {
        self.pattern = pattern;
    }
}
