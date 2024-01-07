mod command;

use command::{
    basic::{exit::ExitCommand, help::HelpCommand},
    compute::regression::RegressionCommand,
    Command, CommandBehavior, CommandBuilder, CommandParser,
};
use std::{collections::HashMap, rc::Rc};
use std::io;

fn main() {
    //下面进行命令加载,未来不应该这样硬编码,且用插件的形式加载
    let mut command_and_behavior_map: HashMap<Command, Rc<dyn CommandBehavior>> = HashMap::new();
    let help_cmd = CommandBuilder::new("help".to_string())
        .add_pattern("help x".to_string())
        .add_document("help command document".to_string())
        .build();

    let help = HelpCommand {
        command: help_cmd.clone(),
    };
    command_and_behavior_map.insert(help_cmd, Rc::new(help));

    let exit_cmd = CommandBuilder::new("exit".to_string())
        .add_pattern("exit".to_string())
        .add_document("exit command document".to_string())
        .build();

    let exit = ExitCommand {
        command: exit_cmd.clone(),
    };
    command_and_behavior_map.insert(exit_cmd, Rc::new(exit));

    let reg_cmd = CommandBuilder::new("regression".to_string())
        .add_alias(Some("reg".to_string()))
        .add_pattern("reg y, x1 x2 ...".to_string())
        .add_document("regression command document".to_string())
        .build();

    let reg = RegressionCommand {
        command: reg_cmd.clone(),
    };
    command_and_behavior_map.insert(reg_cmd, Rc::new(reg));
    //let map_poiner = Rc::new(command_and_behavior_map);

    welcome();
    loop {
        let parser_option = read_command(&command_and_behavior_map);

        execute_command(parser_option);
    }
}

fn welcome() {
    println!("Welcome to NeoEcoAnalysis. Version 0.0.0, supported by Li Yijia from Central China Normal University");
}

fn read_command(
    command_behavior_map: &HashMap<Command, Rc<dyn CommandBehavior>>,
) -> Option<CommandParser> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("无法读取输入");
    Command::explain_from(&buffer, command_behavior_map)
}

fn execute_command(parser_option: Option<CommandParser>) {
    if let Some(command_parser) = parser_option {
        let specific_command = command_parser.command();
        let variables = command_parser.variables();
        specific_command.execute(variables.to_owned());
    }
}
