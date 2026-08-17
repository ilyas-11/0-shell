use std::fs::File;
use std::io::{self, Read, Write};

pub fn cat(args: &[&str]) {
    let mut stdout = io::stdout();

    if args.is_empty() {
        let stdin = io::stdin();
        let mut stdin = stdin.lock();

        let mut buffer = [0u8; 8192];

        loop {
            match stdin.read(&mut buffer) {
                Ok(0) => break,

                Ok(n) => {
                    if let Err(err) = stdout.write_all(&buffer[..n]) {
                        eprintln!("cat: {}", err);
                        break;
                    }
                }

                Err(err) => {
                    eprintln!("cat: {}", err);
                    break;
                }
            }
        }

        return;
    }

    for file in args {
        match File::open(file) {
            Ok(mut file) => {
                let mut buffer = [0u8; 8192];

                loop {
                    match file.read(&mut buffer) {
                        Ok(0) => break,

                        Ok(n) => {
                            if let Err(err) = stdout.write_all(&buffer[..n]) {
                                eprintln!("cat: {:?}: {}", file, err);
                                break;
                            }
                        }

                        Err(err) => {
                            eprintln!("cat: {:?}: {}", file, err);
                            break;
                        }
                    }
                }
            }

            Err(err) => {
                eprintln!("cat: {}: {}", file, err);
            }
        }
    }

    let _ = stdout.flush();
}
 // todo list
//test 

// 1- cat file.txt (file is to long 2GB)

// 2- cat d.txt

// 3- cat a.txt b.txt c.txt 
// aaaaaaaaaaaaaaaaaaaaaaaaaaaaa
// bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbcccccccccccccccccccccccccccccccccccccc% 
// return $ in new line 


//pased :

// cat ../b.txt
//cat 0-shell/b.txt
// cat (no arguments)

//cat a.txt f.txt b.txt
// command: cat, args: ["a.txt", "f.txt", "b.txt"]
// cat: f.txt : No such file or directory (os error 2)
// aaaaaaaaaaaaaaaaaaaaabbbbbbbbbbbbbbbbbbbbbbbbbbbbbb$
//(file not found) edit error message position to be after the file name and before the colon


//https://chatgpt.com/c/6a721872-b5d4-83ea-9cd1-c3baaec25191
//https://chatgpt.com/c/6a7b4e21-b1c4-83ea-9b73-066f27255774
