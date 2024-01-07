pub mod basic;
pub mod compute;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::rc::Rc;

#[derive(Serialize, Deserialize, Eq, Hash, PartialEq, Clone)]
pub struct Command {
    name: String,
    alias: Option<String>,
    pattern: String,
    document: String,
}

#[derive(Serialize, Deserialize, Eq, Hash, PartialEq, Clone)]
pub struct CommandBuilder {
    command: Command,
}

impl CommandBuilder {
    pub fn new(command_name: String) -> Self {
        Self {
            command: Command {
                name: command_name,
                alias: None,
                pattern: String::new(),
                document: String::new(),
            },
        }
    }

    pub fn add_alias(mut self, alias: Option<String>) -> Self {
        match alias {
            None => {
                panic!("parameter alias can't be None when you use add_alias method.")
            }
            Some(_) => {
                self.command.alias = alias;
                self
            }
        }
    }

    pub fn add_pattern(mut self, pattern: String) -> Self {
        if pattern.is_empty() {
            panic!("parameter pattern can't be empty when you use add_pattern method.")
        }

        self.command.pattern = pattern;
        self
    }

    pub fn add_document(mut self, document: String) -> Self {
        if document.is_empty() {
            panic!("parameter document can't be empty when you use add_document method.")
        }

        self.command.document = document;
        self
    }

    pub fn build(self) -> Command {
        // 检查name,pattern,document不能为空
        if self.command.name.is_empty() {
            panic!("name can't be empty when you build a Command.");
        }

        if self.command.pattern.is_empty() {
            panic!("pattern can't be empty when you build a Command.");
        }

        if self.command.document.is_empty() {
            panic!("document can't be empty when you build a Command.");
        }
        self.command
    }
}

impl Command {
    pub fn explain_from(
        stdin: &String,
        command_behavior_map: &HashMap<Command, Rc<dyn CommandBehavior>>,
    ) -> Option<CommandParser> {
        // 提取命令名称
        let items: Vec<&str> = stdin.split(",").collect();
        let pattern_prefix = items[0].trim();
        let mut prefix_iterator = pattern_prefix.split_whitespace();
        let command_option = prefix_iterator.next();

        //首先判断是不是基础命令
        match command_option {
            Some(command_name) => {
                if Self::does_exist(command_option) {
                    Self::explain_by_pattern(command_name, stdin, command_behavior_map)
                } else {
                    eprintln!("Command '{}' does not exit!", command_name);
                    None
                }
            }
            None => {
                eprintln!("Command is not input, which is not allowed !");
                None
            }
        }
    }

    fn explain_by_pattern(
        name: &str,
        stdin: &String,
        command_behavior_map: &HashMap<Command, Rc<dyn CommandBehavior>>,
    ) -> Option<CommandParser> {
        if let Some(command) = Self::deserialize_command(name) {
            let specific_command = command_behavior_map.get(&command).unwrap();

            match specific_command.check_pattern(&command.pattern, stdin) {
                Some(variables) => Some(CommandParser {
                    specific_command: specific_command.clone(),
                    variables: Some(variables),
                }),
                None => Some(CommandParser {
                    specific_command: specific_command.clone(),
                    variables: None,
                }),
            }
        } else {
            None
        }
    }

    // name of command can be formal or its alias
    pub fn deserialize_command(name: &str) -> Option<Command> {
        match Self::deserialize_command_by_name(name) {
            None => {
                // 命令找不到，试试命令别名
                let name_file_path = format!("./command/name.json");
                // 此时 name.json文件一定能找到
                let name_file_content = fs::read_to_string(name_file_path).unwrap();

                let values: serde_json::Value = serde_json::from_str(&name_file_content).unwrap();
                let not_alias = values[name].as_str().unwrap().to_string(); //此时name是别名，通过别名获取到了正式命令名称

                match Self::deserialize_command_by_name(&not_alias) {
                    Some(cmd) => Some(cmd),
                    None => {
                        //别名也不是，那么该命令本地不存在或者拼写错误了
                        eprintln!("command '{}' does not exist!", name);
                        return None;
                    }
                }
            }

            Some(cmd) => Some(cmd),
        }
    }

    // name of command must be formal
    // if name is formal, return Command struct
    // if name is alias, return None to tell deserialize_command method to use formal name
    fn deserialize_command_by_name(name: &str) -> Option<Command> {
        let path = format!("./command/{}.json", name);
        match fs::read_to_string(&path) {
            Ok(_content) => {
                let command: Command = serde_json::from_str(&_content).unwrap();
                return Some(command);
            }

            Err(_) => {
                //此处无需报错,因为name可能是别名,而别名是没有单独的json文件的
                return None;
            }
        };
    }

    /*通过检查./command/name.json与./command/{official command name}.json文件来确定命令是否存在
     *先检查name.json文件,再检查{official command name}.json文件,
     *当{official command name}.json里有{name}命令则返回true,否则其他情况返回false
     *
     * !!!注意目前只检查./command/name.json文件中是否存在该命令
     */
    fn does_exist(name: Option<&str>) -> bool {
        let name = match name {
            Some(str) => str,
            None => {
                eprintln!("Command is not inputed, which is not allowed !");
                return false;
            }
        };
        let path = format!("./command/name.json");

        match fs::read_to_string(&path) {
            Ok(content) => {
                let values: serde_json::Value = serde_json::from_str(&content).unwrap();
                return !values[name].is_null();
            }
            Err(_) => {
                eprintln!("command/name.json file does not exist");
                return false;
            }
        }
    }

    fn check_variable_name(variable_name: &str) -> bool {
        //特殊字符不允许成为变量名
        let special_characters = vec![",", "，", ";", "；"];
        for character in special_characters {
            if variable_name.contains(character) {
                return false; // 不允许变量名里包含逗号与分号,糟糕的变量名
            }
        }

        return true; //好的变量名
    }
}

pub trait CommandBehavior {
    fn execute(&self, variables: Option<Variables>);
    fn check_pattern(&self, pattern: &String, stdin: &str) -> Option<Variables>;
}

#[derive(Debug, Clone)]
pub struct Variables {
    explained: String,
    explaining: Vec<String>,
    other: Vec<String>,
}

pub struct CommandParser {
    specific_command: Rc<dyn CommandBehavior>,
    variables: Option<Variables>,
}

impl CommandParser {
    pub fn command(&self) -> &dyn CommandBehavior {
        self.specific_command.as_ref()
    }

    pub fn variables(&self) -> &Option<Variables> {
        &self.variables
    }
}
