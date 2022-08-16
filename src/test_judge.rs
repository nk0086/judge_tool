use crate::json::read_json;
use anyhow::Result;
use colored::*;
use serde_json::Value;
use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::Read;
use std::path::PathBuf;
use std::process::{Command, Stdio};

/// Run all the test cases in question and display the overall results.
pub fn test_judge(file_name: &str, extensions: &str) -> Result<()> {
    let command_json = read_json();

    let problem_id = file_name.split("_").collect::<Vec<_>>();
    let mut path = env::current_dir()?;
    path.push("test_cases");
    path.push(problem_id[0]);
    path.push(problem_id[1]);

    //println!("{:?}", path);
    let dir = fs::read_dir(&path)?;
    let dir_number = dir.into_iter().collect::<Vec<_>>().len();
    println!("problem_ID: {}", file_name);
    for i in 1..dir_number / 2 + 1 {
        run(file_name, i, &command_json, &path, extensions)?;
        println!();
    }

    Ok(())
}

/// Execute the received test cases and display the results.
fn run(
    file_name: &str,
    num: usize,
    command_json: &Value,
    path: &PathBuf,
    extensions: &str,
) -> Result<()> {
    let mut input_case = path.clone();
    input_case.push(format!("in_{}", num.to_string()));
    let mut input_buf = Vec::new();
    let _ = File::open(input_case)?.read_to_end(&mut input_buf)?;

    let mut output_case = path.clone();
    output_case.push(format!("out_{}", num.to_string()));
    let mut answer = Vec::new();
    let _ = File::open(output_case)?.read_to_end(&mut answer)?;

    let extension = command_json[extensions]["extensions"].as_str().unwrap();
    let tmp = format!("{}.{}", file_name, extension);
    let file_name = if extensions != "rs" { &tmp } else { file_name };

    // read compile command from command.json
    let command: &str = command_json[extensions]["command"].as_str().unwrap();
    let args = match command_json[extensions]["args"].as_array() {
        Some(k) => {
            let mut k = k
                .into_iter()
                .map(|x| x.as_str().unwrap())
                .collect::<Vec<&str>>();
            k.push(file_name);
            k
        }
        None => vec![file_name],
    };

    // jsonからファイル読み込んでコンパイルする
    let mut child = Command::new(command)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    child.stdin.as_mut().unwrap().write_all(&input_buf)?;

    let mut stdout = child.wait_with_output()?.stdout;
    stdout.pop();

    // Test result
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
