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
        let w = match Word::parse(feeder, core) {
            Some(w) => w,
            _       => return false,
        };

        ans.text += &w.text;
        ans.words.push(w);
        true
    }

    pub fn parse(feeder: &mut Feeder, core: &mut ShellCore) -> Option<BraceSubword> {
        if ! feeder.starts_with("{") {
            return None;
        }

        core.word_nest.push(("{".to_string(), "}".to_string()));
        let mut ans = Self::new();
        ans.text = feeder.consume(1); // {
        while Self::eat_word(feeder, &mut ans, core) {
            //TODO
        }

        core.word_nest.pop();
        Some(ans)
    }
}
