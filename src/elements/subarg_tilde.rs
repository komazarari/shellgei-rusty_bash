//SPDX-FileCopyrightText: 2022 Ryuichi Ueda ryuichiueda@gmail.com
//SPDX-License-Identifier: BSD-3-Clause

use crate::debuginfo::DebugInfo;
use crate::ShellCore;
use crate::Feeder;
//use crate::feeder::scanner::*;
use crate::utils::expand_tilde;

use crate::abst_elems::ArgElem;

pub struct SubArgTildeUser {
    pub text: String,
    pub pos: DebugInfo,
}

impl ArgElem for SubArgTildeUser {
    fn get_text(&self) -> String {
        self.text.clone()
    }

    fn eval(&mut self, _conf: &mut ShellCore) -> Vec<Vec<String>> {
        vec!(vec!(expand_tilde(&self.text).0))
    }
}

impl SubArgTildeUser {
    pub fn parse(text: &mut Feeder) -> Option<SubArgTildeUser> {
        let pos = text.scanner_tilde_prefix();
        if pos != 0 {
            Some( SubArgTildeUser{text: text.consume(pos), pos: DebugInfo::init(text) } )
        }else{
            None
        }
    }
}
