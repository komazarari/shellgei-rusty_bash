//SPDX-FileCopyrightText: 2023 Ryuichi Ueda ryuichiueda@gmail.com
//SPDX-License-Identifier: BSD-3-Clause

use crate::{Feeder, ShellCore};
use crate::elements::subword;
use crate::elements::subword::Subword;

#[derive(Debug)]
pub struct Word {
    pub text: String,
    pub subwords: Vec<Box<dyn Subword>>,
    pub evaluated: Vec<Vec<String>>,
}

fn connect_args(left: &Vec<String>, right: &Vec<String>) -> Vec<String> {
    if left.len() == 0 {
        return right.to_vec();
    }

    let mut tmp = left.to_vec();
    tmp.extend(right.to_vec());

    let con_pos = left.len()-1;
    let new_str = tmp[con_pos].clone() + &tmp[con_pos+1];

    tmp.remove(con_pos);
    tmp.remove(con_pos);
    tmp.insert(con_pos, new_str);

    tmp
}

impl Word {
    pub fn new() -> Word {
        Word {
            text: String::new(),
            subwords: vec![],
            evaluated: vec![],
        }
    }

    fn append(&mut self, subword_pos: usize) {
        let subword = &mut self.subwords[subword_pos];

        if self.evaluated.len() == 0 { //先頭のsubword
            self.evaluated = subword.eval();
            return;
        }

        let len = self.evaluated.len();
        let mut new_evaluated = vec![];
        for i in 0..len {
            let rights = subword.eval();
            let right_len = rights.len();

            for r in 0..right_len {
                new_evaluated.push(connect_args(&self.evaluated[i], &rights[r]));
            }
        }

        self.evaluated = new_evaluated;
    }

    pub fn eval(&mut self) -> Vec<String> {
        for i in 0..self.subwords.len() {
            self.append(i);
        }

        let mut ans = vec![];
        for v in &self.evaluated {
            ans.extend(v.to_vec());
        }

        ans
    }

    pub fn parse(feeder: &mut Feeder, core: &mut ShellCore) -> Option<Word> {
        let mut ans = Word::new();
        while let Some(sw) = subword::parse(feeder, core) {
            ans.text += &sw.get_text();
            ans.subwords.push(sw);
        }

        if ans.text.len() == 0 {
            None
        }else{
            Some(ans)
        }
    }
}
