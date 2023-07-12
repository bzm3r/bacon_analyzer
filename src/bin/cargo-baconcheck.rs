use std::{
    io::{self, Write},
    process::{Command, Output},
};

struct CargoCmd {
    cmd: Command,
}

impl From<Command> for CargoCmd {
    fn from(value: Command) -> CargoCmd {
        CargoCmd { cmd: value }
    }
}

impl CargoCmd {
    fn output(&mut self, panic_msg: &'static str) -> Output {
        self.cmd.output().expect(panic_msg)
    }
}

fn new_cargo_cmd(args: &[&'static str]) -> CargoCmd {
    if cfg!(target_os = "windows") {
        let mut cmd = Command::new("cargo");
        cmd.args(args);
        cmd.into()
    } else {
        panic!("Windows is currently the only supported platform");
    }
}

fn main() {
    let check_args = vec!["check", "--workspace", "--all-targets"];

    let check_with_json_args = {
        let mut args = check_args.clone();
        args.append(&mut vec!["--quiet", "--message-format=json"]);
        args
    };

    let (mut check_cmd, mut json_out_cmd) = (
        new_cargo_cmd(&check_args),
        new_cargo_cmd(&check_with_json_args),
    );

    let check_output = check_cmd.output("baconcheck: failed to run cargo check");

    let mut std_out = io::stdout();

    std_out
        .write_all(&check_output.stdout)
        .expect("baconcheck: error printing out check's stdout");

    std::fs::write(
        "json.out",
        json_out_cmd
            .output("baconcheck: failed to run cargo check with json")
            .stdout,
    )
    .map_err(|err| panic!("baconcheck: file write error: {}", err))
    .unwrap();

    io::stderr()
        .write_all(&check_output.stderr)
        .expect("baconcheck: error printing out check's stderr");
}
