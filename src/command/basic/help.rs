use crate::command::{Command, CommandBehavior, Variables};

#[derive(Clone)]
pub struct HelpCommand {
    pub command: Command,
}

impl CommandBehavior for HelpCommand {
    fn execute(&self, variables_option: Option<Variables>) {
        match variables_option {
            None => {
                if let Some(help) = Command::deserialize_command(self.command.name.as_str()) {
                    println!("{}", help.document);
                }
            }

            Some(variables) => {
                let parameter_name = variables.explained.as_str();
                if Command::does_exist(Some(parameter_name)) {
                    //此时参数(是被解释的命令名)`x `存在，则在./command目录下找到对应的x.json文件
                    if let Some(explained_cmd) =
                        Command::deserialize_command(variables.explained.as_str())
                    {
                        println!("{}", explained_cmd.document);
                    }
                } else {
                    println!(
                        "help command's parameter  `{}` that should be a command does not exist",
                        parameter_name
                    );
                }
            }
        }
    }

    fn check_pattern(&self, pattern: &String, stdin: &str) -> Option<Variables> {
        // pattern: "help x" will show how to use command `x`,such as "help regression"
        // 注意命令名称已经知道了
        //let pattern_vec: Vec<&str> = pattern.split_whitespace().collect();
        let stdin_vec: Vec<&str> = stdin.split_whitespace().collect();

        // help命令一共只有一个参数,包括命令本身就只有2个字符串
        if stdin_vec.len() > 2 {
            eprintln!("Syntex error,command 'help' just has one parameter which is a command name");
            return None;
        }

        if stdin_vec.len() == 1 {
            return None;
        }

        let parameter_name = stdin_vec[1];
        let explained = parameter_name.to_string();
        Some(Variables {
            explained,
            explaining: Vec::new(),
            other: Vec::new(),
        })
    }
}
