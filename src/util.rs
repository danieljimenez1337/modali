use std::process::Command;

pub fn run_command_detached(cmd: String) {
    Command::new("sh")
        .arg("-c")
        .arg(format!(
            "nohup sh -c 'exec {}' >/dev/null 2>&1 &",
            cmd.replace("'", "'\"'\"'")
        ))
        .spawn()
        .expect("Failed to start command")
        .wait()
        .expect("Failed to wait for intermediate process");
}
