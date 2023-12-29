//SPDX-FileCopyrightText: 2022 Ryuichi Ueda ryuichiueda@gmail.com
//SPDX-License-Identifier: BSD-3-Clause

use crate::{ShellCore, Feeder};
use crate::elements::subword::UnquotedSubword;
use crate::elements::word::Word;
use super::Subword;

#[derive(Debug)]
pub struct BraceSubword {
    text: String,
    words: Vec<Word>,
}

impl Subword for BraceSubword {
    fn get_text(&self) -> String { self.text.clone() }

    fn eval(&mut self) -> Vec<Vec<String>> { 
        let mut ans = vec![];
        for w in self.words.iter_mut() {
            ans.extend(w.eval()); 
        }

        ans
    }
}

impl BraceSubword {
    fn new() -> BraceSubword {
        BraceSubword {
            text: String::new(),
            words: vec![],
        }
    }

    fn new_at_failure() -> BraceSubword {
        let sub = Box::new(UnquotedSubword::new_with_text("{".to_string()));
        let mut word = Word::new();
        word.subwords.push(sub);
        BraceSubword {
            text: "{".to_string(),
            words: vec![word],
        }
    }

    fn eat_word(feeder: &mut Feeder, ans: &mut BraceSubword, core: &mut ShellCore) -> bool {
        match Word::parse(feeder, core) {
            Some(w) => {
                ans.text += &w.text;
                ans.words.push(w);
                true
            },
            _  => false,
        }
    }

    pub fn parse(feeder: &mut Feeder, core: &mut ShellCore) -> Option<BraceSubword> {
        if ! feeder.starts_with("{") {
            return None;
        }
        feeder.set_backup();

        core.word_nest.push("{".to_string());
        let mut ans = Self::new();
        let mut closed = false;
        ans.text = feeder.consume(1); // {

        while Self::eat_word(feeder, &mut ans, core) {
            if feeder.starts_with(",") {
                ans.text += &feeder.consume(1); 
                continue;
            }

            if feeder.starts_with("}") {
                ans.text += &feeder.consume(1); 
                closed = true;
            }
            break;
        }

        core.word_nest.pop();
        if ! closed || ans.words.len() < 2 {
            feeder.rewind();
            feeder.consume(1);
            Some(Self::new_at_failure())
        }else {
            feeder.pop_backup();
//            dbg!("{:?}", &ans);
            Some(ans)
        }
    }
}
