use super::Command;
pub struct Echo;


impl Command for Echo {
    fn name(&self) -> &'static str { "echo" }
    fn run(&self, args: &[&str]) {
        println!("{}", args.join(" "));
    }
}
