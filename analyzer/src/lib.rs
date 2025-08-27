#[derive(PartialEq, Debug)]
pub enum ParseState {
    Quote(char),
    Word,
    Operator(char),
    None,
}

pub struct Lexer {
    lexemes: Vec<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Command {
    program: String,
    arguments: Vec<String>,
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

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        let mut lexer = Lexer { lexemes: Vec::new() };
        let mut state: ParseState = ParseState::None;
        let chars: Vec<char> = input.chars().collect();
        let mut buffer: String = String::new();
        let input_len = chars.len();
        let mut index: usize = 0;

        while index < input_len {
            match chars[index] {
                '"' | '\'' => {
                    if matches!(state, ParseState::Quote(c) if c == chars[index]) {
                        lexer.lexemes.push(buffer.clone());
                        state = ParseState::None;
                        buffer.clear();
                    } else if matches!(state, ParseState::Quote(_)) {
                        buffer.push(chars[index]);
                    } else {
                        state = ParseState::Quote(chars[index]);
                    }
                }
                '|' | '&' | '>' | '<' | ';' => {
                    if
                        matches!(state, ParseState::Quote(_)) ||
                        matches!(state, ParseState::Operator(c) if c == chars[index])
                    {
                        buffer.push(chars[index]);
                    } else if state != ParseState::None {
                        lexer.lexemes.push(buffer.clone());
                        buffer.clear();
                    }
                    if state == ParseState::Word || state == ParseState::None {
                        state = ParseState::Operator(chars[index]);
                        buffer.push(chars[index]);
                    }
                }
                ' ' if !buffer.is_empty() && !matches!(state, ParseState::Quote(_)) => {
                    lexer.lexemes.push(buffer.clone());
                    buffer.clear();
                }
                _ => {
                    if chars[index] != ' ' || matches!(state, ParseState::Quote(_)) {
                        if matches!(state, ParseState::Operator(_)) && !buffer.is_empty() {
                            lexer.lexemes.push(buffer.clone());
                            buffer.clear();
                        }
                        if !matches!(state, ParseState::Quote(_)) {
                            state = ParseState::Word;
                        }
                        buffer.push(chars[index]);
                    }
                }
            }
            index += 1;
        }

        if !buffer.is_empty() {
            lexer.lexemes.push(buffer);
        }

        lexer.lexemes = lexer.lexemes
            .into_iter()
            .filter(|s| !s.is_empty())
            .collect();

        lexer
    }
}
