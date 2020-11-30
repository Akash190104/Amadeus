let Playerx : Type = ./types/player_type.dhall

let p = ./functions/p.dhall
let t = ./functions/t.dhall
let g = ./functions/g.dhall

let playersList : List Playerx =
  [ t "Fingon#2350"        361930230375514112 "Skyrimoon"
  , t "Punisher#24744"     442391445341732864 "punisherl_l"
  , p "Alucard#1389"       196922236895100928
  , p "Geriksmerik#2605"   444864857658097664
  ]

in playersList
