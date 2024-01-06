use crate::command::{Command, CommandBehavior, Variables};

#[derive(Clone)]
pub struct ExitCommand {
    pub command: Command,
}

//TODO
impl CommandBehavior for ExitCommand {
    fn execute(&self, variables: Option<Variables>) {
        std::process::exit(0);
    }

    fn check_pattern(&self, pattern: &String, stdin: &str) -> Option<Variables> {
        // pattern: "exit" will show how to use command `x`,such as "help regression"
        // 注意命令名称已经知道了
        let stdin_vec: Vec<&str> = stdin.split_whitespace().collect();
        if stdin_vec.len() > 1{
            eprintln!("command `exit` should not have parameters.");
        }        
        return None;
    }
}
