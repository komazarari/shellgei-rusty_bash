//SPDX-FileCopyrightText: 2022 Ryuichi Ueda ryuichiueda@gmail.com
//SPDX-License-Identifier: BSD-3-Clause

use crate::{ShellCore,Feeder};
use nix::unistd;
use std::ffi::CString;
use std::process;

use nix::unistd::ForkResult;
use nix::errno::Errno;

pub struct Command {
    _text: String,
    args: Vec<String>,
    cargs: Vec<CString>,
}

impl Command {
    pub fn exec(&mut self, core: &mut ShellCore) {
        if core.run_builtin(&mut self.args) {
            return;
        }

        match unsafe{unistd::fork()} {
            Ok(ForkResult::Child) => {
                match unistd::execvp(&self.cargs[0], &self.cargs) {
                    Err(Errno::EACCES) => {
                        println!("sh: {}: Permission denied.", &self.args[0]);
                        process::exit(126);
                    },
                    Err(Errno::ENOENT) => {
                        println!("sh: {}: command not found.", &self.args[0]);
                        process::exit(127);
                    },
                    Err(err) => {
                        println!("Failed to exec: {:?}", err);
                        process::exit(127);
                    },
                    _ => ()
                }
            },
            Ok(ForkResult::Parent { child }) => {
                core.wait_process(child);
            },
            Err(err) => panic!("Failed to fork. {}", err),
        }
    }

    pub fn parse(feeder: &mut Feeder, _core: &mut ShellCore) -> Option<Command> {
        let line = feeder.consume(feeder.remaining.len());

        let args: Vec<String> = line
            .trim_end()
            .split(' ')
            .map(|s| s.to_string())
            .collect();

        let cargs: Vec<CString> = args
            .iter()
            .map(|w| CString::new(w.clone()).unwrap())
            .collect();

        if args.len() > 0 { // 1つ以上の単語があれば Command を作成する
            Some( Command {_text: line, args, cargs} )
        } else {
            None
        }
    }
}
