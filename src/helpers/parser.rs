
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

    for ch in input.chars() {
        match ch {
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
    // if mode!= Mode::Normal {
    //         match mode {
    //             Mode::Double => return Err(ParseError::UnclosedDoubleQuote),
    //             Mode::Single => return Err(ParseError::UnclosedSingleQuote),
    //             Mode::Normal => return Err("dquote> ".to_string())
    //         }
    // }
    if mode == Mode::Double {
        return Err(ParseError::UnclosedDoubleQuote);
    }

    if mode == Mode::Single {
        return Err(ParseError::UnclosedSingleQuote);
    }

    Ok(args)
}