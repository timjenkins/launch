use std::process::{ Command, Output };

// ====================
// TMUX command methods
// ====================
pub fn cmd_start_session(name: &String) -> Output {
    Command::new("tmux")
        .arg("new")
        .arg("-d")
        .arg("-s")
        .arg(&name)
        .arg("-n")
        .arg("delete_me")
        .output()
        .expect("Error: Failed to create session")
}

pub fn cmd_new_window(name: &String) -> Output {
    Command::new("tmux")
        .arg("new-window")
        .arg("-n")
        .arg(&name)
        .output()
        .expect("Error: Failed to create window")
}

pub fn cmd_split_window() -> Output {
    Command::new("tmux").arg("split-window").output().expect("Failed to create new pane")
}

pub fn cmd_set_dir(target: &String, root_dir: &String) -> Output {
    // tmux send-keys -t #{target} \"cd #{root_dir}\" Enter
    Command::new("tmux")
        .arg("send-keys")
        .arg("-t")
        .arg(&format!("{}", target))
        .arg(&format!("cd {}", root_dir))
        .arg(&format!("Enter"))
        .output()
        .expect(&format!("Failed to set root directory for {target}"))
}

pub fn cmd_run_cmd(target: &String, cmd: &String) -> Output {
    // tmux send-keys -t #{target} \"#{cmd}\" Enter
    Command::new("tmux")
        .arg("send-keys")
        .arg("-t")
        .arg(&format!("{}", target))
        .arg(&format!("{}", cmd))
        .arg(&format!("Enter"))
        .output()
        .expect(&format!("Failed to send command {cmd} to {target}"))
}

pub fn cmd_cleanup(session_name: &String) -> Output {
    // tmux kill-window -t #{session_name}:delete_me
    Command::new("tmux")
        .arg("kill-window")
        .arg("-t")
        .arg(&format!("{}:delete_me", session_name))
        .output()
        .expect("Cleanup failed, there is an extra tmux window called 'delete_me'")
}

pub fn cmd_attach() -> () {
    let mut command = Command::new("tmux");
    if let Ok(mut child) = command.arg("attach").spawn() {
        child.wait().expect("Failed to attach to new session");
    }
}
