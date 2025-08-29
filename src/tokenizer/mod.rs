mod parsing;
use parsing::lexing::Lexer;
use parsing::AstNode;
use parsing::Command;
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
			}
		}
	}
}

fn evaluate_pipeline(commands: Vec<Command>) {
	for command in commands {
		println!("{:?}", command);
	}
}
