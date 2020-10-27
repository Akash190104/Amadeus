use regex::Regex;

pub fn get_race(r : u32) -> String {
  String::from(
    match r { 1 => "Human"
            , 2 => "Orc"
            , 4 => "Night Elf"
            , 8 => "Undead"
            , _ => "Random" })
}

pub fn get_race2(r : u32) -> String {
  String::from(
    match r { 1 => "H"
            , 2 => "O"
            , 4 => "NE"
            , 8 => "UD"
            , _ => "RND"})
}

pub fn get_league(l: u32) -> String {
  String::from(match l { 0 => "GrandMaster"
                       , 1 => "Master"
                       , 2 => "Diamond"
                       , 3 => "Platinum"
                       , 4 => "Gold"
                       , 5 => "Silver"
                       , 6 => "Bronze"
                       , _ => "" })
}

fn try_get_map(m: &str) -> String {
  String::from(
    match m { "Overall"               => "All"
            , "echoisles"             => "EI"
            , "northernisles"         => "NIS"
            , "amazonia"              => "AZ"
            , "lastrefuge"            => "LR"
            , "concealedhill"         => "CH"
            , "twistedmeadows"        => "TM"
            , "terenasstand"          => "TS"
            , "autumnleaves"          => "AL"
            , "avalanche"             => "AVLV"
            , "losttemple"            => "LT"
            , "turtlerock"            => "TR"
            , "ruinsofazshara"        => "ROA"
            , "gnollwood"             => "Gnoll Wood"
            , "hillsbradcreek"        => "Hillsbrad Creek"
            , "goldshire"             => "Goldshire"
            , "tidewaterglades"       => "Tide Water Glades"
            , "circleoffallenheroes"  => "Circle of Hate"
            , "phantomgrove"          => "Phantom Grove"
            , _                       => "" })
}

pub fn get_map(m: &str) -> String {
  let map_regex = Regex::new(
    r"^(?:_)?(?:[0-4]{1}v[0-4]{1}_)?([A-z._']+?)(?:[0-9]+)?(?:_lv)?(?:_|\.)?(?:anon)?(?:_|\.)?(?:w3c|w3x)?$")
      .unwrap();
  let mut map = String::new();
  if let Some(caps) = map_regex.captures(m) {
    if let Some(group1) = caps.get(1) {
      map = try_get_map(group1.as_str());
    }
  }
  if map.is_empty() { m.to_string() } else { map }
}

pub fn get_league_png(lid: u32) -> String {
  let base = "https://github.com/w3champions/w3champions-ui/raw/master/src/assets/leagueFlags/";
  format!("{}{}.png", base
    , match lid { 0 => "grandmaster"
                , 1 => "master"
                , 2 => "diamond"
                , 3 => "platinum"
                , 4 => "gold"
                , 5 => "silver"
                , 6 => "bronze"
                , _ => "unranked" })
}

pub fn get_hero_png(hero: &str) -> String {
  let base = "https://github.com/w3champions/w3champions-ui/raw/master/src/assets/heroes/";
  format!("{}{}.png", base, hero)
}

#[cfg(test)]
mod cyber_utils_tests {
  use super::*;
  #[test]
  fn get_map_test() {
    assert_eq!(get_map("echoisles"), "EI");
    assert_eq!(get_map("autumnleaves201016"), "AL");
    assert_eq!(get_map("_1v1_autumnleaves_anon"), "AL");
    assert_eq!(get_map("_gnollwood_anon"), "Gnoll Wood");
  }
}
