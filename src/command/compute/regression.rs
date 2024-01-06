use crate::command::{Command, CommandBehavior, Variables};

#[derive(Clone)]
pub struct RegressionCommand {
    pub command: Command,
}

impl CommandBehavior for RegressionCommand {
    fn execute(&self, variables_option: Option<Variables>) {

        match variables_option{
            None => {return;}
            Some(variables) => 
                dbg!(variables),
            
        };
        
    }

    fn check_pattern(&self, pattern: &String, stdin: &str) -> Option<Variables> {
        // 注意命令名称已经知道了，只需要检查第一个逗号前是否有被解释变量y，
        // 以及第一个逗号后(第一个分号之前)是否存在数据中的被解释变量
        // 如果存在第一个分号...todo
        // ","变成" , "
        //let pattern = pattern.replace(",", " , ");
        let stdin = stdin.replace(",", " , ");

        // 根据空白分割
        //let pattern_vec: Vec<&str> = pattern.split_whitespace().collect();
        let stdin_vec: Vec<&str> = stdin.split_whitespace().collect();

        // 需要检查只输入命令主干，没有后续参数的情形
        if stdin_vec.len() < 3 {
            // 这表示只有命令或者也仅包含被解释变量，比如输入reg 或者reg y
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
