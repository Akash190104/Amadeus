let Playerx : Type = ./types/player_type.dhall

let p = ./functions/p.dhall
let t = ./functions/t.dhall
let g = ./functions/g.dhall

let playersList : List Playerx =
  [ t "Fingon#2350"        361930230375514112 "Skyrimoon"
  , t "SMDVKF#2721"        632300213494611968 "siriustvzzz"
  , p "Alucard#1389"       196922236895100928
  ]

in playersList
