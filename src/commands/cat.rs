use std::io::{self,BufRead};
pub fn cat (args: &[&str]){
    if args.is_empty() {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            match line {
                Ok(line) => println!("{}", line),
                Err(err) => {
                    eprintln!("cat: {}", err);
                    break;
                }
            }
        }
        return;
    }
    for file in args{

        match std::fs::read_to_string(file) {
            Ok(content) => print!("{}", content),
            Err(err) => eprintln!("cat: {} : {}",file, err),
        }
    }
    
}

//test 

//cat a.txt b.txt c.txt
//cat d.txt

