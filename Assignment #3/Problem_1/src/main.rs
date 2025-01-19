use nix::errno::Errno;
use nix::sys::wait::{waitpid, WaitStatus};
use nix::unistd::{close, dup2, execvp, fork, pipe, ForkResult, Pid};
use std::ffi::CString;
use std::fs::File;
use std::io::{self, Write};
use std::os::fd::IntoRawFd;
use std::process::exit;

fn main() {
    println!("######### oh-my-shell starts! #########");
    loop {
        print!(">>> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        if input == "exit" {
            break;
        }

        let commands: Vec<&str> = input.split_whitespace().collect();

        handle_piping(commands);
        println!();
    }
    println!("Exit oh-my-shell. Bye!");
}

fn handle_piping(commands: Vec<&str>) {
    let mut commands = commands
        .split(|&arg| arg == "|")
        .map(|cmd| cmd.to_vec())
        .collect::<Vec<_>>();
    let mut child_processes = Vec::new();
    let mut last_process: (Pid, i32) = (Pid::from_raw(-1), -1);
    let num_commands = commands.len();

    let mut pipes = Vec::new();
    for _ in 0..num_commands - 1 {
        let (pipe_read, pipe_write) = pipe().unwrap_or_else(|err| {
            eprintln!("pipe failed: {}", err);
            exit(Errno::last_raw());
        });
        pipes.push((pipe_read.into_raw_fd(), pipe_write.into_raw_fd()));
    }

    for (i, command) in commands.iter_mut().enumerate() {
        match unsafe { fork() } {
            Ok(ForkResult::Parent { child }) => {
                if i > 0 {
                    let _ = close(pipes[i - 1].0);
                    let _ = close(pipes[i - 1].1);
                }
                if i < pipes.len() {
                    let _ = close(pipes[i].1);
                }

                if i == num_commands - 1 {
                    if let Ok(WaitStatus::Exited(child, status)) = waitpid(child, None) {
                        last_process = (child, status);
                    }
                }
                child_processes.push(child);
            }
            Ok(ForkResult::Child) => {
                if i > 0 {
                    let _ = dup2(pipes[i - 1].0, 0);
                    let _ = close(pipes[i - 1].0);
                    let _ = close(pipes[i - 1].1);
                }

                if i < pipes.len() {
                    let _ = dup2(pipes[i].1, 1);
                    let _ = close(pipes[i].0);
                    let _ = close(pipes[i].1);
                }

                handle_redirection(command);
            }
            Err(err) => {
                eprintln!("fork failed: {}", err);
                exit(-1);
            }
        }
    }

    for pid in child_processes {
        if let Ok(WaitStatus::Exited(pid, status)) = waitpid(pid, None) {
            println!(
                "[oh-my-shell] Child process terminated: pid {}, status {}",
                pid, status
            );
        }
    }
    println!(
        "[oh-my-shell] Child process terminated: pid {}, status {}",
        last_process.0, last_process.1
    );
}

fn handle_redirection(redirection_command: &mut Vec<&str>) {
    let mut command = redirection_command.clone();
    if let Some(pos) = redirection_command.iter().position(|&arg| arg == ">") {
        command = redirection_command[..pos].to_vec();
        let output_file = redirection_command[pos + 1];
        let fd = File::create(output_file)
            .unwrap_or_else(|err| {
                eprintln!("open failed: {}", err);
                exit(Errno::last_raw());
            })
            .into_raw_fd();
        let _ = dup2(fd, 1); // Redirect stdout to the file
        let _ = close(fd);
    }

    if let Some(pos) = redirection_command.iter().position(|&arg| arg == "<") {
        command = redirection_command[..pos].to_vec();
        let input_file = redirection_command[pos + 1];
        let fd = File::open(input_file)
            .unwrap_or_else(|err| {
                eprintln!("open failed: {}", err);
                exit(Errno::last_raw());
            })
            .into_raw_fd();
        let _ = dup2(fd, 0); // Redirect stdin to the file
        let _ = close(fd);
    }
    exec_command(command);
}

fn exec_command(command: Vec<&str>) {
    let filename = CString::new(command[0]).unwrap();
    let args = command
        .iter()
        .map(|&arg| CString::new(arg).unwrap())
        .collect::<Vec<_>>();

    let _ = execvp(&filename, &args);
    exit(Errno::last_raw());
}
