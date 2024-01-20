use getopts::Options;
use std::env;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
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
    opts.optopt("b", "", "set buffer capacity (default is 8KiB)", "SIZE");
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
    let mut stdin_writers = Vec::new();
    for _ in 0..num_workers {
        let mut child = Command::new(&command[0])
            .args(command_args)
            .stdin(Stdio::piped())
            .spawn()?;

        let stdin_pipe = child
            .stdin
            .take()
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Failed to open stdin"))?;
        let stdin_writer = BufWriter::new(stdin_pipe);
        children.push(child);
        stdin_writers.push(stdin_writer);
    }

    // Distribute work to workers
    let input = io::stdin(); // Use stdin as input
    let buffered_input =
        BufReader::with_capacity(matches.opt_get_default("b", 8192).unwrap(), input.lock());
    let mut line_counter = 0;
    for line in buffered_input.lines() {
        let line = line?;
        writeln!(stdin_writers[line_counter % num_workers], "{}", line)?;
        line_counter += 1;
    }

    // Close the pipes to signal the end of input
    drop(stdin_writers);

    // Wait for all subprocesses to finish
    for mut child in children {
        let ecode = child.wait().expect("Failed to wait on child");
        assert!(ecode.success(), "Command failed to run successfully");
    }

    Ok(())
}
