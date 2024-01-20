use getopts::Options;
use std::env;
use std::io::{self, BufRead, Write};
use std::process::{Command, Stdio};

fn print_usage(program: &str, opts: &Options) {
    let brief = format!("Usage: {} [-n NUM] <command> [<arg1> <arg2> ...]", program);
    print!("{}", opts.usage(&brief));
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    // Parse command line options
    let mut opts = Options::new();
    opts.optopt("n", "", "set number of workers (default is 4)", "NUM");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!("{}", f),
    };

    if matches.opt_present("h") {
        print_usage(&args[0], &opts);
        return Ok(());
    }

    let num_workers = matches.opt_get_default("n", 4).unwrap();
    if matches.free.is_empty() {
        eprintln!("Error: No command specified.");
        print_usage(&args[0], &opts);
        return Err(io::Error::new(io::ErrorKind::Other, "No command specified"));
    }

    let (command, command_args) = matches.free.split_at(1);

    // Spawn worker subprocesses and create pipes
    let mut children = Vec::new();
    let mut stdin_pipes = Vec::new();
    for _ in 0..num_workers {
        let mut child = Command::new(&command[0])
            .args(command_args)
            .stdin(Stdio::piped())
            .spawn()
            .expect("Failed to spawn child process");

        let stdin_pipe = child.stdin.take().expect("Failed to open stdin");
        children.push(child);
        stdin_pipes.push(stdin_pipe);
    }

    // Distribute work to workers
    let mut line_counter = 0;
    let input = io::stdin(); // Use stdin as input
    for line in input.lock().lines() {
        let line = line?;
        writeln!(stdin_pipes[line_counter % num_workers], "{}", line)
            .expect("Failed to write to stdin of child process");
        line_counter += 1;
    }

    // Close the pipes to signal the end of input
    drop(stdin_pipes);

    // Wait for all subprocesses to finish
    for mut child in children {
        let ecode = child.wait().expect("Failed to wait on child");
        assert!(ecode.success(), "Command failed to run successfully");
    }

    Ok(())
}
