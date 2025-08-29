mod tokenizer;
use tokenizer::evaluate;

fn main() {
	let cmd_line_vec = vec![
	    "ls -l | grep txt",               // Pipe: output of ls to grep
	    "mkdir test_dir && cd test_dir", // AND: cd only if mkdir succeeds
	    "echo 'hello' > file.txt ; cat file.txt", // Semicolon: two sequential commands
	    "false && echo 'will not run'",  // AND with failing first command
	    "echo done ; ls | wc -l",        // Mix of ; and pipe
	];
	for cmd_line in cmd_line_vec {
		evaluate(cmd_line);
	}
}
