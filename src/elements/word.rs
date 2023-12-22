//SPDX-FileCopyrightText: 2023 Ryuichi Ueda ryuichiueda@gmail.com
//SPDX-License-Identifier: BSD-3-Clause

use crate::{Feeder, ShellCore};

#[derive(Debug)]
pub struct Word {
    pub text: String,
    pub subwords: Vec<String>,
}

impl Word {
    pub fn new() -> Word {
        Word {
            text: String::new(),
            subwords: vec![],
        }
    }

    pub fn parse(feeder: &mut Feeder, core: &mut ShellCore) -> Option<Word> {
        let mut ans = Word::new();

        let arg_len = feeder.scanner_word(core);
        if arg_len == 0 {
            return None;
        }
 
        ans.text = feeder.consume(arg_len);
        Some(ans)
    }
}
