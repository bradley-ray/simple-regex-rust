#[derive(Debug, PartialEq)]
pub enum Op {
    NoOp,
    Cmp(char),
    AtLeast(u64),
    AtMost(u64),
    Final,
}

impl Op {
    fn run(&self, chr: Option<char>, num: Option<u64>) -> bool {
        match self {
            Op::Cmp(op) => *op == chr.unwrap(),
            Op::AtLeast(n) => *n <= num.unwrap(),
            _ => true,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct RegExpr{ instructions: Vec<Op> }


fn is_quantifier(token: char) -> bool {
    token == '*' || token == '+' || token == '?'
}

pub fn compile(src: &str) -> Result<RegExpr, &str> {
    let mut instructions = Vec::with_capacity(64);
    let mut valid_quant = false;
    for (i, token) in src.chars().enumerate() {
        if is_quantifier(token) && valid_quant {
            valid_quant = false;
            instructions.insert(i-1, match token {
                '*' => Op::AtLeast(0),
                '+' => Op::AtLeast(1),
                _   => Op::AtMost(1),
            });
        } else if !is_quantifier(token) {
            valid_quant = true;
            instructions.push(match token {
                '.' => Op::NoOp,
                chr => Op::Cmp(chr),
            })
        } else {
            return Err("invalid quantifier position");
        }
    }

    instructions.push(Op::Final);
    Ok(RegExpr{ instructions })
}

// start with just match for now
// TODO: refactor probably the worst code i've ever written
impl RegExpr {
    pub fn contain_match(&self,  src: &str) ->  bool {
        let mut instr_idx = 0;
        let mut src_idx = 0;
        loop {
            for (i, token) in src[src_idx..].chars().enumerate() {
                let successful;
                let instr = self.instructions.get(instr_idx).unwrap();
                let mut count = 0;
                match instr {
                    Op::AtLeast(_) => {
                        instr_idx += 1;
                        let cmp = self.instructions.get(instr_idx).unwrap();
                        for token_ in src[src_idx+i..].chars() {
                            if !cmp.run(Some(token_), None) {
                                break;
                            }
                            count += 1;
                        }
                        successful = instr.run(None, Some(count));
                    },
                    Op::AtMost(num) => {
                        instr_idx += 1;
                        let cmp = self.instructions.get(instr_idx).unwrap();
                        for token_ in src[src_idx+i..].chars() {
                            if !cmp.run(Some(token_), None) || count >= *num {
                                break;
                            }
                            count += 1;
                        }
                        successful = instr.run(None, None);
                    },
                    Op::Cmp(_) => successful = instr.run(Some(token), None),
                    Op::NoOp => successful = instr.run(None, None),
                    Op::Final => return instr.run(None, None),
                };
                if !successful {
                    instr_idx = 0;
                    src_idx += 1;
                    break;
                }
                instr_idx += 1;
            }
            if src_idx >= src.len() {
                return false;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compile_valid_test() {
        let re = "abc.[]+";
        let tgt = vec![Op::Cmp('a'), Op::Cmp('b'), Op::Cmp('c'), 
                       Op::NoOp, Op::Cmp('['), Op::AtLeast(1), 
                       Op::Cmp(']'), Op::Final];
        let res = compile(re).unwrap();
        assert_eq!(tgt, res.instructions);
    }

    #[test]
    fn compile_invalid_test() {
        let re = "*abc.[]+";
        assert_eq!(Err("invalid quantifier position"), compile(re));
        let re = "abc.[]++";
        assert_eq!(Err("invalid quantifier position"), compile(re));
        let re = "abc.[]+?";
        assert_eq!(Err("invalid quantifier position"), compile(re));
    }

    #[test]
    fn contains_match_valid_test() {
        // string needs to contain 'abc'
        // followed by any character
        // followed by [
        // follow by ] one or more times
        let re = "010234";
        let regex = compile(re).unwrap();
        let is_match = regex.contain_match("010234");
        assert!(is_match);

    }
    #[test]
    fn contains_match_invalid_test() {
        // string needs to contain 'abc'
        // followed by any character
        // followed by [
        // follow by ] one or more times
        let re = "aa+.[]+";
        let regex = compile(re).unwrap();
        let is_match = regex.contain_match("abc[]");
        assert!(!is_match);
    }
}
