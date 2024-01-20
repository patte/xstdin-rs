#[cfg(test)]
mod tests {
    use rstest::rstest;
    use std::io::Write;
    use std::process::{Command, Stdio};

    #[rstest]
    #[case::default(&[])]
    #[case::line_mode(&["-l"])]
    #[case::small_buffer(&["-b8"])]
    #[case::small_buffer_line_mode(&["-b8", "-l"])]
    fn test_execution(#[case] args: &[&str]) {
        let mut child = Command::new("cargo")
            .arg("run")
            .arg("--")
            .args(args)
            .arg("cat")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("failed to execute process");

        {
            let stdin = child.stdin.as_mut().expect("failed to get stdin");
            stdin
                .write_all(b"1\n2\n3\n4\n")
                .expect("failed to write to stdin");
        }

        let output = child.wait_with_output().expect("failed to wait on child");

        let output = String::from_utf8(output.stdout).expect("output is not UTF-8");
        let mut lines: Vec<_> = output.lines().collect();
        lines.sort(); // Sorting because output order might not be guaranteed

        assert_eq!(lines, vec!["1", "2", "3", "4"]);
    }

    #[rstest]
    #[case::default(&[])]
    #[case::line_mode(&["-l"])]
    #[case::small_buffer(&["-b8"])]
    #[case::small_buffer_line_mode(&["-b8", "-l"])]
    fn test_concurrency(#[case] args: &[&str]) {
        let start = std::time::Instant::now();

        let mut child = Command::new("cargo")
            .arg("run")
            .arg("--")
            .args(args)
            .arg("-n")
            .arg("2") // 2 workers
            .arg("--")
            .arg("sh")
            .arg("-c")
            .arg("sleep 1; echo done")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("failed to execute process");

        {
            let stdin = child.stdin.as_mut().expect("failed to get stdin");
            stdin
                .write_all(b"1\n2\n")
                .expect("failed to write to stdin");
        }

        let output = child.wait_with_output().expect("failed to wait on child");

        let duration = start.elapsed();

        let output = String::from_utf8(output.stdout).expect("output is not UTF-8");
        assert_eq!(output.lines().collect::<Vec<_>>(), vec!["done", "done"]);

        // Check if the total time is more than one time the sleep time
        assert!(duration > std::time::Duration::from_secs(1));

        // Check if the total time is less than double the sleep time, indicating concurrency
        assert!(duration < std::time::Duration::from_secs(2));
    }

    #[rstest]
    #[case::default(&[])]
    #[case::line_mode(&["-l"])]
    #[case::small_buffer(&["-b8"])]
    #[case::small_buffer_line_mode(&["-b8", "-l"])]
    fn test_large_input(#[case] args: &[&str]) {
        let num_lines = 10000;
        let input = (0..num_lines)
            .map(|i| i.to_string())
            .collect::<Vec<_>>()
            .join("\n");

        let mut child = Command::new("cargo")
            .arg("run")
            .arg("--")
            .args(args)
            .arg("cat")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("failed to execute process");

        {
            let stdin = child.stdin.as_mut().expect("failed to get stdin");
            stdin
                .write_all(input.as_bytes())
                .expect("failed to write to stdin");
        }

        let output = child.wait_with_output().expect("failed to wait on child");
        let output = String::from_utf8(output.stdout).expect("output is not UTF-8");
        let mut output_lines: Vec<_> = output.lines().collect();
        output_lines.sort(); // Sorting because order is not guaranteed
        println!(
            "last 3 output_lines: {:?}",
            output_lines[(output_lines.len() - 3)..output_lines.len()].to_vec()
        );
        assert_eq!(output_lines.len(), num_lines);
    }

    #[rstest]
    fn test_worker_id_substitution() {
        let mut child = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg("-n")
            .arg("2") // 2 workers
            .arg("echo")
            .arg("worker {}")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("failed to execute process");

        {
            let stdin = child.stdin.as_mut().expect("failed to get stdin");
            stdin
                .write_all(b"1\n2\n3\n4\n")
                .expect("failed to write to stdin");
        }

        let output = child.wait_with_output().expect("failed to wait on child");
        let output = String::from_utf8(output.stdout).expect("output is not UTF-8");
        let mut output_lines: Vec<_> = output.lines().collect();
        output_lines.sort(); // Sorting because output order might not be guaranteed

        assert_eq!(output_lines, vec!["worker 0", "worker 1"]);
    }
}
