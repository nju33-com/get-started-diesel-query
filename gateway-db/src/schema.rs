table! {
  ecogreen.track (id) {
    id -> Integer,
    number -> Integer,
  }
}

table! {
  #[allow(non_snake_case)]
  ecogreen.transportTicket (id) {
    id -> Integer,
    trackId -> Nullable<Integer>,
  }
}
