pub use lexing::Lexer;

#[derive(Clone, Debug, PartialEq)]
pub struct Command {
    pub program: String,
    pub arguments: Vec<String>,
}

#[derive(PartialEq, Debug, Clone)]
pub enum AstNode {
    Pipeline(Vec<Command>),
    Command(Command),
    None,
}

impl AstNode {
    pub fn new(lexer: Lexer) -> Vec<Vec<AstNode>> {
        let mut result: Vec<Vec<AstNode>> = Vec::new();
        let mut sequence: Vec<AstNode> = Vec::new();
        let mut current_cmd: Command = Command {
            program: String::new(),
            arguments: Vec::new(),
        };
        let mut current_token: AstNode = AstNode::None;
        let mut inside_pipeline: bool = false;

        for lexeme in lexer.lexemes {
            match lexeme.as_str() {
                "&&" | ";" => {
                    if inside_pipeline && let AstNode::Pipeline(pipeline_cmds) = &mut current_token {
                        pipeline_cmds.push(current_cmd.clone());
                        sequence.push(current_token.clone());
                        inside_pipeline = false;
                    } else {
                        sequence.push(AstNode::Command(current_cmd.clone()));
                    }
                    result.push(sequence.clone());
                    current_token = AstNode::None;
                    sequence = Vec::new();
                    current_cmd = Command {
                        program: String::new(),
                        arguments: Vec::new(),
                    };
                }
                "|" => {
                    inside_pipeline = true;
                    if !matches!(current_token, AstNode::Pipeline(_)) {
                        let mut pipes = Vec::new();
                        pipes.push(current_cmd.clone());
                        current_token = AstNode::Pipeline(pipes);
                    } else if let AstNode::Pipeline(pipeline_cmds) = &mut current_token {
                        pipeline_cmds.push(current_cmd.clone());
                    }
                    current_cmd = Command {
                        program: String::new(),
                        arguments: Vec::new(),
                    };
                }
                _ => {
                    if !current_cmd.program.is_empty() {
                        current_cmd.arguments.push(lexeme);
                    } else {
                        current_cmd.program = lexeme;
                    }
                }
            }
        }

        if inside_pipeline {
            if let AstNode::Pipeline(pipeline_cmds) = &mut current_token {
                pipeline_cmds.push(current_cmd.clone());
                sequence.push(current_token.clone());
            }
        }

        if
            !current_cmd.program.is_empty() &&
            let AstNode::Pipeline(pipeline_cmds) = &mut current_token
        {
            pipeline_cmds.push(current_cmd.clone());
            sequence.push(current_token);
        } else if !current_cmd.program.is_empty() {
            sequence.push(AstNode::Command(current_cmd.clone()));
        }

        if !sequence.is_empty() {
            result.push(sequence);
        }
        result
    }
}
