pub_struct!(Stats {
  race: u32,
  gateWay: u32,
  id: String,
  wins: u32,
  losses: u32,
  games: u32,
  winrate: f64,
});

pub_struct!(WinLosses {
  race: u32,
  wins: u32,
  losses: u32,
  games: u32,
  winrate: f64,
});

pub_struct!(WinLossesOnMap {
  map: String,
  winLosses: Vec<WinLosses>,
});

pub_struct!(RaceWinsOnMap {
  race: u32,
  winLossesOnMap: Vec<WinLossesOnMap>,
});

pub_struct!(Stats2 {
  id: String,
  raceWinsOnMap: Vec<RaceWinsOnMap>,
  battleTag: String,
  season: u32,
});

pub_struct!(PlayerId {
  name: String,
  battleTag: String,
});

pub_struct!(Player {
  playerIds: Vec<PlayerId>,
  name: String,
  id: String,
  mmr: u32,
  gateWay: u32,
  gameMode: u32,
  season: u32,
  wins: u32,
  losses: u32,
  games: u32,
  winrate: f64,
});

pub_struct!(RankingPointsProgress {
  rankingPoints: u32,
  mmr: u32,
});

pub_struct!(Search {
  gateway: u32,
  id: String,
  league: u32,
  rankNumber: u32,
  rankingPoints: u32,
  playerId: String,
  player: Player,
  gameMode: u32,
  season: u32,
});

pub_struct!(GMStats {
  division: u32,
  gameMode: u32,
  games: u32,
  gateWay: u32,
  id: String,
  leagueId: u32,
  leagueOrder: u32,
  losses: u32,
  mmr: u32,
  playerIds: Vec<PlayerId>,
  rank: u32,
  rankingPoints: u32,
  rankingPointsProgress: RankingPointsProgress,
  season: u32,
  winrate: f64,
  wins: u32,
});

pub_struct!(TeamPlayer {
  race: u32,
  oldMmr: u32,
  currentMmr: u32,
  battleTag: String,
  name: String,
  mmrGain: i32,
  won: bool,
});

pub_struct!(Team {
  players: Vec<TeamPlayer>,
  won: bool,
});

pub_struct!(Match {
  map: String,
  id: String,
  durationInSeconds: u32,
  startTime: String,
  endTime: String,
  gameMode: u32,
  teams: Vec<Team>,
  gateWay: u32,
  season: u32,
});

pub_struct!(Going {
  matches: Vec<Match>,
  count: u32,
});
