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

struct Command {
    cmd: String,
    args: Vec<String>,
    bg: bool,
}

enum Token {
    Pipe((Output<Command>, Output<Command>)),
    Cmd(Command),
}

impl Token {
    fn new(data: Data) -> Vec<Vec<Token>> {
        let mut res: Vec<Vec<Token>> = Vec::new();
        let mut seq: Vec<Token> = Vec::new();
        let mut command: Command = Command {
            cmd: String::new(),
            args: Vec::new(),
            bg: bool,
        };
        let mut token: Token;
        for tkn in data.tokens {
            match tkn {
                "&&" | ";" => {
                    seq.push(Token::Cmd(command));
                    res.push(seq);
                }
                "|" => {
                    token = Token::Pipe((Some(command), None));
                    
                }
                _ => {
                    if command.cmd.is_empty() {
                        command.args.push(tkn);
                    } else {
                        command.cmd = tkn;
                    }
                }
            }
        }
    }
}

impl Data {
    fn new(input: &str) -> Data {
        let mut tkns = Data {
            tokens: Vec::new(),
        };
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
        println!("{:?}", tkns.tokens);
        tkns
    }
}

fn main() {
    let commands: Vec<&'static str> = vec![
        "ls -l /home/user",
        "grep \"pattern\" file.txt",
        "cat file1.txt file2.txt > output.txt",
        "echo \"Hello, World!\" | tee log.txt",
        "mkdir new_folder && cd new_folder",
        "rm -rf old_folder &",
        "VAR=value",
        "echo $VAR",
        "command $(subcommand)",
        "find . -name \"*.py\" -exec grep \"def \" {} \\;",
        "la|ls",
        "ls -l | grep file; la"
    ];
    for cmd in commands {
        let _tkn = Data::new(cmd);
    }
}
