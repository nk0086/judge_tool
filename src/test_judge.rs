use anyhow::{ensure, Context, Result};
use colored::*;
use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Read};
use std::process::{Command, Stdio};

/// Run all the test cases in question and display the overall results.
pub fn test_judge(file_name: &str) -> Result<()> {
    let mut path = env::current_dir()?;
    path.push("test_cases");
    path.push(file_name);
    env::set_current_dir(&path).unwrap();

    let dir = fs::read_dir(path)?;

    let dir_number = dir.into_iter().collect::<Vec<_>>().len();
    println!("problem_ID: {}", file_name);
    for i in 1..dir_number / 2 + 1 {
        run(file_name, i)?;
        println!();
    }

    Ok(())
}

/// Execute the received test cases and display the results.
fn run(file_name: &str, num: usize) -> Result<()> {
    let input_case = "in_".to_string() + &num.to_string();
    let mut input_buf = Vec::new();
    let _ = File::open(input_case)?.read_to_end(&mut input_buf)?;

    let output_case = "out_".to_string() + &num.to_string();
    let mut answer = Vec::new();
    let _ = File::open(output_case)?.read_to_end(&mut answer)?;

    let mut child = Command::new("cargo")
        .args(["run", "--bin", file_name])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    child.stdin.as_mut().unwrap().write_all(&input_buf)?;

    let mut stdout = child.wait_with_output()?.stdout;
    stdout.pop();

    println!("test_case{}", num);
    let result = if stdout == answer { "AC" } else { "Not AC" };

    let answer: Vec<char> = answer.into_iter().map(|s| s as char).collect();
    let stdout: Vec<char> = stdout.into_iter().map(|s| s as char).collect();

    if result == "AC" {
        println!("status: {}", result.green());
    } else {
        println!("status: {}", result);
        println!("expected: {:?}", answer);
        println!("output: {:?}", stdout);
    }

    Ok(())
}
