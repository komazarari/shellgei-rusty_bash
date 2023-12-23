//SPDX-FileCopyrightText: 2023 Ryuichi Ueda ryuichiueda@gmail.com
//SPDX-License-Identifier: BSD-3-Clause

use crate::{Feeder, ShellCore};
use crate::elements::subword;
use crate::elements::subword::Subword;

pub type Subwords = Vec<Box<dyn Subword>>;

#[derive(Debug)]
pub struct Word {
    pub text: String,
    pub subwords: Subwords,
}

impl Word {
    pub fn new() -> Word {
        Word {
            text: String::new(),
            subwords: vec![],
        }
    }

    pub fn eval(&self) -> Vec<String> {
        vec![self.text.clone()]
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
            dbg!("{:?}", &ans);
            Some(ans)
        }
    }
}
