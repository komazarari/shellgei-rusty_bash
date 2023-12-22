//SPDX-FileCopyrightText: 2022 Ryuichi Ueda ryuichiueda@gmail.com
//SPDX-License-Identifier: BSD-3-Clause

use crate::{ShellCore, Feeder};
use crate::elements::word::Word;
use super::Subword;

#[derive(Debug)]
pub struct BraceSubword {
    text: String,
    words: Vec<Word>, 
}

impl Subword for BraceSubword {
    fn get_text(&self) -> String { self.text.clone() }
}

impl BraceSubword {
    fn new() -> BraceSubword {
        BraceSubword {
            text: String::new(),
            words: vec![],
        }
    }

    fn eat_word(feeder: &mut Feeder, ans: &mut BraceSubword, core: &mut ShellCore) -> bool {
        match Word::parse(feeder, core) {
            Some(w) => {
                ans.text += &w.text;
                ans.words.push(w);
                true
            },
            _       => false,
        }
    }

    pub fn parse(feeder: &mut Feeder, core: &mut ShellCore) -> Option<BraceSubword> {
        if ! feeder.starts_with("{") {
            return None;
        }
        feeder.set_backup();

        core.word_nest.push("{".to_string());
        let mut ans = Self::new();
        ans.text = feeder.consume(1); // {
        while Self::eat_word(feeder, &mut ans, core) {
            if feeder.starts_with(",") {
                ans.text += &feeder.consume(1); 
                continue;
            }

            if feeder.starts_with("}") {
                ans.text += &feeder.consume(1); 
            }
            break;
        }

        core.word_nest.pop();
        if ! ans.text.ends_with("}") || ans.words.len() < 2 {
            feeder.rewind();
            None
        }else {
            feeder.pop_backup();
//            dbg!("{:?}", &ans);
            Some(ans)
        }
    }
}
