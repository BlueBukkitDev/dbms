fn main(){
    run_cmds();
}

fn run_cmds(){
    println!("Commands are as follows: \nGo\nStop");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Did not enter a correct string");
    let cmd = input.trim();
    if cmd.to_lowercase() == "go" {
        println!("You selected {}", cmd);
        run_cmds();
    }else if cmd.to_lowercase() == "stop" {
        println!("Bye!");
    }else {
        println!("Invalid Input! You typed {}, but options are only 'Go' and 'Stop'", cmd);
        run_cmds();
    }
}