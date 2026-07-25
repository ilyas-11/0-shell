pub fn cat (args: &[&str]){
    if args.is_empty() {
                    eprintln!("cat: missing operand");
                } else {
                    match std::fs::read_to_string(args[0]) {
                        Ok(content) => {
                            print!("{}", content);
                        }
                        Err(err) => {
                            eprintln!("cat: {}", err);
                        }
                    }
                }
}