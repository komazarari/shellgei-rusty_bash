//SPDX-FileCopyrightText: 2022 Ryuichi Ueda ryuichiueda@gmail.com
//SPDX-License-Identifier: BSD-3-Clause

use std::process;
use crate::{ShellCore,Feeder};
use nix::unistd;
use nix::unistd::ForkResult;
use nix::sys::wait;
use std::ffi::CString;

pub struct Command {
    text: String,
    args: Vec<String>,
    cargs: Vec<CString>,
}

impl Command {
    pub fn exec(&mut self, _core: &mut ShellCore) { //引数_coreはまだ使いません
        if self.text == "exit\n" {
            process::exit(0);
        }

        match unsafe{unistd::fork()} {
            Ok(ForkResult::Child) => {
                let err = unistd::execvp(&self.cargs[0], &self.cargs);
                println!("Failed to exec: {:?}", err);
                process::exit(127);
            },
            Ok(ForkResult::Parent { child }) => {
                let _ = wait::waitpid(child, None);
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

        if args.len() > 0 {
            Some( Command {text: line, args, cargs} )
        } else {
            None
        }
    }
}
