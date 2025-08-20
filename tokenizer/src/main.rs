#[warn(unused_variables)]
fn main() {
    let source = "for (0 == 0) {
    	let i = 1;
    }";
    let mut syntax: Vec<String> = Vec::new();
    let mut stack: Vec<char> = Vec::new();
    syntax.push(String::new());
//    let mut ignore = false;
    let mut pointer = 0;
    let mut line = 0;
    for c in source.chars() {
		match c {
			'{'|'(' => {
				stack.push(c);
				pointer += 1;
				if pointer == syntax.len() {
					syntax.push(String::new());
				}
//				ignore = true;
			}
			'}'|')' => {
				stack.pop();
//				pointer -= 1;
//				ignore = false;
			}
			'\n' => {
				line += 1;
				continue;
			}
			'\t'=> continue,
			_ => {
				syntax[pointer].push(c);
			}
		}
    }
    syntax.clone().into_iter().for_each(|sr| println!("{}", sr.to_string().trim()));
    println!("syntax: \n{:?}", syntax);
    println!("pointer: {}, line: {}, stack: {:?}", pointer, line, stack);
}
