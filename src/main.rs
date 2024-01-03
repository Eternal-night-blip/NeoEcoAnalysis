use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;
use std::io;

fn main() {
    welcome();
    loop {
        read_command();

        execute_command();
    }
}

fn welcome() {
    println!("Welcome to NeoEcoAnalysis. Version 0.0.0, supported by Li Yijia from Central China Normal University");
}

fn read_command() {
    // regression y, x1 x2 x3
    // reg y, x1 x2 x3
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("无法读取输入");
    Command::explain_from(&buffer);
}

fn execute_command() {}

// 命令 注册一个命令，一个命令需要命令名，用法模式，以及文档

#[derive(Serialize, Deserialize)]
struct Command {
    name: String,
    alias: Option<String>,
    pattern: String,
    //document: Document
}

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
    fn explain_from(stdin: &String) {
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
                        Self::explain_by_pattern(command_name.unwrap(), stdin);
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

    fn explain_by_pattern(name: &str, stdin: &String) {
        let pattern = Self::get_pattern_by(name); // 注意也要识别命令别名，还没有解决
        match Self::check_pattern(&pattern, stdin) {
            Some(variables) => {
                if Self::does_variables_in_data_table() {
                    Self::execute(&variables)
                }
            }
            None => {}
        };
    }

    fn execute(variables: &Variables) {
        dbg!(variables);
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

    fn get_pattern_by(name: &str) -> String {
        let path = format!("./command/{}.json", name);
        let mut content = String::new();
        match fs::read_to_string(&path) {
            Ok(_content) => {
                content = _content;
            }

            Err(_) => {
                // 命令找不到，试试命令别名
                let name_file_path = format!("./command/name.json");
                // 此时 name.json文件一定能找到
                let name_file_content = fs::read_to_string(name_file_path).unwrap();

                let values: serde_json::Value = serde_json::from_str(&name_file_content).unwrap();
                let not_alias = values[name].as_str().unwrap().to_string(); //此时name是别名，通过别名获取到了正式命令名称
                let not_alias_path = format!("./command/{}.json", not_alias);
                match fs::read_to_string(&not_alias_path) {
                    Ok(not_alias_content) => {
                        content = not_alias_content;
                    }
                    Err(_) => {
                        //别名也不是，那么该命令本地不存在或者拼写错误了
                        eprintln!("command '{}' does not exist!", name);
                    }
                }
            }
        };
        let command: Command = serde_json::from_str(&content).unwrap();
        return command.pattern;
    }

    fn check_pattern(pattern: &String, stdin: &str) -> Option<Variables> {
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
        if(stdin_vec.len()<3){
            // 这表示只有命令没有参数，比如输入reg 或者reg y
            eprintln!("Sytex error");
            return None;
        }

        // 两个iterator互相比较进行模式匹配,命令名或者别名本身无需比较，只需要比较参数
        if stdin_vec[2] == "," {
            let explained = stdin_vec[1].to_string();
            
            if !Self::check_variable_name(explained.as_str()) {
                eprintln!("Syntex error, special characters are not allowed to be explained variable");
                return None;
            }

            let explaining :Vec<String>= stdin_vec[3..].iter().map(|&s| s.to_owned()).collect();
            if explaining.is_empty(){
                eprintln!("Syntex error, no explaining variables");
                return None;
            }

            for each in &explaining{
                if !Self::check_variable_name(each.as_str()) {
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

    fn check_variable_name(variable_name:&str) ->bool {
        //特殊字符不允许成为变量名
        let special_characters = vec![",","，",";","；"];
        for character in special_characters{
            if variable_name.contains(character){
                return false; // 不允许变量名里包含逗号与分号,糟糕的变量名
            }
        }
        
        return true;//好的变量名


    }

    fn does_variables_in_data_table() -> bool {
        true
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

