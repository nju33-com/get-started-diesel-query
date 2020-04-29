use anyhow::Result;
use gateway_db::reset_tickets_by_track_number;
use std::env::args;

fn main() -> Result<()> {
  let argv: Vec<String> = args().collect();
  let connection_string = &argv[1];
  let track_number = &argv[2];
  let track_number: i32 = track_number.parse()?;

  reset_tickets_by_track_number(&connection_string, &track_number)?;

  Ok(())
}
