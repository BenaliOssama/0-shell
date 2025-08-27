#[derive(PartialEq, Debug)]
enum Status {
    Quotes(char),
    Word,
    Operation(char),
    Null,
}

struct Data {
    tokens: Vec<String>,
}

#[derive(Clone, Debug, PartialEq)]
struct Command {
    cmd: String,
    args: Vec<String>,
}

#[derive(PartialEq, Debug, Clone)]
enum Token {
    Pipe(Vec<Command>),
    Cmd(Command),
    Null,
}

impl Token {
    fn new(data: Data) -> Vec<Vec<Token>> {
        let mut res: Vec<Vec<Token>> = Vec::new();
        let mut seq: Vec<Token> = Vec::new();
        let mut command: Command = Command {
            cmd: String::new(),
            args: Vec::new(),
        };
        let mut token: Token = Token::Null;
        let mut pipe: bool = false;
        for tkn in data.tokens {
            match tkn.as_str() {
                "&&" | ";" => {
                    if pipe && let Token::Pipe(pipe_commands) = &mut token {
                        pipe_commands.push(command.clone());
                        seq.push(token.clone());
                        pipe = false;
                    } else {
                        seq.push(Token::Cmd(command.clone()));
                    }
                    res.push(seq.clone());
                    token = Token::Null;
                    seq = Vec::new();
                    command = Command {
                        cmd: String::new(),
                        args: Vec::new(),
                    };
                }
                "|" => {
                    pipe = true;
                    if !matches!(token, Token::Pipe(_)) {
                        let mut pipes = Vec::new();
                        pipes.push(command.clone());
                        token = Token::Pipe(pipes);
                    } else {
                        if let Token::Pipe(pipe_commands) = &mut token {
                            pipe_commands.push(command.clone());
                        }
                    }
                    command = Command {
                        cmd: String::new(),
                        args: Vec::new(),
                    };
                }
                _ => {
                    if !command.cmd.is_empty() {
                        command.args.push(tkn);
                    } else {
                        command.cmd = tkn;
                    }
                }
            }
        }
        if pipe {
            if let Token::Pipe(pipe_commands) = &mut token {
                pipe_commands.push(command.clone());
                seq.push(token.clone());
            };
        }
        if command.cmd != "" && let Token::Pipe(pipe_commands) = &mut token {
            pipe_commands.push(command.clone());
            seq.push(token);
        } else if command.cmd != "" {
            seq.push(Token::Cmd(command.clone()));
        }
        if seq.len() != 0 {
            res.push(seq);
        }
        res
    }
}

impl Data {
    fn new(input: &str) -> Data {
        let mut tkns = Data { tokens: Vec::new() };
        let mut status: Status = Status::Null;
        let chars: Vec<char> = input.chars().collect::<Vec<char>>();
        let mut word: String = String::new();
        let input_len = chars.len();
        let mut index: usize = 0;
        while index < input_len {
            match chars[index] {
                '"' | '\'' => {
                    if matches!(status, Status::Quotes(c) if c == chars[index]) {
                        tkns.tokens.push(word.clone());
                        status = Status::Null;
                        word = String::new();
                    } else if matches!(status, Status::Quotes(_)) {
                        word.push(chars[index]);
                    } else {
                        status = Status::Quotes(chars[index]);
                    }
                }
                '|' | '&' | '>' | '<' | ';' => {
                    if
                        matches!(status, Status::Quotes(_)) ||
                        matches!(status, Status::Operation(c) if c == chars[index])
                    {
                        word.push(chars[index]);
                    } else if status != Status::Null {
                        tkns.tokens.push(word.clone());
                        word = String::new();
                    }
                    if status == Status::Word || status == Status::Null {
                        status = Status::Operation(chars[index]);
                        word.push(chars[index]);
                    }
                }
                ' ' if word.len() != 0 && !matches!(status, Status::Quotes(_)) => {
                    tkns.tokens.push(word.clone());
                    word = String::new();
                }
                _ => {
                    if chars[index] != ' ' || matches!(status, Status::Quotes(_)) {
                        if matches!(status, Status::Operation(_)) && word != "" {
                            tkns.tokens.push(word.clone());
                            word = String::new();
                        }
                        if !matches!(status, Status::Quotes(_)) {
                            status = Status::Word;
                        }
                        word.push(chars[index]);
                    }
                }
            }
            index += 1;
        }
        if word.len() != 0 {
            tkns.tokens.push(word.clone());
        }
        tkns.tokens = tkns.tokens
            .clone()
            .into_iter()
            .filter(|s| s != "")
            .collect();
        tkns
    }
}

fn main() {
    let commands: Vec<&'static str> = vec!["l1 jfdlj | l2 arg2 && l3 | l4 arg arg arg | cmd | l5;"];
    for cmd in commands {
        let tkn = Data::new(cmd);
        println!("{:?}", tkn.tokens);
        let tokens = Token::new(tkn);
        println!("{:?}", tokens);
    }
}
