use chrono::prelude::*;
use crossterm::style::*;
use crossterm::{
  cursor, execute,
  terminal::{Clear, ClearType},
  QueueableCommand, Result,
};
use std::io::{stdout, Write, stdin, Read};
use std::thread;
use std::time;
use std::prelude::v1::*;
use std::iter::Iterator;
use getchar::getchar;

mod logger;

fn main() -> Result<()> {
  let mut dl = logger::Logger::new_logger("clock_run.log");
  dl.enable();
  execute!(stdout(), Clear(ClearType::All))?;
  std::thread::spawn(move || -> Result<()> {
    loop {
      stdout().queue(cursor::MoveTo(0, 0))?;
      stdout().flush()?;
      execute!(stdout(), Clear(ClearType::All))?;
      println!(
        "{}",
        "Console Clock written in Rust, by 27Onion".blue().bold()
      );
      println!();
      let the_time = Local::now();
      println!(
        "{}",
        format!(
          "{}:{:02}:{:02} {}",
          the_time.hour(),
          the_time.minute(),
          the_time.second(),
          the_time.date()
        )
        .green()
      );
      println!();
      println!();
      thread::sleep(time::Duration::from_millis(500));
    }
  });
  dl.log(&"[INFO] hi\n".to_string());
  loop {
    // let mut buf = String::new();
    // let ok = stdin().read_line(&mut buf).ok();
    // buf = buf.trim().to_string();
    // if ok != None {
    //   dl.log(&format!("[INFO] got it: {}\n", buf));
    //   if buf == "q" {
    //     break;
    //   }
    // } else {
    //   dl.log(&"[DEBUG] Failed to read line.\n".to_string())
    // }
    let ch = getchar().unwrap_or(' ');
    if ch == 'q' {
      break
    } else {
      dl.log(&format!("[DEBUG] Key stroke: '{}'\n", ch))
    }
  }
  Ok(())
}
