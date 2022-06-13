// Copyright (C) 2022 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::ffi::OsStr;
use std::io::BufRead as _;
use std::io::BufReader;
use std::io::Error;
use std::io::ErrorKind;
use std::io::Result;
use std::process::Child;
use std::process::ChildStdout;
use std::process::Command;
use std::process::Stdio;

use diff_parse::File;


/// The `clang-format` used by default.
pub const CLANG_FORMAT: &str = "clang-format";


/// Wait for a child process to finish and map failures to an
/// appropriate error.
pub fn await_child<S>(program: S, child: Child) -> Result<Option<ChildStdout>>
where
  S: AsRef<OsStr>,
{
  let mut child = child;

  let status = child.wait()?;
  if !status.success() {
    let error = format!("process `{}` failed", program.as_ref().to_string_lossy());

    if let Some(stderr) = child.stderr {
      let mut stderr = BufReader::new(stderr);
      let mut line = String::new();

      // Let's try to include the first line of the error output in our
      // error, to at least give the user something.
      if stderr.read_line(&mut line).is_ok() {
        let line = line.trim();
        return Err(Error::new(ErrorKind::Other, format!("{error}: {line}")))
      }
    }
    return Err(Error::new(ErrorKind::Other, error))
  }
  Ok(child.stdout)
}


/// Invoke `clang-format` to format all the diff hunks.
pub fn format(diffs: &[(File, File)]) -> Result<()> {
  fn format_now(file: &str, lines_args: &[String]) -> Result<()> {
    let child = Command::new(CLANG_FORMAT)
      .arg("-i")
      .arg(file)
      .args(lines_args)
      .stdin(Stdio::null())
      .stdout(Stdio::null())
      .stderr(Stdio::piped())
      .spawn()?;
    let _ = await_child(CLANG_FORMAT, child)?;
    Ok(())
  }

  let mut last_dst = Option::<String>::None;
  let mut lines = Vec::new();

  for (_, dst) in diffs {
    match last_dst {
      Some(prev_dst) if prev_dst != *dst.file => {
        let () = format_now(&prev_dst, &lines)?;
        last_dst = Some(dst.file.to_string());
        lines.clear();
      },
      _ => (),
    }

    let start_line = dst.line;
    let end_line = dst.line + dst.count;
    lines.push(format!("--lines={start_line}:{end_line}"));

    if last_dst.is_none() {
      last_dst = Some(dst.file.to_string());
    }
  }

  if let Some(prev_dst) = last_dst {
    let () = format_now(&prev_dst, &lines)?;
  }
  Ok(())
}
