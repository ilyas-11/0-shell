
#[derive(Debug)]
pub enum ParseError {
    UnclosedSingleQuote,
    UnclosedDoubleQuote,
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
    let mut mode = Mode::Normal;

    let mut chars = input.chars();
    while let Some(ch) = chars.next() {
        //println!("{} ", ch);
        match ch {
            '\\' if mode == Mode::Normal => {
                if let Some(next) = chars.next() {
                    current.push(next);
                }
            }
            '\\' if mode == Mode::Double => {
                if let Some(&next) = chars.clone().next().as_ref() {
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
            // #
            }
            '"' if mode ==Mode::Normal||mode == Mode::Double=> {
                if mode ==Mode::Normal{
                    mode = Mode::Double;
                }else if mode == Mode::Double{
                    mode =Mode::Normal
                }
            }
            '\'' if mode ==Mode::Normal||mode == Mode::Single=> {
                if mode ==Mode::Normal{
                    mode = Mode::Single;
                }else if mode == Mode::Single{
                    mode =Mode::Normal
                }
            }
           ' ' | '\t' if mode ==Mode::Normal => {
                if !current.is_empty() {
                    args.push(std::mem::take(&mut current));
                }
            }

            _ => current.push(ch),
        }
    }

    if !current.is_empty() {
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
