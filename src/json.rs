/*
make json: jsonファイルの作成
read json: jsonコマンドの読み込み
 */
use serde_json::Value;
use std::fs::OpenOptions;
use std::io::BufReader;

pub fn read_json() -> Value {
    let file = OpenOptions::new().read(true).open("command.json").unwrap();
    let reader = BufReader::new(file);
    let command: Value = serde_json::from_reader(reader).unwrap();
    //println!("{:?}", command["default"]);
    command
}
