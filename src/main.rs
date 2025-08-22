#[derive(PartialEq)]
enum Status {
    Quotes(char),
    Word,
    Operation(char),
    Null,
}

struct Code {
    input: String,
    tokens: Vec<String>,
}

impl Code {
    fn new(input: &str) -> Code {
        let mut tkns = Code {
            input: input.to_string(),
            tokens: Vec::new(),
        };
        let mut status: Status = Status::Null;
        let chars: Vec<char> = input.chars().collect::<Vec<char>>();
        let mut word: String = String::new();
	let input_len = input.len();
        let mut index: usize = 0;
        while index < input.len() {
            match chars[index] {
                '"' | '\'' => {
                    if matches!(status, Status::Quotes(c) if c == chars[index]) {
			tkns.tokens.push(word);
			status = Status::Null;
			word = String::new();
                    } else if status == Status::Null {
			status = Status::Quotes(chars[index]);
                    } else {
			word.push(chars[index]);
                    }
                }
		'|' | '&' | '>' | '<' | ';' => {
		    if matches!(status, Status::Quotes(_)) || matches!(status, Status::Operation(c) if c == chars[index]) {
			word.push(chars[index]);
		    } else if status != Status::Null {
			tkns.tokens.push(word);
			word = String::new();
		    }
		    if status == Status::Word || status == Status::Null {
			status = Status::Operation(chars[index]);
		    }
		}
		' ' if word.len() != 0 => {
		    tkns.tokens.push(word);
		}
		_=> {
		    tkns.tokens.push(word);
		}
            }
	    index += 1;
        }
	println!("Tokens: {:?}", tkns.tokens);
	tkns
    }
}

fn main() {}
