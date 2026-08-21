use std::fs::File;
use std::io::{self, Read, Write};
use crate::commands::{display_operand, err_msg, resolve_path};

fn copy_out(src: &mut dyn Read, out: &mut dyn Write, name: &str) {
    let mut buffer = [0u8; 8192];

    loop {
        match src.read(&mut buffer) {
            Ok(0) => break,

            Ok(n) => {
                if let Err(err) = out.write_all(&buffer[..n]) {
                    eprintln!("cat: {}: {}", name, err_msg(&err));
                    break;
                }
            }

            Err(err) => {
                eprintln!("cat: {}: {}", name, err_msg(&err));
                break;
            }
        }
    }
}

pub fn cat(args: &[&str]) {
    // Lock stdout once for the whole run instead of per 8 KiB write.
    let stdout = io::stdout();
    let mut out = stdout.lock();

    // No operands is the same as a single `-`: read standard input.
    let stdin_only = ["-"];
    let files: &[&str] = if args.is_empty() { &stdin_only } else { args };

    for &file in files {
        if file == "-" {
            copy_out(&mut io::stdin().lock(), &mut out, "-");
            continue;
        }

        let name = display_operand(file);
        let path = resolve_path(file);

        match File::open(&path) {
            Ok(mut file_handle) => copy_out(&mut file_handle, &mut out, &name),
            Err(err) => eprintln!("cat: {}: {}", name, err_msg(&err)),
        }
    }

    // A tail with no trailing newline stays buffered until here, so this is the
    // only place its write error can surface. GNU blames `write error`, not the file.
    if let Err(err) = out.flush() {
        eprintln!("cat: write error: {}", err_msg(&err));
    }
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
