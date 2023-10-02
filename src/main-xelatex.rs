use std::io::{self, BufRead, BufReader};
use std::process::{Command, Stdio};

async fn run_xelatex() -> io::Result<String> {
    let mut cmd = Command::new("xelatex")
        //.arg("-interaction=nonstopmode")
        .arg("/Users/xiaoqiangjiang/Nutstore/document/dolphin-book-2023/src/dolphin-book-2023.tex")
        .stdout(Stdio::piped())
        .current_dir("/Users/xiaoqiangjiang/Nutstore/document/dolphin-book-2023/src")
        .spawn()?;
    
    let stdout = cmd.stdout.take().unwrap();
    let reader = BufReader::new(stdout);
    
    tokio::spawn(async move {
        for line in reader.lines() {
            if let Ok(line) = line {
                println!("llll{}", line);
            }
        }
    });
    
    let status = cmd.wait()?;
    
    if status.success() {
        Ok("Compilation successful".to_string())
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "Compilation failed"))
    }
}

#[tokio::main]
async fn main() {
    let result = run_xelatex().await;
    
    match result {
        Ok(message) => println!("{}", message),
        Err(err) => eprintln!("Error: {}", err),
    }
}