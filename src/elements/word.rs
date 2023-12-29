//SPDX-FileCopyrightText: 2023 Ryuichi Ueda ryuichiueda@gmail.com
//SPDX-License-Identifier: BSD-3-Clause

use crate::{Feeder, ShellCore};
use crate::elements::subword;
use crate::elements::subword::Subword;

#[derive(Debug)]
pub struct Word {
    pub text: String,
    pub subwords: Vec<Box<dyn Subword>>,
    //pub evaluated: Vec<Vec<String>>,
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
        }
    }

    fn append(&mut self, evaluated: &Vec<Vec<String>>, subword_pos: usize) -> Vec<Vec<String>> {
        let rights = self.subwords[subword_pos].eval();
        //dbg!("{:?}", &rights);

        if evaluated.len() == 0 { //先頭のsubword
            return rights.to_vec();
        }

        let len = evaluated.len();
        let mut new_evaluated = vec![];
        for i in 0..len {
            let right_len = rights.len();

            for r in 0..right_len {
                new_evaluated.push(connect_args(&evaluated[i], &rights[r]));
            }
        }

        new_evaluated
    }

    pub fn get_args(&mut self) -> Vec<String> {
        let mut tmp = vec![];
        let evaluated = self.eval();
        for v in &evaluated {
            tmp.extend(v.to_vec());
        }
        tmp
    }

    pub fn eval(&mut self) -> Vec<Vec<String>> {
        let mut evaluated: Vec<Vec<String>> = vec![];
        for i in 0..self.subwords.len() {
            evaluated = self.append(&evaluated, i);
        }
        evaluated
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
