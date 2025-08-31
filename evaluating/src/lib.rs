use parsing::Lexer;
use parsing::{ AstNode, Command };
use executer::{ exec, Cmd };
use std::io;

pub fn evaluate(user_input: &str) {
    let mut cmd_line: String = user_input.to_string().trim().to_string();
    if !cmd_line.ends_with(";") && !cmd_line.ends_with("&&") {
        cmd_line += ";";
    }
    let lexer_tokens = Lexer::new(&cmd_line);
    let ast_data = AstNode::new(lexer_tokens);
    for sub_vector in ast_data {
        for node in sub_vector {
            if let AstNode::Pipeline(commands) = node {
                evaluate_pipeline(commands);
            } else if let AstNode::Command(command) = node {
                // from_filename(".env").expect("Failed to read .env file");
                exec(to_cmd(command));
            }
        }
    }
}

fn evaluate_pipeline(commands: Vec<Command>) {
    for command in commands {
        exec(to_cmd(command));
    }
}

fn to_cmd(command: Command) -> Cmd {
    Cmd {
        cmd: command.program,
        args: command.arguments,
        stdin: Box::new(io::stdin()),
        stdout: Box::new(io::stdout()),
        stderr: Box::new(io::stderr()),
    }
}
