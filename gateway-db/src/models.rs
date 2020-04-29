use super::schema::{track, transportTicket};

#[derive(Identifiable, Queryable, PartialEq, Associations, Debug)]
#[table_name = "track"]
pub struct Track {
  pub id: i32,
  pub number: i32,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(Track, foreign_key = "trackId")]
#[table_name = "transportTicket"]
#[allow(non_snake_case)]
pub struct TransportTicket {
  pub id: i32,
  #[column_name = "trackId"]
  pub track_id: Option<i32>,
}
