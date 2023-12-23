//SPDX-FileCopyrightText: 2022 Ryuichi Ueda ryuichiueda@gmail.com
//SPDX-License-Identifier: BSD-3-Clause

use crate::{ShellCore, Feeder};
use crate::elements::subword;
use crate::elements::subword::UnquotedSubword;
use super::Subword;
use crate::elements::word::Subwords;

#[derive(Debug)]
pub struct BraceSubword {
    text: String,
    subwords: Vec<Subwords>,
    //words: Vec<Word>,
}

impl Subword for BraceSubword {
    fn get_text(&self) -> String { self.text.clone() }
}

impl BraceSubword {
    fn new() -> BraceSubword {
        BraceSubword {
            text: String::new(),
            subwords: vec![],
        }
    }

    fn new_at_failure() -> BraceSubword {
        let sub = Box::new(UnquotedSubword::new_with_text("{".to_string()));
        BraceSubword {
            text: "{".to_string(),
            subwords: vec![vec![sub]],
        }
    }

    fn eat_word(feeder: &mut Feeder, ans: &mut BraceSubword,
                core: &mut ShellCore, counter: usize) -> bool {
        match subword::parse(feeder, core) {
            Some(w) => {
                ans.text += &w.get_text();
                if ans.subwords.len() == counter {
                    ans.subwords.push(vec![]);
                }
                ans.subwords[counter].push(w);
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
        let mut counter = 0;
        while Self::eat_word(feeder, &mut ans, core, counter) {
            if feeder.starts_with(",") {
                ans.text += &feeder.consume(1); 
                counter += 1;
                continue;
            }

            if feeder.starts_with("}") {
                ans.text += &feeder.consume(1); 
            }
            break;
        }

        core.word_nest.pop();
        if ! ans.text.ends_with("}") || ans.subwords.len() < 2 {
            feeder.rewind();
            feeder.consume(1);
            Some(Self::new_at_failure())
        }else {
            feeder.pop_backup();
            Some(ans)
        }
    }
}
