let L = < English
        | Russian
        | Bilingual >
let Channel : Type =
  { id: Natural
  , lang: L }
let ru = λ(id: Natural) → { id = id, lang = L.Russian }
let en = λ(id: Natural) → { id = id, lang = L.English }
let bi = λ(id: Natural) → { id = id, lang = L.Bilingual }
let channels : List Channel =
  [ bi 611822932897038341 -- "main"
  , bi 773830849124106250 -- "english"
  , en 316390574808760322 -- "gym"
  , en 316391872970883072 -- "human"
  , en 316391919070740490 -- "nightelf"
  , en 316391887265071116 -- "orc"
  , en 316391902838521866 -- "undead"
  , bi 679723943535312903 -- "dating"
  , bi 611825511433240577 -- "warcraft"
  , bi 631379331515678720 -- "team-chat"
  , bi 695909020732620830 -- "division-2"
  , bi 666614389805416448 -- "🌈climatestrike"
  , bi 611824913829068800 -- "🇯🇵日本語"
  , ru 827151604053835807 -- nejit1
  , ru 827154134163390535 -- nejit2
  , ru 766697158245089310 -- korchma 1
  , ru 802227624964522054 -- korchma 2
  , ru 766759137760903200 -- korchma 3
  , ru 766697312659177503 -- korchma 4
  , ru 767500986465648641 -- korchma 5
  , ru 766750928757456926 -- korchma 6
  ]
in channels
