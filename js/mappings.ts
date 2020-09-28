const items: { [key: string]: string } = {
  amrc: "i_Amulet of Recall",
  ankh: "i_Ankh of Reincarnation",
  belv: "i_Boots of Quel'Thalas +6",
  bgst: "i_Belt of Giant Strength +6",
  bspd: "i_Boots of Speed",
  ccmd: "i_Scepter of Mastery",
  ciri: "i_Robe of the Magi +6",
  ckng: "i_Crown of Kings +5",
  clsd: "i_Cloak of Shadows",
  crys: "i_Crystal Ball",
  desc: "i_Kelen's Dagger of Escape",
  gemt: "i_Gem of True Seeing",
  gobm: "i_Goblin Land Mines",
  gsou: "i_Soul Gem",
  guvi: "i_Glyph of Ultravision",
  gfor: "i_Glyph of Fortification",
  soul: "i_Soul",
  mdpb: "i_Medusa Pebble",
  rag1: "i_Slippers of Agility +3",
  rat3: "i_Claws of Attack +3",
  rin1: "i_Mantle of Intelligence +3",
  rde1: "i_Ring of Protection +2",
  rde2: "i_Ring of Protection +3",
  rde3: "i_Ring of Protection +4",
  rhth: "i_Khadgar's Gem of Health",
  rst1: "i_Gauntlets of Ogre Strength +3",
  ofir: "i_Orb of Fire",
  ofro: "i_Orb of Frost",
  olig: "i_Orb of Lightning",
  oli2: "i_Orb of Lightning",
  oven: "i_Orb of Venom",
  odef: "i_Orb of Darkness",
  ocor: "i_Orb of Corruption",
  pdiv: "i_Potion of Divinity",
  phea: "i_Potion of Healing",
  pghe: "i_Potion of Greater Healing",
  pinv: "i_Potion of Invisibility",
  pgin: "i_Potion of Greater Invisibility",
  pman: "i_Potion of Mana",
  pgma: "i_Potion of Greater Mana",
  pnvu: "i_Potion of Invulnerability",
  pnvl: "i_Potion of Lesser Invulnerability",
  pres: "i_Potion of Restoration",
  pspd: "i_Potion of Speed",
  rlif: "i_Ring of Regeneration",
  rwiz: "i_Sobi Mask",
  sfog: "i_Horn of the Clouds",
  shea: "i_Scroll of Healing",
  sman: "i_Scroll of Mana",
  spro: "i_Scroll of Protection",
  sres: "i_Scroll of Restoration",
  ssil: "i_Staff of Silence",
  stwp: "i_Scroll of Town Portal",
  tels: "i_Goblin Night Scope",
  tdex: "i_Tome of Agility",
  texp: "i_Tome of Experience",
  tint: "i_Tome of Intelligence",
  tkno: "i_Tome of Power",
  tstr: "i_Tome of Strength",
  ward: "i_Warsong Battle Drums",
  will: "i_Wand of Illusion",
  wneg: "i_Wand of Negation",
  rdis: "i_Rune of Dispel Magic",
  rwat: "i_Rune of the Watcher",
  fgrd: "i_Red Drake Egg",
  fgrg: "i_Stone Token",
  fgdg: "i_Demonic Figurine",
  fgfh: "i_Spiked Collar",
  fgsk: "i_Book of the Dead",
  engs: "i_Enchanted Gemstone",
  k3m1: "i_Mooncrystal",
  modt: "i_Mask of Death",
  sand: "i_Scroll of Animate Dead",
  srrc: "i_Scroll of Resurrection",
  sror: "i_Scroll of the Beast",
  infs: "i_Inferno Stone",
  shar: "i_Ice Shard",
  wild: "i_Amulet of the Wild",
  wswd: "i_Sentry Wards",
  whwd: "i_Healing Wards",
  wlsd: "i_Wand of Lightning Shield",
  wcyc: "i_Wand of the Wind",
  rnec: "i_Rod of Necromancy",
  pams: "i_Anti-magic Potion",
  clfm: "i_Cloak of Flames",
  evtl: "i_Talisman of Evasion",
  nspi: "i_Necklace of Spell Immunity",
  lhst: "i_The Lion Horn of Stormwind",
  kpin: "i_Khadgar's Pipe of Insight",
  sbch: "i_Scourge Bone Chimes",
  afac: "i_Alleria's Flute of Accuracy",
  ajen: "i_Ancient Janggo of Endurance",
  lgdh: "i_Legion Doom-Horn",
  hcun: "i_Hood of Cunning",
  mcou: "i_Medallion of Courage",
  hval: "i_Helm of Valor",
  cnob: "i_Circlet of Nobility",
  prvt: "i_Periapt of Vitality",
  tgxp: "i_Tome of Greater Experience",
  mnst: "i_Mana Stone",
  hlst: "i_Health Stone",
  tpow: "i_Tome of Knowledge",
  tst2: "i_Tome of Strength +2",
  tin2: "i_Tome of Intelligence +2",
  tdx2: "i_Tome of Agility +2",
  rde0: "i_Ring of Protection +1",
  rde4: "i_Ring of Protection +5",
  rat6: "i_Claws of Attack +6",
  rat9: "i_Claws of Attack +9",
  ratc: "i_Claws of Attack +12",
  ratf: "i_Claws of Attack +15",
  manh: "i_Manual of Health",
  pmna: "i_Pendant of Mana",
  penr: "i_Pendant of Energy",
  gcel: "i_Gloves of Haste",
  totw: "i_Talisman of the Wild",
  phlt: "i_Phat Lewt",
  gopr: "i_Glyph of Purification",
  ches: "i_Cheese",
  mlst: "i_Maul of Strength",
  rnsp: "i_Ring of Superiority",
  brag: "i_Bracer of Agility",
  sksh: "i_Skull Shield",
  vddl: "i_Voodoo Doll",
  sprn: "i_Spider Ring",
  tmmt: "i_Totem of Might",
  anfg: "i_Ancient Figurine",
  lnrn: "i_Lion's Ring",
  iwbr: "i_Ironwood Branch",
  jdrn: "i_Jade Ring",
  drph: "i_Druid Pouch",
  hslv: "i_Healing Salve",
  pclr: "i_Clarity Potion",
  plcl: "i_Lesser Clarity Potion",
  rej1: "i_Minor Replenishment Potion",
  rej2: "i_Lesser Replenishment Potion",
  rej3: "i_Replenishment Potion",
  rej4: "i_Greater Replenishment Potion",
  rej5: "i_Lesser Scroll of Replenishment ",
  rej6: "i_Greater Scroll of Replenishment ",
  sreg: "i_Scroll of Regeneration",
  gold: "i_Gold Coins",
  lmbr: "i_Bundle of Lumber",
  fgun: "i_Flare Gun",
  pomn: "i_Potion of Omniscience",
  gomn: "i_Glyph of Omniscience",
  wneu: "i_Wand of Neutralization",
  silk: "i_Spider Silk Broach",
  lure: "i_Monster Lure",
  skul: "i_Sacrificial Skull",
  moon: "i_Moonstone",
  brac: "i_Runed Bracers",
  vamp: "i_Vampiric Potion",
  woms: "i_Wand of Mana Stealing",
  tcas: "i_Tiny Castle",
  tgrh: "i_Tiny Great Hall",
  tsct: "i_Ivory Tower",
  wshs: "i_Wand of Shadowsight",
  tret: "i_Tome of Retraining",
  sneg: "i_Staff of Negation",
  stel: "i_Staff of Teleportation",
  spre: "i_Staff of Preservation",
  mcri: "i_Mechanical Critter",
  spsh: "i_Amulet of Spell Shield",
  sbok: "i_Spell Book",
  ssan: "i_Staff of Sanctuary",
  shas: "i_Scroll of Speed",
  dust: "i_Dust of Appearance",
  oslo: "i_Orb of Slow",
  dsum: "i_Diamond of Summoning",
  sor1: "i_Shadow Orb +1",
  sor2: "i_Shadow Orb +2",
  sor3: "i_Shadow Orb +3",
  sor4: "i_Shadow Orb +4",
  sor5: "i_Shadow Orb +5",
  sor6: "i_Shadow Orb +6",
  sor7: "i_Shadow Orb +7",
  sor8: "i_Shadow Orb +8",
  sor9: "i_Shadow Orb +9",
  sora: "i_Shadow Orb +10",
  sorf: "i_Shadow Orb Fragment",
  fwss: "i_Frost Wyrm Skull Shield",
  ram1: "i_Ring of the Archmagi",
  ram2: "i_Ring of the Archmagi",
  ram3: "i_Ring of the Archmagi",
  ram4: "i_Ring of the Archmagi",
  shtm: "i_Shamanic Totem",
  shwd: "i_Shimmerweed",
  btst: "i_Battle Standard",
  skrt: "i_Skeletal Artifact",
  thle: "i_Thunder Lizard Egg",
  sclp: "i_Secret Level Powerup",
  gldo: "i_Orb of Kil'jaeden",
  tbsm: "i_Tiny Blacksmith",
  tfar: "i_Tiny Farm",
  tlum: "i_Tiny Lumber Mill",
  tbar: "i_Tiny Barracks",
  tbak: "i_Tiny Altar of Kings",
  mgtk: "i_Magic Key Chain",
  stre: "i_Staff of Reanimation",
  horl: "i_Sacred Relic",
  hbth: "i_Helm of Battlethirst",
  blba: "i_Bladebane Armor",
  rugt: "i_Runed Gauntlets",
  frhg: "i_Firehand Gauntlets",
  gvsm: "i_Gloves of Spell Mastery",
  crdt: "i_Crown of the Deathlord",
  arsc: "i_Arcane Scroll",
  scul: "i_Scroll of the Unholy Legion",
  tmsc: "i_Tome of Sacrifices",
  dtsb: "i_Drek'thar's Spellbook",
  grsl: "i_Grimoire of Souls",
  arsh: "i_Arcanite Shield",
  shdt: "i_Shield of the Deathlord",
  shhn: "i_Shield of Honor",
  shen: "i_Enchanted Shield",
  thdm: "i_Thunderlizard Diamond",
  stpg: "i_Clockwork Penguin",
  shrs: "i_Shimmerglaze Roast",
  bfhr: "i_Bloodfeather's Heart",
  cosl: "i_Celestial Orb of Souls",
  shcw: "i_Shaman Claws",
  srbd: "i_Searing Blade",
  frgd: "i_Frostguard",
  envl: "i_Enchanted Vial",
  rump: "i_Rusty Mining Pick",
  mort: "i_Mogrin's Report",
  srtl: "i_Serathil",
  stwa: "i_Sturdy War Axe",
  klmm: "i_Killmaim",
  rots: "i_Scepter of the Sea",
  axas: "i_Ancestral Staff",
  mnsf: "i_Mindstaff",
  schl: "i_Scepter of Healing",
  asbl: "i_Assassin's Blade",
  kgal: "i_Keg of Ale",
  dphe: "i_Thunder Phoenix Egg",
  dkfw: "i_Keg of Thunderwater",
  dthb: "i_Thunderbloom Bulb",
};

const units: { [key: string]: string } = {
  hfoo: "u_Footman",
  hkni: "u_Knight",
  hmpr: "u_Priest",
  hmtm: "u_Mortar Team",
  hpea: "u_Peasant",
  hrif: "u_Rifleman",
  hsor: "u_Sorceress",
  hmtt: "u_Siege Engine",
  hrtt: "u_Siege Engine",
  hgry: "u_Gryphon Rider",
  hgyr: "u_Flying Machine",
  hspt: "u_Spell Breaker",
  hdhw: "u_Dragonhawk Rider",
  // building upgrades
  hkee: "b_Keep",
  hcas: "b_Castle",
  hctw: "b_Cannon Tower",
  hgtw: "b_Guard Tower",
  hatw: "b_Arcane Tower",
  ebal: "u_Glaive Thrower",
  echm: "u_Chimaera",
  edoc: "u_Druid of the Claw",
  edot: "u_Druid of the Talon",
  ewsp: "u_Wisp",
  esen: "u_Huntress",
  earc: "u_Archer",
  edry: "u_Dryad",
  ehip: "u_Hippogryph",
  emtg: "u_Mountain Giant",
  efdr: "u_Faerie Dragon",
  // building upgrades
  etoa: "b_Tree of Ages",
  etoe: "b_Tree of Eternity",
  ocat: "u_Demolisher",
  odoc: "u_Troll Witch Doctor",
  ogru: "u_Grunt",
  ohun: "u_Troll Headhunter/Berserker",
  otbk: "u_Troll Headhunter/Berserker",
  okod: "u_Kodo Beast",
  opeo: "u_Peon",
  orai: "u_Raider",
  oshm: "u_Shaman",
  otau: "u_Tauren",
  owyv: "u_Wind Rider",
  ospw: "u_Spirit Walker",
  ospm: "u_Spirit Walker",
  otbr: "u_Troll Batrider",
  // building upgrades
  ofrt: "b_Fortress",
  ostr: "b_Stronghold",
  uaco: "u_Acolyte",
  uabo: "u_Abomination",
  uban: "u_Banshee",
  ucry: "u_Crypt Fiend",
  ufro: "u_Frost Wyrm",
  ugar: "u_Gargoyle",
  ugho: "u_Ghoul",
  unec: "u_Necromancer",
  umtw: "u_Meatwagon",
  ushd: "u_Shade",
  uobs: "u_Obsidian Statue",
  ubsp: "u_Destroyer",
  // building upgrades
  unp1: "b_Halls of the Dead",
  unp2: "b_Black Citadel",
  uzg1: "b_Spirit Tower",
  uzg2: "b_Nerubian Tower",

  nskm: "u_Skeletal Marksman",
  nskf: "u_Burning Archer",
  nws1: "u_Dragon Hawk",
  nban: "u_Bandit",
  nrog: "u_Rogue",
  nenf: "u_Enforcer",
  nass: "u_Assassin",
  nbdk: "u_Black Drake",
  nrdk: "u_Red Dragon Whelp",
  nbdr: "u_Black Dragon Whelp",
  nrdr: "u_Red Drake",
  nbwm: "u_Black Dragon",
  nrwm: "u_Red Dragon",
  nadr: "u_Blue Dragon",
  nadw: "u_Blue Dragon Whelp",
  nadk: "u_Blue Drake",
  nbzd: "u_Bronze Dragon",
  nbzk: "u_Bronze Drake",
  nbzw: "u_Bronze Dragon Whelp",
  ngrd: "u_Green Dragon",
  ngdk: "u_Green Drake",
  ngrw: "u_Green Dragon Whelp",
  ncea: "u_Centaur Archer",
  ncen: "u_Centaur Outrunner",
  ncer: "u_Centaur Drudge",
  ndth: "u_Dark Troll High Priest",
  ndtp: "u_Dark Troll Shadow Priest",
  ndtb: "u_Dark Troll Berserker",
  ndtw: "u_Dark Troll Warlord",
  ndtr: "u_Dark Troll",
  ndtt: "u_Dark Troll Trapper",
  nfsh: "u_Forest Troll High Priest",
  nfsp: "u_Forest Troll Shadow Priest",
  nftr: "u_Forest Troll",
  nftb: "u_Forest Troll Berserker",
  nftt: "u_Forest Troll Trapper",
  nftk: "u_Forest Troll Warlord",
  ngrk: "u_Mud Golem",
  ngir: "u_Goblin Shredder",
  nfrs: "u_Furbolg Shaman",
  ngna: "u_Gnoll Poacher",
  ngns: "u_Gnoll Assassin",
  ngno: "u_Gnoll",
  ngnb: "u_Gnoll Brute",
  ngnw: "u_Gnoll Warden",
  ngnv: "u_Gnoll Overseer",
  ngsp: "u_Goblin Sapper",
  nhrr: "u_Harpy Rogue",
  nhrw: "u_Harpy Windwitch",
  nits: "u_Ice Troll Berserker",
  nitt: "u_Ice Troll Trapper",
  nkob: "u_Kobold",
  nkog: "u_Kobold Geomancer",
  nthl: "u_Thunder Lizard",
  nmfs: "u_Murloc Flesheater",
  nmrr: "u_Murloc Huntsman",
  nowb: "u_Wildkin",
  nrzm: "u_Razormane Medicine Man",
  nnwa: "u_Nerubian Warrior",
  nnwl: "u_Nerubian Webspinner",
  nogr: "u_Ogre Warrior",
  nogm: "u_Ogre Mauler",
  nogl: "u_Ogre Lord",
  nomg: "u_Ogre Magi",
  nrvs: "u_Frost Revenant",
  nslf: "u_Sludge Flinger",
  nsts: "u_Satyr Shadowdancer",
  nstl: "u_Satyr Soulstealer",
  nzep: "u_Goblin Zeppelin",
  ntrt: "u_Giant Sea Turtle",
  nlds: "u_Makrura Deepseer",
  nlsn: "u_Makrura Snapper",
  nmsn: "u_Mur'gul Snarecaster",
  nscb: "u_Spider Crab Shorecrawler",
  nbot: "u_Transport Ship",
  nsc2: "u_Spider Crab Limbripper",
  nsc3: "u_Spider Crab Behemoth",
  nbdm: "u_Blue Dragonspawn Meddler",
  nmgw: "u_Magnataur Warrior",
  nanb: "u_Barbed Arachnathid",
  nanm: "u_Barbed Arachnathid",
  nfps: "u_Polar Furbolg Shaman",
  nmgv: "u_Magic Vault",
  nitb: "u_Icy Treasure Box",
  npfl: "u_Fel Beast",
  ndrd: "u_Draenei Darkslayer",
  ndrm: "u_Draenei Disciple",
  nvdw: "u_Voidwalker",
  nvdg: "u_Greater Voidwalker",
  nnht: "u_Nether Dragon Hatchling",
  nndk: "u_Nether Drake",
  nndr: "u_Nether Dragon",
};

const buildings: { [key: string]: string } = {
  hhou: "Farm",
  halt: "Altar of Kings",
  harm: "Workshop",
  hars: "Arcane Sanctum",
  hbar: "Barracks",
  hbla: "Blacksmith",
  hgra: "Gryphon Aviary",
  hwtw: "Scout Tower",
  hvlt: "Arcane Vault",
  hlum: "Lumber Mill",
  htow: "Town Hall",
  etrp: "Ancient Protector",
  etol: "Tree of Life",
  edob: "Hunter's Hall",
  eate: "Altar of Elders",
  eden: "Ancient of Wonders",
  eaoe: "Ancient of Lore",
  eaom: "Ancient of War",
  eaow: "Ancient of Wind",
  edos: "Chimaera Roost",
  emow: "Moon Well",
  oalt: "Altar of Storms",
  obar: "Barracks",
  obea: "Beastiary",
  ofor: "War Mill",
  ogre: "Great Hall",
  osld: "Spirit Lodge",
  otrb: "Orc Burrow",
  orbr: "Reinforced Orc Burrow",
  otto: "Tauren Totem",
  ovln: "Voodoo Lounge",
  owtw: "Watch Tower",
  uaod: "Altar of Darkness",
  unpl: "Necropolis",
  usep: "Crypt",
  utod: "Temple of the Damned",
  utom: "Tomb of Relics",
  ugol: "Haunted Gold Mine",
  uzig: "Ziggurat",
  ubon: "Boneyard",
  usap: "Sacrificial Pit",
  uslh: "Slaughterhouse",
  ugrv: "Graveyard",
};

const upgrades: { [key: string]: string } = {
  Rhss: "p_Control Magic",
  Rhme: "p_Swords",
  Rhra: "p_Gunpowder",
  Rhar: "p_Plating",
  Rhla: "p_Armor",
  Rhac: "p_Masonry",
  Rhgb: "p_Flying Machine Bombs",
  Rhlh: "p_Lumber Harvesting",
  Rhde: "p_Defend",
  Rhan: "p_Animal War Training",
  Rhpt: "p_Priest Training",
  Rhst: "p_Sorceress Training",
  Rhri: "p_Long Rifles",
  Rhse: "p_Magic Sentry",
  Rhfl: "p_Flare",
  Rhhb: "p_Storm Hammers",
  Rhrt: "p_Barrage",
  Rhpm: "p_Backpack",
  Rhfc: "p_Flak Cannons",
  Rhfs: "p_Fragmentation Shards",
  Rhcd: "p_Cloud",
  Resm: "p_Strength of the Moon",
  Resw: "p_Strength of the Wild",
  Rema: "p_Moon Armor",
  Rerh: "p_Reinforced Hides",
  Reuv: "p_Ultravision",
  Renb: "p_Nature's Blessing",
  Reib: "p_Improved Bows",
  Remk: "p_Marksmanship",
  Resc: "p_Sentinel",
  Remg: "p_Upgrade Moon Glaive",
  Redt: "p_Druid of the Talon Training",
  Redc: "p_Druid of the Claw Training",
  Resi: "p_Abolish Magic",
  Reht: "p_Hippogryph Taming",
  Recb: "p_Corrosive Breath",
  Repb: "p_Vorpal Blades",
  Rers: "p_Resistant Skin",
  Rehs: "p_Hardened Skin",
  Reeb: "p_Mark of the Claw",
  Reec: "p_Mark of the Talon",
  Rews: "p_Well Spring",
  Repm: "p_Backpack",
  Roch: "p_Chaos",
  Rome: "p_Melee Weapons",
  Rora: "p_Ranged Weapons",
  Roar: "p_Armor",
  Rwdm: "p_War Drums Damage Increase",
  Ropg: "p_Pillage",
  Robs: "p_Berserker Strength",
  Rows: "p_Pulverize",
  Roen: "p_Ensnare",
  Rovs: "p_Envenomed Spears",
  Rowd: "p_Witch Doctor Training",
  Rost: "p_Shaman Training",
  Rosp: "p_Spiked Barricades",
  Rotr: "p_Troll Regeneration",
  Rolf: "p_Liquid Fire",
  Ropm: "p_Backpack",
  Rowt: "p_Spirit Walker Training",
  Robk: "p_Berserker Upgrade",
  Rorb: "p_Reinforced Defenses",
  Robf: "p_Burning Oil",
  Rusp: "p_Destroyer Form",
  Rume: "p_Unholy Strength",
  Rura: "p_Creature Attack",
  Ruar: "p_Unholy Armor",
  Rucr: "p_Creature Carapace",
  Ruac: "p_Cannibalize",
  Rugf: "p_Ghoul Frenzy",
  Ruwb: "p_Web",
  Rusf: "p_Stone Form",
  Rune: "p_Necromancer Training",
  Ruba: "p_Banshee Training",
  Rufb: "p_Freezing Breath",
  Rusl: "p_Skeletal Longevity",
  Rupc: "p_Disease Cloud",
  Rusm: "p_Skeletal Mastery",
  Rubu: "p_Burrow",
  Ruex: "p_Exhume Corpses",
  Rupm: "p_Backpack",
};

const heroAbilities: { [key: string]: string } = {
  AHbz: "a_Archmage:Blizzard",
  AHwe: "a_Archmage:Summon Water Elemental",
  AHab: "a_Archmage:Brilliance Aura",
  AHmt: "a_Archmage:Mass Teleport",
  AHtb: "a_Mountain King:Storm Bolt",
  AHtc: "a_Mountain King:Thunder Clap",
  AHbh: "a_Mountain King:Bash",
  AHav: "a_Mountain King:Avatar",
  AHhb: "a_Paladin:Holy Light",
  AHds: "a_Paladin:Divine Shield",
  AHad: "a_Paladin:Devotion Aura",
  AHre: "a_Paladin:Resurrection",
  AHdr: "a_Blood Mage:Siphon Mana",
  AHfs: "a_Blood Mage:Flame Strike",
  AHbn: "a_Blood Mage:Banish",
  AHpx: "a_Blood Mage:Summon Phoenix",
  AEmb: "a_Demon Hunter:Mana Burn",
  AEim: "a_Demon Hunter:Immolation",
  AEev: "a_Demon Hunter:Evasion",
  AEme: "a_Demon Hunter:Metamorphosis",
  AEer: "a_Keeper of the Grove:Entangling Roots",
  AEfn: "a_Keeper of the Grove:Force of Nature",
  AEah: "a_Keeper of the Grove:Thorns Aura",
  AEtq: "a_Keeper of the Grove:Tranquility",
  AEst: "a_Priestess of the Moon:Scout",
  AHfa: "a_Priestess of the Moon:Searing Arrows",
  AEar: "a_Priestess of the Moon:Trueshot Aura",
  AEsf: "a_Priestess of the Moon:Starfall",
  AEbl: "a_Warden:Blink",
  AEfk: "a_Warden:Fan of Knives",
  AEsh: "a_Warden:Shadow Strike",
  AEsv: "a_Warden:Spirit of Vengeance",
  AOwk: "a_Blademaster:Wind Walk",
  AOmi: "a_Blademaster:Mirror Image",
  AOcr: "a_Blademaster:Critical Strike",
  AOww: "a_Blademaster:Bladestorm",
  AOcl: "a_Far Seer:Chain Lighting",
  AOfs: "a_Far Seer:Far Sight",
  AOsf: "a_Far Seer:Feral Spirit",
  AOeq: "a_Far Seer:Earth Quake",
  AOsh: "a_Tauren Chieftain:Shockwave",
  AOae: "a_Tauren Chieftain:Endurance Aura",
  AOws: "a_Tauren Chieftain:War Stomp",
  AOre: "a_Tauren Chieftain:Reincarnation",
  AOhw: "a_Shadow Hunter:Healing Wave",
  AOhx: "a_Shadow Hunter:Hex",
  AOsw: "a_Shadow Hunter:Serpent Ward",
  AOvd: "a_Shadow Hunter:Big Bad Voodoo",
  AUdc: "a_Death Knight:Death Coil",
  AUdp: "a_Death Knight:Death Pact",
  AUau: "a_Death Knight:Unholy Aura",
  AUan: "a_Death Knight:Animate Dead",
  AUcs: "a_Dreadlord:Carrion Swarm",
  AUsl: "a_Dreadlord:Sleep",
  AUav: "a_Dreadlord:Vampiric Aura",
  AUin: "a_Dreadlord:Inferno",
  AUfn: "a_Lich:Frost Nova",
  AUfa: "a_Lich:Frost Armor",
  AUfu: "a_Lich:Frost Armor",
  AUdr: "a_Lich:Dark Ritual",
  AUdd: "a_Lich:Death and Decay",
  AUim: "a_Crypt Lord:Impale",
  AUts: "a_Crypt Lord:Spiked Carapace",
  AUcb: "a_Crypt Lord:Carrion Beetles",
  AUls: "a_Crypt Lord:Locust Swarm",
  ANbf: "a_Pandaren Brewmaster:Breath of Fire",
  ANdb: "a_Pandaren Brewmaster:Drunken Brawler",
  ANdh: "a_Pandaren Brewmaster:Drunken Haze",
  ANef: "a_Pandaren Brewmaster:Storm Earth and Fire",
  ANdr: "a_Dark Ranger:Life Drain",
  ANsi: "a_Dark Ranger:Silence",
  ANba: "a_Dark Ranger:Black Arrow",
  ANch: "a_Dark Ranger:Charm",
  ANms: "a_Naga Sea Witch:Mana Shield",
  ANfa: "a_Naga Sea Witch:Frost Arrows",
  ANfl: "a_Naga Sea Witch:Forked Lightning",
  ANto: "a_Naga Sea Witch:Tornado",
  ANrf: "a_Pit Lord:Rain of Fire",
  ANca: "a_Pit Lord:Cleaving Attack",
  ANht: "a_Pit Lord:Howl of Terror",
  ANdo: "a_Pit Lord:Doom",
  ANsg: "a_Beastmaster:Summon Bear",
  ANsq: "a_Beastmaster:Summon Quilbeast",
  ANsw: "a_Beastmaster:Summon Hawk",
  ANst: "a_Beastmaster:Stampede",
  ANeg: "a_Goblin Tinker:Engineering Upgrade",
  ANcs: "a_Goblin Tinker:Cluster Rockets",
  ANc1: "a_Goblin Tinker:Cluster Rockets",
  ANc2: "a_Goblin Tinker:Cluster Rockets",
  ANc3: "a_Goblin Tinker:Cluster Rockets",
  ANsy: "a_Goblin Tinker:Pocket Factory",
  ANs1: "a_Goblin Tinker:Pocket Factory",
  ANs2: "a_Goblin Tinker:Pocket Factory",
  ANs3: "a_Goblin Tinker:Pocket Factory",
  ANrg: "a_Goblin Tinker:Robo-Goblin",
  ANg1: "a_Goblin Tinker:Robo-Goblin",
  ANg2: "a_Goblin Tinker:Robo-Goblin",
  ANg3: "a_Goblin Tinker:Robo-Goblin",
  ANic: "a_Firelord:Incinerate",
  ANia: "a_Firelord:Incinerate",
  ANso: "a_Firelord:Soul Burn",
  ANlm: "a_Firelord:Summon Lava Spawn",
  ANvc: "a_Firelord:Volcano",
  ANhs: "a_Goblin Alchemist:Healing Spray",
  ANab: "a_Goblin Alchemist:Acid Bomb",
  ANcr: "a_Goblin Alchemist:Chemical Rage",
  ANtm: "a_Goblin Alchemist:Transmute",
};

const heroes: { [key: string]: string } = {
  Hamg: "Archmage",
  Hmkg: "Mountain King",
  Hpal: "Paladin",
  Hblm: "Blood Mage",
  Edem: "Demon Hunter",
  Ekee: "Keeper",
  Emoo: "Pristress",
  Ewar: "Warden",
  Obla: "Blademaster",
  Ofar: "FarSeer",
  Otch: "Tauren Chefitan",
  Oshd: "Shadow Hunter",
  Udea: "Death Knight",
  Udre: "Dreadlord",
  Ulic: "Lich",
  Ucrl: "Crypt Lord",
  Npbm: "Panda",
  Nbrn: "Dark Ranger",
  Nngs: "Naga",
  Nplh: "Pit Lord",
  Nbst: "Beast Master",
  Ntin: "Tinker",
  Nfir: "Firelord",
  Nalc: "Alchemist"
}

const abilityToHero: { [key: string]: string } = {
  AHbz: "Hamg",
  AHwe: "Hamg",
  AHab: "Hamg",
  AHmt: "Hamg",
  AHtb: "Hmkg",
  AHtc: "Hmkg",
  AHbh: "Hmkg",
  AHav: "Hmkg",
  AHhb: "Hpal",
  AHds: "Hpal",
  AHad: "Hpal",
  AHre: "Hpal",
  AHdr: "Hblm",
  AHfs: "Hblm",
  AHbn: "Hblm",
  AHpx: "Hblm",
  AEmb: "Edem",
  AEim: "Edem",
  AEev: "Edem",
  AEme: "Edem",
  AEer: "Ekee",
  AEfn: "Ekee",
  AEah: "Ekee",
  AEtq: "Ekee",
  AEst: "Emoo",
  AHfa: "Emoo",
  AEar: "Emoo",
  AEsf: "Emoo",
  AEbl: "Ewar",
  AEfk: "Ewar",
  AEsh: "Ewar",
  AEsv: "Ewar",
  AOwk: "Obla",
  AOmi: "Obla",
  AOcr: "Obla",
  AOww: "Obla",
  AOcl: "Ofar",
  AOfs: "Ofar",
  AOsf: "Ofar",
  AOeq: "Ofar",
  AOsh: "Otch",
  AOae: "Otch",
  AOws: "Otch",
  AOre: "Otch",
  AOhw: "Oshd",
  AOhx: "Oshd",
  AOsw: "Oshd",
  AOvd: "Oshd",
  AUdc: "Udea",
  AUdp: "Udea",
  AUau: "Udea",
  AUan: "Udea",
  AUcs: "Udre",
  AUsl: "Udre",
  AUav: "Udre",
  AUin: "Udre",
  AUfn: "Ulic",
  AUfa: "Ulic",
  AUfu: "Ulic",
  AUdr: "Ulic",
  AUdd: "Ulic",
  AUim: "Ucrl",
  AUts: "Ucrl",
  AUcb: "Ucrl",
  AUls: "Ucrl",
  ANbf: "Npbm",
  ANdb: "Npbm",
  ANdh: "Npbm",
  ANef: "Npbm",
  ANdr: "Nbrn",
  ANsi: "Nbrn",
  ANba: "Nbrn",
  ANch: "Nbrn",
  ANms: "Nngs",
  ANfa: "Nngs",
  ANfl: "Nngs",
  ANto: "Nngs",
  ANrf: "Nplh",
  ANca: "Nplh",
  ANht: "Nplh",
  ANdo: "Nplh",
  ANsg: "Nbst",
  ANsq: "Nbst",
  ANsw: "Nbst",
  ANst: "Nbst",
  ANeg: "Ntin",
  ANcs: "Ntin",
  ANc1: "Ntin",
  ANc2: "Ntin",
  ANc3: "Ntin",
  ANsy: "Ntin",
  ANs1: "Ntin",
  ANs2: "Ntin",
  ANs3: "Ntin",
  ANrg: "Ntin",
  ANg1: "Ntin",
  ANg2: "Ntin",
  ANg3: "Ntin",
  ANic: "Nfir",
  ANia: "Nfir",
  ANso: "Nfir",
  ANlm: "Nfir",
  ANvc: "Nfir",
  ANhs: "Nalc",
  ANab: "Nalc",
  ANcr: "Nalc",
  ANtm: "Nalc",
};

const itemIds = Object.keys(items);

export {
  items,
  units,
  heroes,
  buildings,
  upgrades,
  heroAbilities,
  abilityToHero,
  itemIds,
};
