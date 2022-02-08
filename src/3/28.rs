#![allow(dead_code)]

enum Species {
  Crab,
  Octopus,
  Fish,
  Clam,
}

struct SeaCreature {
  species: Species,
  name: String,
  arms: i32,
  legs: i32,
  weapon: String,
}

fn main() {
  let ferris = SeaCreature {
    species: Species::Crab,
    name: String::from("Ferris"),
    arms: 2,
    legs: 4,
    weapon: String::from("craww"),
  };

  match ferris.species {
    Species::Crab => println! {"{} is a crab", ferris.name},
    Species::Octopus => println! {"{} is a octpus", ferris.name},
    Species::Fish => println! {"{} is a fish", ferris.name},
    Species::Clam => println! {"{} is a clam", ferris.name},
  }
}
