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

fn repeat(at_most: bool, src: &str, op: &Op, num: u64) -> u64 {
    let mut count = 0;
    for token in src.chars() {
        let success = op.run(Some(token), None);
        if !success || (at_most && count == num) {
            break;
        }
        count += 1;
    }
    count
}

// start with just match for now
// TODO: refactor probably the worst code i've ever written
impl RegExpr {
    pub fn contain_match(&self,  src: &str) ->  bool {
        let mut chr_idx = 0; // global run character idx
        let mut src_idx = 0; // local run character idx
        let mut instr_idx = 0;
        loop {
            for (i, token) in src[src_idx..].chars().enumerate() {
                let mut successful;
                let instr = self.instructions.get(instr_idx).unwrap();
                let mut count = 0;
                match instr {
                    Op::Final => return instr.run(None, None),
                    Op::AtLeast(_) => {
                        instr_idx += 1;
                        let op = self.instructions.get(instr_idx).unwrap();
                        count = repeat(false, &src[src_idx+i..], &op, 0);
                        successful = instr.run(Some(token), Some(count));
                        // necessary so that current character isn't skipped
                        if count == 0 && successful {
                            src_idx += i;
                            instr_idx += 1;
                            break;
                        }
                    },
                    Op::AtMost(num) => {
                        // same here
                        instr_idx += 1;
                        let op = self.instructions.get(instr_idx).unwrap();
                        count = repeat(true, &src[src_idx+i..], &op, *num);
                        successful = instr.run(Some(token), Some(count));
                        if count == 0 && successful {
                            src_idx += i;
                            instr_idx += 1;
                            break;
                        }
                    },
                    _ => {},
                };
                successful = instr.run(Some(token), Some(count));
                if !successful {
                    instr_idx = 0;
                    chr_idx += 1;
                    src_idx = chr_idx;
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
        let re = "abc.*[]+";
        let tgt = vec![Op::Cmp('a'), Op::Cmp('b'), Op::Cmp('c'), 
                       Op::AtLeast(0), Op::NoOp, Op::Cmp('['), Op::AtLeast(1), 
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
        let re = "abc.+[]+?";
        assert_eq!(Err("invalid quantifier position"), compile(re));
    }

    #[test]
    fn contains_match_valid_test() {
        let re = "abce*a[]+";
        // let re = "abce*.?[]+";
        let regex = compile(re).unwrap();
        let is_match = regex.contain_match("eftabca[]]]]");
        assert!(is_match);
    }

    #[test]
    fn contains_match_invalid_test() {
        let re = "ab*a[]+";
        let regex = compile(re).unwrap();
        let is_match = regex.contain_match("abc[]]]]");
        assert!(!is_match);
    }
}
