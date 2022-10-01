use std::process::Command;

fn add_to_terminal_startup(rc_file_path: &str) {
    let startup_command = "if [ -x \"$(command -v rlrn)\" ]; then\n  rlrn learn\nfi\n";

    // check if command is already in rc file
    let output = Command::new("sh")
        .arg("-c")
        .arg(format!(
            "cat {} | grep -q 'rlrn' && echo 'true' || echo 'false'",
            rc_file_path
        ))
        .output()
        .unwrap_or_else(|_| {
            panic!(
                "Failed to check if rlrn is already added to terminal startup script ({})",
                rc_file_path
            )
        });
    let output = String::from_utf8(output.stdout).unwrap();
    let is_already_added = output.trim() == "true";

    if !is_already_added {
        Command::new("sh")
            .arg("-c")
            .arg(format!("echo '{}' >> {} ", startup_command, rc_file_path))
            .output()
            .unwrap_or_else(|_| {
                panic!(
                    "Failed to add rlrn to terminal startup script ({})",
                    rc_file_path
                )
            });
    }
}

/// Will be used in some "uninit" command in the future
fn _remove_from_terminal_startup(rc_file_name: &str) {
    Command::new("sh")
        .arg("-c")
        .arg(format!(
            "sed -i '/command -v rlrn/,+2d' ~/{} ",
            rc_file_name
        ))
        .output()
        .unwrap_or_else(|_| {
            panic!(
                "Failed to remove from terminal startup script ({})",
                rc_file_name
            )
        });
}

fn check_if_file_exist(file_path: &str) -> bool {
    let output = Command::new("sh")
        .arg("-c")
        .arg(format!("test -f {} && echo '1' || echo '0'", file_path))
        .output()
        .expect("Failed to check if file exist");

    let output = String::from_utf8(output.stdout).unwrap();

    return output.trim() == "1";
}

/// add rlrn to terminal startup scripts
/// (e.g. .bashrc, .zshrc)
/// it will not add if it is already added
pub fn add_rlrn_to_existing_rc_files() {
    let supported_rc_files = vec![".bashrc", ".zshrc"];

    for rc_file in supported_rc_files {
        let rc_file_path = format!("~/{}", rc_file);
        if check_if_file_exist(&rc_file_path) {
            add_to_terminal_startup(&rc_file_path);
        }
    }
}
