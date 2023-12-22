//SPDX-FileCopyrightText: 2023 Ryuichi Ueda ryuichiueda@gmail.com
//SPDX-License-Identifier: BSD-3-Clause

use crate::{Feeder, ShellCore};
use crate::elements::subword;
use crate::elements::subword::Subword;

#[derive(Debug)]
pub struct Word {
    pub text: String,
    pub subwords: Vec<Box<dyn Subword>>,
}

impl Word {
    pub fn new() -> Word {
        Word {
            text: String::new(),
            subwords: vec![],
        }
    }

    pub fn parse(feeder: &mut Feeder, core: &mut ShellCore) -> Option<Word> {
        let sw = match subword::parse(feeder, core) {
            Some(sw) => sw,
            _        => return None, 
        };

        let mut ans = Word::new();
        ans.text = sw.get_text();
        ans.subwords.push(sw);
        Some(ans)
    }
}
