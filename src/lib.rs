pub struct RegExpr {}

// starting with just simple stuff currently
// when this works, then will handle other
// stuff such as []  & ()
pub enum Token {
    Any,
    Char(char),
    Quantifier(char),
}

fn lexer(src: &str) -> Vec<Token> {
    let tokens = Vec::with_capacity(64);
    for (i, chr) in src.chars().enumerate() {
        todo!();
    }

    tokens
}

pub fn compile(src: &str) -> RegExpr {
    let tokens = lexer(src);
    for (i, token) in tokens.iter().enumerate() {
        match token {
            _ => todo!(),
        }
    }
    RegExpr{}
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
}
