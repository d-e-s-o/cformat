// Copyright (C) 2022 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::io::stdin;
use std::io::Result;

use cformat::format;
use diff_parse::Parser;

fn main() -> Result<()> {
  let mut parser = Parser::new();
  parser.parse(stdin().lock())?;

  // TODO: We may want to catch BrokenPipe errors here and exit
  //       gracefully.
  format(parser.diffs())
}
