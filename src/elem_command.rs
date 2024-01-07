//SPDX-FileCopyrightText: 2022 Ryuichi Ueda ryuichiueda@gmail.com
//SPDX-License-Identifier: BSD-3-Clause

use std::process;
use crate::{ShellCore,Feeder};
use nix::unistd;
use nix::unistd::ForkResult;
use nix::errno::Errno;
use std::ffi::CString;
use std::env;
use std::path::Path;

pub struct Command {
    _text: String,
    args: Vec<String>,
    cargs: Vec<CString>,
}

impl Command {
    pub fn exec(&mut self, core: &mut ShellCore) { //引数_coreはまだ使いません
        if self.args[0] == "exit" {
            eprintln!("exit!");
            if self.args.len() > 1 {
                core.vars.insert("?".to_string(), self.args[1].clone());
            }

            let exit_status = match core.vars["?"].parse::<i32>() {
                Ok(n) => n%256,
                Err(_) => {
                    eprintln!("sh: exit: {}: numeric argument required", core.vars["?"]);
                    2
                }
            };
            process::exit(exit_status);
        }

        if self.args[0] == "cd" && self.args.len() > 1 {
            let path = Path::new(&self.args[1]);
            let exit_status = match env::set_current_dir(&path) {
                Ok(_) => 0,
                Err(_) => 1,
            };
            if exit_status != 0 {
                println!("Failed to change directory.");
            }

            core.vars.insert("?".to_string(), exit_status.to_string());
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

        if args.len() > 0 {
            Some( Command {_text: line, args, cargs} )
        } else {
            None
        }
    }
}
