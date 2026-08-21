
#[derive(Debug)]
pub enum ParseError {
    UnclosedSingleQuote,
    UnclosedDoubleQuote,
    LineContinuation,

}

#[derive(Clone, Copy, PartialEq)]
enum Mode{
    Normal,
    Single,
    Double,
}

pub fn parse(input: &str) -> Result<Vec<String>,ParseError> {
    let mut args = Vec::new();
    let mut current = String::new();
    // A word can be started and still be empty: `""` is a real, empty argument.
    // Tracking it separately also tells us whether we are at a word boundary,
    // which is the only place a `#` opens a comment.
    let mut started = false;
    let mut mode = Mode::Normal;

    let mut chars = input.chars().peekable();
    while let Some(ch) = chars.next() {
        match ch {
             '#' if mode == Mode::Normal && !started => {
                break;
            }
            '\\' if mode == Mode::Normal => {
                match chars.next() {
                    Some(next) => {
                        current.push(next);
                        started = true;
                    }
                    None => {
                        return Err(ParseError::LineContinuation);
                    }
                }
            }
            '\\' if mode == Mode::Double => {
                if let Some(&next) = chars.peek() {
                    match next {
                        '"' | '\\' | '$' | '`' => {
                            current.push(next);
                            chars.next();
                        }
                        
                        '\n' => {
                            chars.next();
                        }
                        
                        _ => {
                            current.push('\\');
                        }
                    }
                } else {
                    current.push('\\');
                }
            }
            '"' if mode ==Mode::Normal||mode == Mode::Double=> {
                started = true;
                if mode ==Mode::Normal{
                    mode = Mode::Double;
                }else if mode == Mode::Double{
                    mode =Mode::Normal
                }
            }
            '\'' if mode ==Mode::Normal||mode == Mode::Single=> {
                started = true;
                if mode ==Mode::Normal{
                    mode = Mode::Single;
                }else if mode == Mode::Single{
                    mode =Mode::Normal
                }
            }
            ' ' | '\t' if mode ==Mode::Normal => {
                if started {
                    args.push(std::mem::take(&mut current));
                    started = false;
                }
            }

            _ => {
                current.push(ch);
                started = true;
            }


        }
    }
    if started {
        args.push(current);
    }
    
    if mode == Mode::Double {
        return Err(ParseError::UnclosedDoubleQuote);
    }

    if mode == Mode::Single {
        return Err(ParseError::UnclosedSingleQuote);
    }
    Ok(args)
}


//test 

// echo "\"" => "
//echo '\''  => '
