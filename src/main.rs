use std::process::{Command, Stdio};

use target::Target;

pub mod lir;
pub mod target;

fn main() {
    let target = target::vm::c::VM {};
    let lir = lir::IR::new(
        vec![],
        lir::IRFunctionEntry::new(
            1000,
            4000,
            vec![
                lir::IRStatement::Push(0.0),
                lir::IRStatement::Push(1.0),
                lir::IRStatement::Add,
            ],
        ),
    );
    let assembly = lir.assemble(&target, 0);
    println!("{}", assembly);
    let mut command = Command::new("gcc");
    let command_mut = command
        .arg("-O2")
        .arg("-o")
        .arg("test.exe")
        .arg("-x")
        .arg("c")
        .arg("-");
    let _ = target.compile(assembly, command_mut, Stdio::piped());
}
