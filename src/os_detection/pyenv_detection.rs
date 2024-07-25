// libraries
use std::process::Command;
use std::io;

// modules


pub fn install_pyenv(os: String) -> bool
{
    if detect_pyenv() {
        println!("pyenv is already installed");
        return true;
    }

    if os == "Windows" {
        return install_pyenv_windows();
    } else if os == "Linux" {
        return install_pyenv_debian();
    } else {
        return false;
    }
}


// install pyenv-win on windows
// src: https://github.com/pyenv-win/pyenv-win
fn install_pyenv_windows() -> bool
{
    let output = Command::new("powershell")
        .arg("-Command")
        .arg("Invoke-WebRequest -UseBasicParsing -Uri \"https://raw.githubusercontent.com/pyenv-win/pyenv-win/master/pyenv-win/install-pyenv-win.ps1\" -OutFile \"./install-pyenv-win.ps1\"; &\"./install-pyenv-win.ps1\"")
        .output()
        .expect("Failed to execute command");
    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    if output.status.success() {
        true
    } else {
        false
    }

}

// install pyenv on debian based systems
fn install_pyenv_debian() -> bool
{
    let output = Command::new("bash")
        .arg("-c")
        .arg("curl https://pyenv.run | bash")
        .output()
        .expect("Failed to execute command");
    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    if output.status.success() {
        true
    } else {
        false
    }
}


// is pyenv installed?
fn detect_pyenv() -> bool {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "pyenv --version"])
            .output()
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("pyenv --version")
            .output()
    };

    match output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            l//et stderr = String::from_utf8_lossy(&output.stderr);

            println!("stdout: {}", stdout);
            //println!("stderr: {}", stderr);

            if stdout.contains("pyenv") {
                return true;
            }
            false
        }
        Err(err) => {
            println!("Failed to execute command: {:?}", err);
            false
        }
    }
}