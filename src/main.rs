use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::fs;
use std::io;

fn main() {
    // let mut plugins: Vec<Box<dyn Plugin>> = Vec::new();

    // 加载插件并注册命令
    // for plugin_path in plugin_paths {
    //     let plugin = load_plugin(plugin_path);
    //     plugins.push(plugin);
    // }

    // 注册插件的命令
    // for plugin in plugins {
    //     plugin.register_commands(&mut commands);
    // }

    let mut command_and_behavior_map: HashMap<Command, Box<dyn CommandBehavior>> = HashMap::new();

    let help_cmd = Command {
        name: "help".to_string(),
        alias: None,
        pattern: "help x".to_string(),
    };
    let help = HelpCommand {
        command: help_cmd.clone(),
    };
    command_and_behavior_map.insert(help_cmd, Box::new(help));

    let exit_cmd = Command {
        name: "exit".to_string(),
        alias: None,
        pattern: "exit".to_string(),
    };
    let exit = ExitCommand {
        command: exit_cmd.clone(),
    };
    command_and_behavior_map.insert(exit_cmd, Box::new(exit));

    let reg_cmd = Command {
        name: "regression".to_string(),
        alias: Some("reg".to_string()),
        pattern: "reg y, x1 x2 ...".to_string(),
    };
    let reg = RegCommand {
        command: reg_cmd.clone(),
    };
    command_and_behavior_map.insert(reg_cmd, Box::new(reg));

    welcome();
    loop {
        read_command(&command_and_behavior_map);

        execute_command();
    }
}

fn welcome() {
    println!("Welcome to NeoEcoAnalysis. Version 0.0.0, supported by Li Yijia from Central China Normal University");
}

fn read_command(command_behavior_map: &HashMap<Command, Box<dyn CommandBehavior>>) {
    // regression y, x1 x2 x3
    // reg y, x1 x2 x3
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("无法读取输入");
    Command::explain_from(&buffer, &command_behavior_map);
}

fn execute_command() {}

// 命令 注册一个命令，一个命令需要命令名，用法模式，以及文档

#[derive(Serialize, Deserialize, Eq, Hash, PartialEq, Clone)]
struct Command {
    name: String,
    alias: Option<String>,
    pattern: String,
    //document: Document
}

// pub trait Plugin {
//     fn register_commands(&mut self, commands: &mut HashMap<String, Box<dyn CommandBehavior>>);
// }

enum BasicCommand {
    Help,
    Exit,
}

impl BasicCommand {
    fn help() {}

    fn exit() {
        std::process::exit(0);
    }
}

impl Command {
    fn explain_from(
        stdin: &String,
        command_behavior_map: &HashMap<Command, Box<dyn CommandBehavior>>,
    ) {
        // 提取命令名称
        let items: Vec<&str> = stdin.split(",").collect();
        let pattern_prefix = items[0].trim();
        let mut prefix_iterator = pattern_prefix.split_whitespace();
        let command_name = prefix_iterator.next();

        //首先判断是不是基础命令
        match command_name {
            Some(command) => match command {
                "help" => {}
                "exit" => {
                    BasicCommand::exit();
                }
                _ => {
                    //判断命令是否存在，如果命令存在则按照该命令的模式进行解析
                    if Self::does_exist(command_name) {
                        Self::explain_by_pattern(
                            command_name.unwrap(),
                            stdin,
                            command_behavior_map,
                        );
                    } else {
                        eprintln!("Command '{}' does not exit!", command_name.unwrap());
                    }
                }
            },
            None => {
                eprintln!("Command is not input, which is not allowed !");
            }
        }
    }

    fn explain_by_pattern(
        name: &str,
        stdin: &String,
        command_behavior_map: &HashMap<Command, Box<dyn CommandBehavior>>,
    ) {
        // 注意也要识别命令别名，还没有解决
        let mut cmd_name = name.to_string();
        let dose_use_another_name:bool = false;
        let mut pattern = String::new();
        
        match Self::get_pattern_by(name) {
            None => {
                // 命令找不到，试试命令别名
                let name_file_path = format!("./command/name.json");
                // 此时 name.json文件一定能找到
                let name_file_content = fs::read_to_string(name_file_path).unwrap();

                let values: serde_json::Value = serde_json::from_str(&name_file_content).unwrap();
                let not_alias = values[name].as_str().unwrap().to_string(); //此时name是别名，通过别名获取到了正式命令名称
                
                cmd_name = not_alias;
                match Self::get_pattern_by(&cmd_name) {
                    Some(_pattern) => {
                        pattern = _pattern;
                    }
                    None => {
                        //别名也不是，那么该命令本地不存在或者拼写错误了
                        eprintln!("command '{}' does not exist!", name);
                        return;
                    }
                }
            }

            Some(_pattern) => {
                pattern = _pattern;
            }
        }

        let path: String = format!("./command/{}.json", &cmd_name);
        let content = fs::read_to_string(&path).expect(format!("无法读取文件{}", path).as_str());

        let command: Command = serde_json::from_str(&content).expect("命令反序列化失败");
        let specific_command = &command_behavior_map.get(&command).unwrap();

        match specific_command.check_pattern(&pattern, stdin) {
            Some(variables) => {
                if Self::does_variables_in_data_table() {
                    specific_command.execute(&variables)
                }
            }
            None => {}
        };
    }

    fn get_pattern_by(name: &str) -> Option<String> {
        let path = format!("./command/{}.json", name);
        match fs::read_to_string(&path) {
            Ok(_content) => {
                let command: Command = serde_json::from_str(&_content).unwrap();
                return Some(command.pattern);
            }

            Err(_) => {
                return None;
            }
        };
    }

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

    fn does_variables_in_data_table() -> bool {
        true
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
    fn execute(&self, variables: &Variables);
    fn check_pattern(&self, pattern: &String, stdin: &str) -> Option<Variables>;
}

struct RegCommand {
    command: Command,
}

struct HelpCommand {
    command: Command,
}

struct ExitCommand {
    command: Command,
}

//TODO
impl CommandBehavior for HelpCommand {
    fn execute(&self, variables: &Variables) {
        dbg!(variables);
    }

    fn check_pattern(&self, pattern: &String, stdin: &str) -> Option<Variables> {
        return None;
    }
}

//TODO
impl CommandBehavior for ExitCommand {
    fn execute(&self, variables: &Variables) {
        dbg!(variables);
    }

    fn check_pattern(&self, pattern: &String, stdin: &str) -> Option<Variables> {
        return None;
    }
}

impl CommandBehavior for RegCommand {
    fn execute(&self, variables: &Variables) {
        dbg!(variables);
    }

    fn check_pattern(&self, pattern: &String, stdin: &str) -> Option<Variables> {
        // 注意命令名称已经知道了，只需要检查第一个逗号前是否有被解释变量y，
        // 以及第一个逗号后(第一个分号之前)是否存在数据中的被解释变量
        // 如果存在第一个分号...todo
        // ","变成" , "
        let pattern = pattern.replace(",", " , ");
        let stdin = stdin.replace(",", " , ");

        // 根据空白分割
        let pattern_vec: Vec<&str> = pattern.split_whitespace().collect();
        let stdin_vec: Vec<&str> = stdin.split_whitespace().collect();

        // 需要检查只输入命令主干，没有后续参数的情形
        if stdin_vec.len() < 3 {
            // 这表示只有命令没有参数，比如输入reg 或者reg y
            eprintln!("Sytex error");
            return None;
        }

        // 两个iterator互相比较进行模式匹配,命令名或者别名本身无需比较，只需要比较参数
        if stdin_vec[2] == "," {
            let explained = stdin_vec[1].to_string();

            if !Command::check_variable_name(explained.as_str()) {
                eprintln!(
                    "Syntex error, special characters are not allowed to be explained variable"
                );
                return None;
            }

            let explaining: Vec<String> = stdin_vec[3..].iter().map(|&s| s.to_owned()).collect();
            if explaining.is_empty() {
                eprintln!("Syntex error, no explaining variables");
                return None;
            }

            for each in &explaining {
                if !Command::check_variable_name(each.as_str()) {
                    eprintln!("Syntex error, special characters are not allowed to be explaining variable");
                    return None;
                }
            }

            let variables = Variables {
                explained,
                explaining,
                other: vec![String::new()],
            };

            return Some(variables);
        } else {
            eprintln!("Syntax error, comma should be in the third place !");
            return None;
        }
    }
}

struct Pattern {
    str: String,
}

struct Document {}

#[derive(Debug)]
struct Variables {
    explained: String,
    explaining: Vec<String>,
    other: Vec<String>,
}
