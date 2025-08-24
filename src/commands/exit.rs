use super::Command;

pub struct Exit;

impl Command for Exit {
    fn name(&self) -> &'static str { "exit" }

    fn run(&self, args: &[&str]) {
        let mut code = 0;
        if let Some(arg) = args.get(0) {
            if let Ok(n) = arg.parse::<i32>() {
                code = n;
            }
        }
        std::process::exit(code);
    }
}
