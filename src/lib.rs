pub struct RegExpr {}

// starting with just simple stuff currently
// when this works, then will handle other
// stuff such as []  & ()
#[derive(Debug, PartialEq)]
pub enum Token {
    Any,
    Char(char),
    Quantifier(char),
}

fn lexer(src: &str) -> Result<Vec<Token>, &str> {
    let mut tokens = Vec::with_capacity(64);
    let mut quant_valid = false;
    for (_, chr) in src.chars().enumerate() {
        let is_quantifier = chr == '*' || chr == '+' || chr == '?';
        // quantifiers need to follow a token that can be quantified (Char || Any)
        if is_quantifier && quant_valid  {
                tokens.push(Token::Quantifier(chr));
                quant_valid = false;
        } else if is_quantifier && !quant_valid {
                return Err("invalid quantifier position");
        }else if chr == '.' {
            tokens.push(Token::Any);
            quant_valid = true;
        } else {
            tokens.push(Token::Char(chr));
            quant_valid = true;
        }

    }

    Ok(tokens)
}

pub fn compile(src: &str) -> Result<RegExpr, &str> {
    let tokens = lexer(src);
    for (i, token) in tokens.iter().enumerate() {
        match token {
            _ => todo!(),
        }
    }
    Ok(RegExpr{})
}

// start with just match for now
impl RegExpr {
    pub fn match_expr(&self,  src: &str) ->  Option<String> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexer_valid() {
        let re = "abc*.+[]";
        let tgt = vec![Token::Char('a'), Token::Char('b'), Token::Char('c'), 
                        Token::Quantifier('*'), Token::Any, Token::Quantifier('+'),
                        Token::Char('['), Token::Char(']')];
        let tokens = lexer(re).unwrap();
        assert_eq!(tgt, tokens);
    }

    #[test]
    fn lexer_invalid() {
        let re = "*abc.+[]";
        let tokens_res = lexer(re);
        assert_eq!(tokens_res, Err("invalid quantifier position"));
    }
}
