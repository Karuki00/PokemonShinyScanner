// AUTO-GENERATED FILE. DO NOT EDIT MANUALLY.
// Source: core/pokemon_data.py

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SpeciesInfo {
    pub name: &'static str,
    pub national_dex: u16,
    pub abilities: [Option<&'static str>; 2],
}

pub fn species_info(species: u16) -> Option<SpeciesInfo> {
    match species {
        1 => Some(SpeciesInfo {
            name: "Bulbasaur",
            national_dex: 1,
            abilities: [None, None],
        }),
        2 => Some(SpeciesInfo {
            name: "Ivysaur",
            national_dex: 2,
            abilities: [Some("Overgrow"), Some("Overgrow")],
        }),
        3 => Some(SpeciesInfo {
            name: "Venusaur",
            national_dex: 3,
            abilities: [Some("Overgrow"), Some("Overgrow")],
        }),
        4 => Some(SpeciesInfo {
            name: "Charmander",
            national_dex: 4,
            abilities: [Some("Blaze"), Some("Blaze")],
        }),
        5 => Some(SpeciesInfo {
            name: "Charmeleon",
            national_dex: 5,
            abilities: [Some("Blaze"), Some("Blaze")],
        }),
        6 => Some(SpeciesInfo {
            name: "Charizard",
            national_dex: 6,
            abilities: [Some("Blaze"), Some("Blaze")],
        }),
        7 => Some(SpeciesInfo {
            name: "Squirtle",
            national_dex: 7,
            abilities: [Some("Torrent"), Some("Torrent")],
        }),
        8 => Some(SpeciesInfo {
            name: "Wartortle",
            national_dex: 8,
            abilities: [Some("Torrent"), Some("Torrent")],
        }),
        9 => Some(SpeciesInfo {
            name: "Blastoise",
            national_dex: 9,
            abilities: [Some("Torrent"), Some("Torrent")],
        }),
        10 => Some(SpeciesInfo {
            name: "Caterpie",
            national_dex: 10,
            abilities: [Some("Shield Dust"), Some("Shield Dust")],
        }),
        11 => Some(SpeciesInfo {
            name: "Metapod",
            national_dex: 11,
            abilities: [Some("Shed Skin"), Some("Shed Skin")],
        }),
        12 => Some(SpeciesInfo {
            name: "Butterfree",
            national_dex: 12,
            abilities: [Some("Compound Eyes"), Some("Compound Eyes")],
        }),
        13 => Some(SpeciesInfo {
            name: "Weedle",
            national_dex: 13,
            abilities: [Some("Shield Dust"), Some("Shield Dust")],
        }),
        14 => Some(SpeciesInfo {
            name: "Kakuna",
            national_dex: 14,
            abilities: [Some("Shed Skin"), Some("Shed Skin")],
        }),
        15 => Some(SpeciesInfo {
            name: "Beedrill",
            national_dex: 15,
            abilities: [Some("Swarm"), Some("Swarm")],
        }),
        16 => Some(SpeciesInfo {
            name: "Pidgey",
            national_dex: 16,
            abilities: [Some("Keen Eye"), Some("Keen Eye")],
        }),
        17 => Some(SpeciesInfo {
            name: "Pidgeotto",
            national_dex: 17,
            abilities: [Some("Keen Eye"), Some("Keen Eye")],
        }),
        18 => Some(SpeciesInfo {
            name: "Pidgeot",
            national_dex: 18,
            abilities: [Some("Keen Eye"), Some("Keen Eye")],
        }),
        19 => Some(SpeciesInfo {
            name: "Rattata",
            national_dex: 19,
            abilities: [Some("Run Away"), Some("Guts")],
        }),
        20 => Some(SpeciesInfo {
            name: "Raticate",
            national_dex: 20,
            abilities: [Some("Run Away"), Some("Guts")],
        }),
        21 => Some(SpeciesInfo {
            name: "Spearow",
            national_dex: 21,
            abilities: [Some("Keen Eye"), Some("Keen Eye")],
        }),
        22 => Some(SpeciesInfo {
            name: "Fearow",
            national_dex: 22,
            abilities: [Some("Keen Eye"), Some("Keen Eye")],
        }),
        23 => Some(SpeciesInfo {
            name: "Ekans",
            national_dex: 23,
            abilities: [Some("Intimidate"), Some("Shed Skin")],
        }),
        24 => Some(SpeciesInfo {
            name: "Arbok",
            national_dex: 24,
            abilities: [Some("Intimidate"), Some("Shed Skin")],
        }),
        25 => Some(SpeciesInfo {
            name: "Pikachu",
            national_dex: 25,
            abilities: [Some("Static"), Some("Static")],
        }),
        26 => Some(SpeciesInfo {
            name: "Raichu",
            national_dex: 26,
            abilities: [Some("Static"), Some("Static")],
        }),
        27 => Some(SpeciesInfo {
            name: "Sandshrew",
            national_dex: 27,
            abilities: [Some("Sand Veil"), Some("Sand Veil")],
        }),
        28 => Some(SpeciesInfo {
            name: "Sandslash",
            national_dex: 28,
            abilities: [Some("Sand Veil"), Some("Sand Veil")],
        }),
        29 => Some(SpeciesInfo {
            name: "Nidoran♀",
            national_dex: 29,
            abilities: [Some("Poison Point"), Some("Poison Point")],
        }),
        30 => Some(SpeciesInfo {
            name: "Nidorina",
            national_dex: 30,
            abilities: [Some("Poison Point"), Some("Poison Point")],
        }),
        31 => Some(SpeciesInfo {
            name: "Nidoqueen",
            national_dex: 31,
            abilities: [Some("Poison Point"), Some("Poison Point")],
        }),
        32 => Some(SpeciesInfo {
            name: "Nidoran♂",
            national_dex: 32,
            abilities: [Some("Poison Point"), Some("Poison Point")],
        }),
        33 => Some(SpeciesInfo {
            name: "Nidorino",
            national_dex: 33,
            abilities: [Some("Poison Point"), Some("Poison Point")],
        }),
        34 => Some(SpeciesInfo {
            name: "Nidoking",
            national_dex: 34,
            abilities: [Some("Poison Point"), Some("Poison Point")],
        }),
        35 => Some(SpeciesInfo {
            name: "Clefairy",
            national_dex: 35,
            abilities: [Some("Cute Charm"), Some("Cute Charm")],
        }),
        36 => Some(SpeciesInfo {
            name: "Clefable",
            national_dex: 36,
            abilities: [Some("Cute Charm"), Some("Cute Charm")],
        }),
        37 => Some(SpeciesInfo {
            name: "Vulpix",
            national_dex: 37,
            abilities: [Some("Flash Fire"), Some("Flash Fire")],
        }),
        38 => Some(SpeciesInfo {
            name: "Ninetales",
            national_dex: 38,
            abilities: [Some("Flash Fire"), Some("Flash Fire")],
        }),
        39 => Some(SpeciesInfo {
            name: "Jigglypuff",
            national_dex: 39,
            abilities: [Some("Cute Charm"), Some("Cute Charm")],
        }),
        40 => Some(SpeciesInfo {
            name: "Wigglytuff",
            national_dex: 40,
            abilities: [Some("Cute Charm"), Some("Cute Charm")],
        }),
        41 => Some(SpeciesInfo {
            name: "Zubat",
            national_dex: 41,
            abilities: [Some("Inner Focus"), Some("Inner Focus")],
        }),
        42 => Some(SpeciesInfo {
            name: "Golbat",
            national_dex: 42,
            abilities: [Some("Inner Focus"), Some("Inner Focus")],
        }),
        43 => Some(SpeciesInfo {
            name: "Oddish",
            national_dex: 43,
            abilities: [Some("Chlorophyll"), Some("Chlorophyll")],
        }),
        44 => Some(SpeciesInfo {
            name: "Gloom",
            national_dex: 44,
            abilities: [Some("Chlorophyll"), Some("Chlorophyll")],
        }),
        45 => Some(SpeciesInfo {
            name: "Vileplume",
            national_dex: 45,
            abilities: [Some("Chlorophyll"), Some("Chlorophyll")],
        }),
        46 => Some(SpeciesInfo {
            name: "Paras",
            national_dex: 46,
            abilities: [Some("Effect Spore"), Some("Effect Spore")],
        }),
        47 => Some(SpeciesInfo {
            name: "Parasect",
            national_dex: 47,
            abilities: [Some("Effect Spore"), Some("Effect Spore")],
        }),
        48 => Some(SpeciesInfo {
            name: "Venonat",
            national_dex: 48,
            abilities: [Some("Compound Eyes"), Some("Compound Eyes")],
        }),
        49 => Some(SpeciesInfo {
            name: "Venomoth",
            national_dex: 49,
            abilities: [Some("Shield Dust"), Some("Shield Dust")],
        }),
        50 => Some(SpeciesInfo {
            name: "Diglett",
            national_dex: 50,
            abilities: [Some("Sand Veil"), Some("Arena Trap")],
        }),
        51 => Some(SpeciesInfo {
            name: "Dugtrio",
            national_dex: 51,
            abilities: [Some("Sand Veil"), Some("Arena Trap")],
        }),
        52 => Some(SpeciesInfo {
            name: "Meowth",
            national_dex: 52,
            abilities: [Some("Pickup"), Some("Pickup")],
        }),
        53 => Some(SpeciesInfo {
            name: "Persian",
            national_dex: 53,
            abilities: [Some("Limber"), Some("Limber")],
        }),
        54 => Some(SpeciesInfo {
            name: "Psyduck",
            national_dex: 54,
            abilities: [Some("Damp"), Some("Cloud Nine")],
        }),
        55 => Some(SpeciesInfo {
            name: "Golduck",
            national_dex: 55,
            abilities: [Some("Damp"), Some("Cloud Nine")],
        }),
        56 => Some(SpeciesInfo {
            name: "Mankey",
            national_dex: 56,
            abilities: [Some("Vital Spirit"), Some("Vital Spirit")],
        }),
        57 => Some(SpeciesInfo {
            name: "Primeape",
            national_dex: 57,
            abilities: [Some("Vital Spirit"), Some("Vital Spirit")],
        }),
        58 => Some(SpeciesInfo {
            name: "Growlithe",
            national_dex: 58,
            abilities: [Some("Intimidate"), Some("Flash Fire")],
        }),
        59 => Some(SpeciesInfo {
            name: "Arcanine",
            national_dex: 59,
            abilities: [Some("Intimidate"), Some("Flash Fire")],
        }),
        60 => Some(SpeciesInfo {
            name: "Poliwag",
            national_dex: 60,
            abilities: [Some("Water Absorb"), Some("Damp")],
        }),
        61 => Some(SpeciesInfo {
            name: "Poliwhirl",
            national_dex: 61,
            abilities: [Some("Water Absorb"), Some("Damp")],
        }),
        62 => Some(SpeciesInfo {
            name: "Poliwrath",
            national_dex: 62,
            abilities: [Some("Water Absorb"), Some("Damp")],
        }),
        63 => Some(SpeciesInfo {
            name: "Abra",
            national_dex: 63,
            abilities: [Some("Synchronize"), Some("Inner Focus")],
        }),
        64 => Some(SpeciesInfo {
            name: "Kadabra",
            national_dex: 64,
            abilities: [Some("Synchronize"), Some("Inner Focus")],
        }),
        65 => Some(SpeciesInfo {
            name: "Alakazam",
            national_dex: 65,
            abilities: [Some("Synchronize"), Some("Inner Focus")],
        }),
        66 => Some(SpeciesInfo {
            name: "Machop",
            national_dex: 66,
            abilities: [Some("Guts"), Some("Guts")],
        }),
        67 => Some(SpeciesInfo {
            name: "Machoke",
            national_dex: 67,
            abilities: [Some("Guts"), Some("Guts")],
        }),
        68 => Some(SpeciesInfo {
            name: "Machamp",
            national_dex: 68,
            abilities: [Some("Guts"), Some("Guts")],
        }),
        69 => Some(SpeciesInfo {
            name: "Bellsprout",
            national_dex: 69,
            abilities: [Some("Chlorophyll"), Some("Chlorophyll")],
        }),
        70 => Some(SpeciesInfo {
            name: "Weepinbell",
            national_dex: 70,
            abilities: [Some("Chlorophyll"), Some("Chlorophyll")],
        }),
        71 => Some(SpeciesInfo {
            name: "Victreebel",
            national_dex: 71,
            abilities: [Some("Chlorophyll"), Some("Chlorophyll")],
        }),
        72 => Some(SpeciesInfo {
            name: "Tentacool",
            national_dex: 72,
            abilities: [Some("Clear Body"), Some("Liquid Ooze")],
        }),
        73 => Some(SpeciesInfo {
            name: "Tentacruel",
            national_dex: 73,
            abilities: [Some("Clear Body"), Some("Liquid Ooze")],
        }),
        74 => Some(SpeciesInfo {
            name: "Geodude",
            national_dex: 74,
            abilities: [Some("Rock Head"), Some("Sturdy")],
        }),
        75 => Some(SpeciesInfo {
            name: "Graveler",
            national_dex: 75,
            abilities: [Some("Rock Head"), Some("Sturdy")],
        }),
        76 => Some(SpeciesInfo {
            name: "Golem",
            national_dex: 76,
            abilities: [Some("Rock Head"), Some("Sturdy")],
        }),
        77 => Some(SpeciesInfo {
            name: "Ponyta",
            national_dex: 77,
            abilities: [Some("Run Away"), Some("Flash Fire")],
        }),
        78 => Some(SpeciesInfo {
            name: "Rapidash",
            national_dex: 78,
            abilities: [Some("Run Away"), Some("Flash Fire")],
        }),
        79 => Some(SpeciesInfo {
            name: "Slowpoke",
            national_dex: 79,
            abilities: [Some("Oblivious"), Some("Own Tempo")],
        }),
        80 => Some(SpeciesInfo {
            name: "Slowbro",
            national_dex: 80,
            abilities: [Some("Oblivious"), Some("Own Tempo")],
        }),
        81 => Some(SpeciesInfo {
            name: "Magnemite",
            national_dex: 81,
            abilities: [Some("Magnet Pull"), Some("Sturdy")],
        }),
        82 => Some(SpeciesInfo {
            name: "Magneton",
            national_dex: 82,
            abilities: [Some("Magnet Pull"), Some("Sturdy")],
        }),
        83 => Some(SpeciesInfo {
            name: "Farfetch'd",
            national_dex: 83,
            abilities: [Some("Keen Eye"), Some("Inner Focus")],
        }),
        84 => Some(SpeciesInfo {
            name: "Doduo",
            national_dex: 84,
            abilities: [Some("Run Away"), Some("Early Bird")],
        }),
        85 => Some(SpeciesInfo {
            name: "Dodrio",
            national_dex: 85,
            abilities: [Some("Run Away"), Some("Early Bird")],
        }),
        86 => Some(SpeciesInfo {
            name: "Seel",
            national_dex: 86,
            abilities: [Some("Thick Fat"), Some("Thick Fat")],
        }),
        87 => Some(SpeciesInfo {
            name: "Dewgong",
            national_dex: 87,
            abilities: [Some("Thick Fat"), Some("Thick Fat")],
        }),
        88 => Some(SpeciesInfo {
            name: "Grimer",
            national_dex: 88,
            abilities: [Some("Stench"), Some("Sticky Hold")],
        }),
        89 => Some(SpeciesInfo {
            name: "Muk",
            national_dex: 89,
            abilities: [Some("Stench"), Some("Sticky Hold")],
        }),
        90 => Some(SpeciesInfo {
            name: "Shellder",
            national_dex: 90,
            abilities: [Some("Shell Armor"), Some("Shell Armor")],
        }),
        91 => Some(SpeciesInfo {
            name: "Cloyster",
            national_dex: 91,
            abilities: [Some("Shell Armor"), Some("Shell Armor")],
        }),
        92 => Some(SpeciesInfo {
            name: "Gastly",
            national_dex: 92,
            abilities: [Some("Levitate"), Some("Levitate")],
        }),
        93 => Some(SpeciesInfo {
            name: "Haunter",
            national_dex: 93,
            abilities: [Some("Levitate"), Some("Levitate")],
        }),
        94 => Some(SpeciesInfo {
            name: "Gengar",
            national_dex: 94,
            abilities: [Some("Levitate"), Some("Levitate")],
        }),
        95 => Some(SpeciesInfo {
            name: "Onix",
            national_dex: 95,
            abilities: [Some("Rock Head"), Some("Sturdy")],
        }),
        96 => Some(SpeciesInfo {
            name: "Drowzee",
            national_dex: 96,
            abilities: [Some("Insomnia"), Some("Insomnia")],
        }),
        97 => Some(SpeciesInfo {
            name: "Hypno",
            national_dex: 97,
            abilities: [Some("Insomnia"), Some("Insomnia")],
        }),
        98 => Some(SpeciesInfo {
            name: "Krabby",
            national_dex: 98,
            abilities: [Some("Hyper Cutter"), Some("Shell Armor")],
        }),
        99 => Some(SpeciesInfo {
            name: "Kingler",
            national_dex: 99,
            abilities: [Some("Hyper Cutter"), Some("Shell Armor")],
        }),
        100 => Some(SpeciesInfo {
            name: "Voltorb",
            national_dex: 100,
            abilities: [Some("Soundproof"), Some("Static")],
        }),
        101 => Some(SpeciesInfo {
            name: "Electrode",
            national_dex: 101,
            abilities: [Some("Soundproof"), Some("Static")],
        }),
        102 => Some(SpeciesInfo {
            name: "Exeggcute",
            national_dex: 102,
            abilities: [Some("Chlorophyll"), Some("Chlorophyll")],
        }),
        103 => Some(SpeciesInfo {
            name: "Exeggutor",
            national_dex: 103,
            abilities: [Some("Chlorophyll"), Some("Chlorophyll")],
        }),
        104 => Some(SpeciesInfo {
            name: "Cubone",
            national_dex: 104,
            abilities: [Some("Rock Head"), Some("Lightning Rod")],
        }),
        105 => Some(SpeciesInfo {
            name: "Marowak",
            national_dex: 105,
            abilities: [Some("Rock Head"), Some("Lightning Rod")],
        }),
        106 => Some(SpeciesInfo {
            name: "Hitmonlee",
            national_dex: 106,
            abilities: [Some("Limber"), Some("Limber")],
        }),
        107 => Some(SpeciesInfo {
            name: "Hitmonchan",
            national_dex: 107,
            abilities: [Some("Keen Eye"), Some("Keen Eye")],
        }),
        108 => Some(SpeciesInfo {
            name: "Lickitung",
            national_dex: 108,
            abilities: [Some("Own Tempo"), Some("Oblivious")],
        }),
        109 => Some(SpeciesInfo {
            name: "Koffing",
            national_dex: 109,
            abilities: [Some("Levitate"), Some("Levitate")],
        }),
        110 => Some(SpeciesInfo {
            name: "Weezing",
            national_dex: 110,
            abilities: [Some("Levitate"), Some("Levitate")],
        }),
        111 => Some(SpeciesInfo {
            name: "Rhyhorn",
            national_dex: 111,
            abilities: [Some("Lightning Rod"), Some("Rock Head")],
        }),
        112 => Some(SpeciesInfo {
            name: "Rhydon",
            national_dex: 112,
            abilities: [Some("Lightning Rod"), Some("Rock Head")],
        }),
        113 => Some(SpeciesInfo {
            name: "Chansey",
            national_dex: 113,
            abilities: [Some("Natural Cure"), Some("Serene Grace")],
        }),
        114 => Some(SpeciesInfo {
            name: "Tangela",
            national_dex: 114,
            abilities: [Some("Chlorophyll"), Some("Chlorophyll")],
        }),
        115 => Some(SpeciesInfo {
            name: "Kangaskhan",
            national_dex: 115,
            abilities: [Some("Early Bird"), Some("Early Bird")],
        }),
        116 => Some(SpeciesInfo {
            name: "Horsea",
            national_dex: 116,
            abilities: [Some("Swift Swim"), Some("Swift Swim")],
        }),
        117 => Some(SpeciesInfo {
            name: "Seadra",
            national_dex: 117,
            abilities: [Some("Poison Point"), Some("Poison Point")],
        }),
        118 => Some(SpeciesInfo {
            name: "Goldeen",
            national_dex: 118,
            abilities: [Some("Swift Swim"), Some("Water Veil")],
        }),
        119 => Some(SpeciesInfo {
            name: "Seaking",
            national_dex: 119,
            abilities: [Some("Swift Swim"), Some("Water Veil")],
        }),
        120 => Some(SpeciesInfo {
            name: "Staryu",
            national_dex: 120,
            abilities: [Some("Illuminate"), Some("Natural Cure")],
        }),
        121 => Some(SpeciesInfo {
            name: "Starmie",
            national_dex: 121,
            abilities: [Some("Illuminate"), Some("Natural Cure")],
        }),
        122 => Some(SpeciesInfo {
            name: "Mr. Mime",
            national_dex: 122,
            abilities: [Some("Soundproof"), Some("Soundproof")],
        }),
        123 => Some(SpeciesInfo {
            name: "Scyther",
            national_dex: 123,
            abilities: [Some("Swarm"), Some("Swarm")],
        }),
        124 => Some(SpeciesInfo {
            name: "Jynx",
            national_dex: 124,
            abilities: [Some("Oblivious"), Some("Oblivious")],
        }),
        125 => Some(SpeciesInfo {
            name: "Electabuzz",
            national_dex: 125,
            abilities: [Some("Static"), Some("Static")],
        }),
        126 => Some(SpeciesInfo {
            name: "Magmar",
            national_dex: 126,
            abilities: [Some("Flame Body"), Some("Flame Body")],
        }),
        127 => Some(SpeciesInfo {
            name: "Pinsir",
            national_dex: 127,
            abilities: [Some("Hyper Cutter"), Some("Hyper Cutter")],
        }),
        128 => Some(SpeciesInfo {
            name: "Tauros",
            national_dex: 128,
            abilities: [Some("Intimidate"), Some("Intimidate")],
        }),
        129 => Some(SpeciesInfo {
            name: "Magikarp",
            national_dex: 129,
            abilities: [Some("Swift Swim"), Some("Swift Swim")],
        }),
        130 => Some(SpeciesInfo {
            name: "Gyarados",
            national_dex: 130,
            abilities: [Some("Intimidate"), Some("Intimidate")],
        }),
        131 => Some(SpeciesInfo {
            name: "Lapras",
            national_dex: 131,
            abilities: [Some("Water Absorb"), Some("Shell Armor")],
        }),
        132 => Some(SpeciesInfo {
            name: "Ditto",
            national_dex: 132,
            abilities: [Some("Limber"), Some("Limber")],
        }),
        133 => Some(SpeciesInfo {
            name: "Eevee",
            national_dex: 133,
            abilities: [Some("Run Away"), Some("Run Away")],
        }),
        134 => Some(SpeciesInfo {
            name: "Vaporeon",
            national_dex: 134,
            abilities: [Some("Water Absorb"), Some("Water Absorb")],
        }),
        135 => Some(SpeciesInfo {
            name: "Jolteon",
            national_dex: 135,
            abilities: [Some("Volt Absorb"), Some("Volt Absorb")],
        }),
        136 => Some(SpeciesInfo {
            name: "Flareon",
            national_dex: 136,
            abilities: [Some("Flash Fire"), Some("Flash Fire")],
        }),
        137 => Some(SpeciesInfo {
            name: "Porygon",
            national_dex: 137,
            abilities: [Some("Trace"), Some("Trace")],
        }),
        138 => Some(SpeciesInfo {
            name: "Omanyte",
            national_dex: 138,
            abilities: [Some("Swift Swim"), Some("Shell Armor")],
        }),
        139 => Some(SpeciesInfo {
            name: "Omastar",
            national_dex: 139,
            abilities: [Some("Swift Swim"), Some("Shell Armor")],
        }),
        140 => Some(SpeciesInfo {
            name: "Kabuto",
            national_dex: 140,
            abilities: [Some("Swift Swim"), Some("Battle Armor")],
        }),
        141 => Some(SpeciesInfo {
            name: "Kabutops",
            national_dex: 141,
            abilities: [Some("Swift Swim"), Some("Battle Armor")],
        }),
        142 => Some(SpeciesInfo {
            name: "Aerodactyl",
            national_dex: 142,
            abilities: [Some("Rock Head"), Some("Pressure")],
        }),
        143 => Some(SpeciesInfo {
            name: "Snorlax",
            national_dex: 143,
            abilities: [Some("Immunity"), Some("Thick Fat")],
        }),
        144 => Some(SpeciesInfo {
            name: "Articuno",
            national_dex: 144,
            abilities: [Some("Pressure"), Some("Pressure")],
        }),
        145 => Some(SpeciesInfo {
            name: "Zapdos",
            national_dex: 145,
            abilities: [Some("Pressure"), Some("Pressure")],
        }),
        146 => Some(SpeciesInfo {
            name: "Moltres",
            national_dex: 146,
            abilities: [Some("Pressure"), Some("Pressure")],
        }),
        147 => Some(SpeciesInfo {
            name: "Dratini",
            national_dex: 147,
            abilities: [Some("Shed Skin"), Some("Shed Skin")],
        }),
        148 => Some(SpeciesInfo {
            name: "Dragonair",
            national_dex: 148,
            abilities: [Some("Shed Skin"), Some("Shed Skin")],
        }),
        149 => Some(SpeciesInfo {
            name: "Dragonite",
            national_dex: 149,
            abilities: [Some("Inner Focus"), Some("Inner Focus")],
        }),
        150 => Some(SpeciesInfo {
            name: "Mewtwo",
            national_dex: 150,
            abilities: [Some("Pressure"), Some("Pressure")],
        }),
        151 => Some(SpeciesInfo {
            name: "Mew",
            national_dex: 151,
            abilities: [Some("Synchronize"), Some("Synchronize")],
        }),
        152 => Some(SpeciesInfo {
            name: "Chikorita",
            national_dex: 152,
            abilities: [Some("Overgrow"), Some("Overgrow")],
        }),
        153 => Some(SpeciesInfo {
            name: "Bayleef",
            national_dex: 153,
            abilities: [Some("Overgrow"), Some("Overgrow")],
        }),
        154 => Some(SpeciesInfo {
            name: "Meganium",
            national_dex: 154,
            abilities: [Some("Overgrow"), Some("Overgrow")],
        }),
        155 => Some(SpeciesInfo {
            name: "Cyndaquil",
            national_dex: 155,
            abilities: [Some("Blaze"), Some("Blaze")],
        }),
        156 => Some(SpeciesInfo {
            name: "Quilava",
            national_dex: 156,
            abilities: [Some("Blaze"), Some("Blaze")],
        }),
        157 => Some(SpeciesInfo {
            name: "Typhlosion",
            national_dex: 157,
            abilities: [Some("Blaze"), Some("Blaze")],
        }),
        158 => Some(SpeciesInfo {
            name: "Totodile",
            national_dex: 158,
            abilities: [Some("Torrent"), Some("Torrent")],
        }),
        159 => Some(SpeciesInfo {
            name: "Croconaw",
            national_dex: 159,
            abilities: [Some("Torrent"), Some("Torrent")],
        }),
        160 => Some(SpeciesInfo {
            name: "Feraligatr",
            national_dex: 160,
            abilities: [Some("Torrent"), Some("Torrent")],
        }),
        161 => Some(SpeciesInfo {
            name: "Sentret",
            national_dex: 161,
            abilities: [Some("Run Away"), Some("Keen Eye")],
        }),
        162 => Some(SpeciesInfo {
            name: "Furret",
            national_dex: 162,
            abilities: [Some("Run Away"), Some("Keen Eye")],
        }),
        163 => Some(SpeciesInfo {
            name: "Hoothoot",
            national_dex: 163,
            abilities: [Some("Insomnia"), Some("Keen Eye")],
        }),
        164 => Some(SpeciesInfo {
            name: "Noctowl",
            national_dex: 164,
            abilities: [Some("Insomnia"), Some("Keen Eye")],
        }),
        165 => Some(SpeciesInfo {
            name: "Ledyba",
            national_dex: 165,
            abilities: [Some("Swarm"), Some("Early Bird")],
        }),
        166 => Some(SpeciesInfo {
            name: "Ledian",
            national_dex: 166,
            abilities: [Some("Swarm"), Some("Early Bird")],
        }),
        167 => Some(SpeciesInfo {
            name: "Spinarak",
            national_dex: 167,
            abilities: [Some("Swarm"), Some("Insomnia")],
        }),
        168 => Some(SpeciesInfo {
            name: "Ariados",
            national_dex: 168,
            abilities: [Some("Swarm"), Some("Insomnia")],
        }),
        169 => Some(SpeciesInfo {
            name: "Crobat",
            national_dex: 169,
            abilities: [Some("Inner Focus"), Some("Inner Focus")],
        }),
        170 => Some(SpeciesInfo {
            name: "Chinchou",
            national_dex: 170,
            abilities: [Some("Volt Absorb"), Some("Illuminate")],
        }),
        171 => Some(SpeciesInfo {
            name: "Lanturn",
            national_dex: 171,
            abilities: [Some("Volt Absorb"), Some("Illuminate")],
        }),
        172 => Some(SpeciesInfo {
            name: "Pichu",
            national_dex: 172,
            abilities: [Some("Static"), Some("Static")],
        }),
        173 => Some(SpeciesInfo {
            name: "Cleffa",
            national_dex: 173,
            abilities: [Some("Cute Charm"), Some("Cute Charm")],
        }),
        174 => Some(SpeciesInfo {
            name: "Igglybuff",
            national_dex: 174,
            abilities: [Some("Cute Charm"), Some("Cute Charm")],
        }),
        175 => Some(SpeciesInfo {
            name: "Togepi",
            national_dex: 175,
            abilities: [Some("Hustle"), Some("Serene Grace")],
        }),
        176 => Some(SpeciesInfo {
            name: "Togetic",
            national_dex: 176,
            abilities: [Some("Hustle"), Some("Serene Grace")],
        }),
        177 => Some(SpeciesInfo {
            name: "Natu",
            national_dex: 177,
            abilities: [Some("Synchronize"), Some("Early Bird")],
        }),
        178 => Some(SpeciesInfo {
            name: "Xatu",
            national_dex: 178,
            abilities: [Some("Synchronize"), Some("Early Bird")],
        }),
        179 => Some(SpeciesInfo {
            name: "Mareep",
            national_dex: 179,
            abilities: [Some("Static"), Some("Static")],
        }),
        180 => Some(SpeciesInfo {
            name: "Flaaffy",
            national_dex: 180,
            abilities: [Some("Static"), Some("Static")],
        }),
        181 => Some(SpeciesInfo {
            name: "Ampharos",
            national_dex: 181,
            abilities: [Some("Static"), Some("Static")],
        }),
        182 => Some(SpeciesInfo {
            name: "Bellossom",
            national_dex: 182,
            abilities: [Some("Chlorophyll"), Some("Chlorophyll")],
        }),
        183 => Some(SpeciesInfo {
            name: "Marill",
            national_dex: 183,
            abilities: [Some("Thick Fat"), Some("Huge Power")],
        }),
        184 => Some(SpeciesInfo {
            name: "Azumarill",
            national_dex: 184,
            abilities: [Some("Thick Fat"), Some("Huge Power")],
        }),
        185 => Some(SpeciesInfo {
            name: "Sudowoodo",
            national_dex: 185,
            abilities: [Some("Sturdy"), Some("Rock Head")],
        }),
        186 => Some(SpeciesInfo {
            name: "Politoed",
            national_dex: 186,
            abilities: [Some("Water Absorb"), Some("Damp")],
        }),
        187 => Some(SpeciesInfo {
            name: "Hoppip",
            national_dex: 187,
            abilities: [Some("Chlorophyll"), Some("Chlorophyll")],
        }),
        188 => Some(SpeciesInfo {
            name: "Skiploom",
            national_dex: 188,
            abilities: [Some("Chlorophyll"), Some("Chlorophyll")],
        }),
        189 => Some(SpeciesInfo {
            name: "Jumpluff",
            national_dex: 189,
            abilities: [Some("Chlorophyll"), Some("Chlorophyll")],
        }),
        190 => Some(SpeciesInfo {
            name: "Aipom",
            national_dex: 190,
            abilities: [Some("Run Away"), Some("Pickup")],
        }),
        191 => Some(SpeciesInfo {
            name: "Sunkern",
            national_dex: 191,
            abilities: [Some("Chlorophyll"), Some("Chlorophyll")],
        }),
        192 => Some(SpeciesInfo {
            name: "Sunflora",
            national_dex: 192,
            abilities: [Some("Chlorophyll"), Some("Chlorophyll")],
        }),
        193 => Some(SpeciesInfo {
            name: "Yanma",
            national_dex: 193,
            abilities: [Some("Speed Boost"), Some("Compound Eyes")],
        }),
        194 => Some(SpeciesInfo {
            name: "Wooper",
            national_dex: 194,
            abilities: [Some("Damp"), Some("Water Absorb")],
        }),
        195 => Some(SpeciesInfo {
            name: "Quagsire",
            national_dex: 195,
            abilities: [Some("Damp"), Some("Water Absorb")],
        }),
        196 => Some(SpeciesInfo {
            name: "Espeon",
            national_dex: 196,
            abilities: [Some("Synchronize"), Some("Synchronize")],
        }),
        197 => Some(SpeciesInfo {
            name: "Umbreon",
            national_dex: 197,
            abilities: [Some("Synchronize"), Some("Synchronize")],
        }),
        198 => Some(SpeciesInfo {
            name: "Murkrow",
            national_dex: 198,
            abilities: [Some("Insomnia"), Some("Insomnia")],
        }),
        199 => Some(SpeciesInfo {
            name: "Slowking",
            national_dex: 199,
            abilities: [Some("Oblivious"), Some("Own Tempo")],
        }),
        200 => Some(SpeciesInfo {
            name: "Misdreavus",
            national_dex: 200,
            abilities: [Some("Levitate"), Some("Levitate")],
        }),
        201 => Some(SpeciesInfo {
            name: "Unown",
            national_dex: 201,
            abilities: [Some("Levitate"), Some("Levitate")],
        }),
        202 => Some(SpeciesInfo {
            name: "Wobbuffet",
            national_dex: 202,
            abilities: [Some("Shadow Tag"), Some("Shadow Tag")],
        }),
        203 => Some(SpeciesInfo {
            name: "Girafarig",
            national_dex: 203,
            abilities: [Some("Inner Focus"), Some("Early Bird")],
        }),
        204 => Some(SpeciesInfo {
            name: "Pineco",
            national_dex: 204,
            abilities: [Some("Sturdy"), Some("Sturdy")],
        }),
        205 => Some(SpeciesInfo {
            name: "Forretress",
            national_dex: 205,
            abilities: [Some("Sturdy"), Some("Sturdy")],
        }),
        206 => Some(SpeciesInfo {
            name: "Dunsparce",
            national_dex: 206,
            abilities: [Some("Serene Grace"), Some("Run Away")],
        }),
        207 => Some(SpeciesInfo {
            name: "Gligar",
            national_dex: 207,
            abilities: [Some("Hyper Cutter"), Some("Sand Veil")],
        }),
        208 => Some(SpeciesInfo {
            name: "Steelix",
            national_dex: 208,
            abilities: [Some("Rock Head"), Some("Sturdy")],
        }),
        209 => Some(SpeciesInfo {
            name: "Snubbull",
            national_dex: 209,
            abilities: [Some("Intimidate"), Some("Run Away")],
        }),
        210 => Some(SpeciesInfo {
            name: "Granbull",
            national_dex: 210,
            abilities: [Some("Intimidate"), Some("Intimidate")],
        }),
        211 => Some(SpeciesInfo {
            name: "Qwilfish",
            national_dex: 211,
            abilities: [Some("Poison Point"), Some("Swift Swim")],
        }),
        212 => Some(SpeciesInfo {
            name: "Scizor",
            national_dex: 212,
            abilities: [Some("Swarm"), Some("Swarm")],
        }),
        213 => Some(SpeciesInfo {
            name: "Shuckle",
            national_dex: 213,
            abilities: [Some("Sturdy"), Some("Sturdy")],
        }),
        214 => Some(SpeciesInfo {
            name: "Heracross",
            national_dex: 214,
            abilities: [Some("Swarm"), Some("Guts")],
        }),
        215 => Some(SpeciesInfo {
            name: "Sneasel",
            national_dex: 215,
            abilities: [Some("Inner Focus"), Some("Keen Eye")],
        }),
        216 => Some(SpeciesInfo {
            name: "Teddiursa",
            national_dex: 216,
            abilities: [Some("Pickup"), Some("Pickup")],
        }),
        217 => Some(SpeciesInfo {
            name: "Ursaring",
            national_dex: 217,
            abilities: [Some("Guts"), Some("Guts")],
        }),
        218 => Some(SpeciesInfo {
            name: "Slugma",
            national_dex: 218,
            abilities: [Some("Magma Armor"), Some("Flame Body")],
        }),
        219 => Some(SpeciesInfo {
            name: "Magcargo",
            national_dex: 219,
            abilities: [Some("Magma Armor"), Some("Flame Body")],
        }),
        220 => Some(SpeciesInfo {
            name: "Swinub",
            national_dex: 220,
            abilities: [Some("Oblivious"), Some("Oblivious")],
        }),
        221 => Some(SpeciesInfo {
            name: "Piloswine",
            national_dex: 221,
            abilities: [Some("Oblivious"), Some("Oblivious")],
        }),
        222 => Some(SpeciesInfo {
            name: "Corsola",
            national_dex: 222,
            abilities: [Some("Hustle"), Some("Natural Cure")],
        }),
        223 => Some(SpeciesInfo {
            name: "Remoraid",
            national_dex: 223,
            abilities: [Some("Hustle"), Some("Hustle")],
        }),
        224 => Some(SpeciesInfo {
            name: "Octillery",
            national_dex: 224,
            abilities: [Some("Suction Cups"), Some("Suction Cups")],
        }),
        225 => Some(SpeciesInfo {
            name: "Delibird",
            national_dex: 225,
            abilities: [Some("Vital Spirit"), Some("Hustle")],
        }),
        226 => Some(SpeciesInfo {
            name: "Mantine",
            national_dex: 226,
            abilities: [Some("Swift Swim"), Some("Water Absorb")],
        }),
        227 => Some(SpeciesInfo {
            name: "Skarmory",
            national_dex: 227,
            abilities: [Some("Keen Eye"), Some("Sturdy")],
        }),
        228 => Some(SpeciesInfo {
            name: "Houndour",
            national_dex: 228,
            abilities: [Some("Early Bird"), Some("Flash Fire")],
        }),
        229 => Some(SpeciesInfo {
            name: "Houndoom",
            national_dex: 229,
            abilities: [Some("Early Bird"), Some("Flash Fire")],
        }),
        230 => Some(SpeciesInfo {
            name: "Kingdra",
            national_dex: 230,
            abilities: [Some("Swift Swim"), Some("Swift Swim")],
        }),
        231 => Some(SpeciesInfo {
            name: "Phanpy",
            national_dex: 231,
            abilities: [Some("Pickup"), Some("Pickup")],
        }),
        232 => Some(SpeciesInfo {
            name: "Donphan",
            national_dex: 232,
            abilities: [Some("Sturdy"), Some("Sturdy")],
        }),
        233 => Some(SpeciesInfo {
            name: "Porygon2",
            national_dex: 233,
            abilities: [Some("Trace"), Some("Trace")],
        }),
        234 => Some(SpeciesInfo {
            name: "Stantler",
            national_dex: 234,
            abilities: [Some("Intimidate"), Some("Intimidate")],
        }),
        235 => Some(SpeciesInfo {
            name: "Smeargle",
            national_dex: 235,
            abilities: [Some("Own Tempo"), Some("Own Tempo")],
        }),
        236 => Some(SpeciesInfo {
            name: "Tyrogue",
            national_dex: 236,
            abilities: [Some("Guts"), Some("Guts")],
        }),
        237 => Some(SpeciesInfo {
            name: "Hitmontop",
            national_dex: 237,
            abilities: [Some("Intimidate"), Some("Intimidate")],
        }),
        238 => Some(SpeciesInfo {
            name: "Smoochum",
            national_dex: 238,
            abilities: [Some("Oblivious"), Some("Oblivious")],
        }),
        239 => Some(SpeciesInfo {
            name: "Elekid",
            national_dex: 239,
            abilities: [Some("Static"), Some("Static")],
        }),
        240 => Some(SpeciesInfo {
            name: "Magby",
            national_dex: 240,
            abilities: [Some("Flame Body"), Some("Flame Body")],
        }),
        241 => Some(SpeciesInfo {
            name: "Miltank",
            national_dex: 241,
            abilities: [Some("Thick Fat"), Some("Thick Fat")],
        }),
        242 => Some(SpeciesInfo {
            name: "Blissey",
            national_dex: 242,
            abilities: [Some("Natural Cure"), Some("Serene Grace")],
        }),
        243 => Some(SpeciesInfo {
            name: "Raikou",
            national_dex: 243,
            abilities: [Some("Pressure"), Some("Pressure")],
        }),
        244 => Some(SpeciesInfo {
            name: "Entei",
            national_dex: 244,
            abilities: [Some("Pressure"), Some("Pressure")],
        }),
        245 => Some(SpeciesInfo {
            name: "Suicune",
            national_dex: 245,
            abilities: [Some("Pressure"), Some("Pressure")],
        }),
        246 => Some(SpeciesInfo {
            name: "Larvitar",
            national_dex: 246,
            abilities: [Some("Guts"), Some("Guts")],
        }),
        247 => Some(SpeciesInfo {
            name: "Pupitar",
            national_dex: 247,
            abilities: [Some("Shed Skin"), Some("Shed Skin")],
        }),
        248 => Some(SpeciesInfo {
            name: "Tyranitar",
            national_dex: 248,
            abilities: [Some("Sand Stream"), Some("Sand Stream")],
        }),
        249 => Some(SpeciesInfo {
            name: "Lugia",
            national_dex: 249,
            abilities: [Some("Pressure"), Some("Pressure")],
        }),
        250 => Some(SpeciesInfo {
            name: "Ho-Oh",
            national_dex: 250,
            abilities: [Some("Pressure"), Some("Pressure")],
        }),
        251 => Some(SpeciesInfo {
            name: "Celebi",
            national_dex: 251,
            abilities: [Some("Natural Cure"), Some("Natural Cure")],
        }),
        277 => Some(SpeciesInfo {
            name: "Treecko",
            national_dex: 252,
            abilities: [Some("Overgrow"), Some("Overgrow")],
        }),
        278 => Some(SpeciesInfo {
            name: "Grovyle",
            national_dex: 253,
            abilities: [Some("Overgrow"), Some("Overgrow")],
        }),
        279 => Some(SpeciesInfo {
            name: "Sceptile",
            national_dex: 254,
            abilities: [Some("Overgrow"), Some("Overgrow")],
        }),
        280 => Some(SpeciesInfo {
            name: "Torchic",
            national_dex: 255,
            abilities: [Some("Blaze"), Some("Blaze")],
        }),
        281 => Some(SpeciesInfo {
            name: "Combusken",
            national_dex: 256,
            abilities: [Some("Blaze"), Some("Blaze")],
        }),
        282 => Some(SpeciesInfo {
            name: "Blaziken",
            national_dex: 257,
            abilities: [Some("Blaze"), Some("Blaze")],
        }),
        283 => Some(SpeciesInfo {
            name: "Mudkip",
            national_dex: 258,
            abilities: [Some("Torrent"), Some("Torrent")],
        }),
        284 => Some(SpeciesInfo {
            name: "Marshtomp",
            national_dex: 259,
            abilities: [Some("Torrent"), Some("Torrent")],
        }),
        285 => Some(SpeciesInfo {
            name: "Swampert",
            national_dex: 260,
            abilities: [Some("Torrent"), Some("Torrent")],
        }),
        286 => Some(SpeciesInfo {
            name: "Poochyena",
            national_dex: 261,
            abilities: [Some("Run Away"), Some("Run Away")],
        }),
        287 => Some(SpeciesInfo {
            name: "Mightyena",
            national_dex: 262,
            abilities: [Some("Intimidate"), Some("Intimidate")],
        }),
        288 => Some(SpeciesInfo {
            name: "Zigzagoon",
            national_dex: 263,
            abilities: [Some("Pickup"), Some("Pickup")],
        }),
        289 => Some(SpeciesInfo {
            name: "Linoone",
            national_dex: 264,
            abilities: [Some("Pickup"), Some("Pickup")],
        }),
        290 => Some(SpeciesInfo {
            name: "Wurmple",
            national_dex: 265,
            abilities: [Some("Shield Dust"), Some("Shield Dust")],
        }),
        291 => Some(SpeciesInfo {
            name: "Silcoon",
            national_dex: 266,
            abilities: [Some("Shed Skin"), Some("Shed Skin")],
        }),
        292 => Some(SpeciesInfo {
            name: "Beautifly",
            national_dex: 267,
            abilities: [Some("Swarm"), Some("Swarm")],
        }),
        293 => Some(SpeciesInfo {
            name: "Cascoon",
            national_dex: 268,
            abilities: [Some("Shed Skin"), Some("Shed Skin")],
        }),
        294 => Some(SpeciesInfo {
            name: "Dustox",
            national_dex: 269,
            abilities: [Some("Shield Dust"), Some("Shield Dust")],
        }),
        295 => Some(SpeciesInfo {
            name: "Lotad",
            national_dex: 270,
            abilities: [Some("Swift Swim"), Some("Rain Dish")],
        }),
        296 => Some(SpeciesInfo {
            name: "Lombre",
            national_dex: 271,
            abilities: [Some("Swift Swim"), Some("Rain Dish")],
        }),
        297 => Some(SpeciesInfo {
            name: "Ludicolo",
            national_dex: 272,
            abilities: [Some("Swift Swim"), Some("Rain Dish")],
        }),
        298 => Some(SpeciesInfo {
            name: "Seedot",
            national_dex: 273,
            abilities: [Some("Chlorophyll"), Some("Early Bird")],
        }),
        299 => Some(SpeciesInfo {
            name: "Nuzleaf",
            national_dex: 274,
            abilities: [Some("Chlorophyll"), Some("Early Bird")],
        }),
        300 => Some(SpeciesInfo {
            name: "Shiftry",
            national_dex: 275,
            abilities: [Some("Chlorophyll"), Some("Early Bird")],
        }),
        301 => Some(SpeciesInfo {
            name: "Nincada",
            national_dex: 290,
            abilities: [Some("Compound Eyes"), Some("Compound Eyes")],
        }),
        302 => Some(SpeciesInfo {
            name: "Ninjask",
            national_dex: 291,
            abilities: [Some("Speed Boost"), Some("Speed Boost")],
        }),
        303 => Some(SpeciesInfo {
            name: "Shedinja",
            national_dex: 292,
            abilities: [Some("Wonder Guard"), Some("Wonder Guard")],
        }),
        304 => Some(SpeciesInfo {
            name: "Taillow",
            national_dex: 276,
            abilities: [Some("Guts"), Some("Guts")],
        }),
        305 => Some(SpeciesInfo {
            name: "Swellow",
            national_dex: 277,
            abilities: [Some("Guts"), Some("Guts")],
        }),
        306 => Some(SpeciesInfo {
            name: "Shroomish",
            national_dex: 285,
            abilities: [Some("Effect Spore"), Some("Effect Spore")],
        }),
        307 => Some(SpeciesInfo {
            name: "Breloom",
            national_dex: 286,
            abilities: [Some("Effect Spore"), Some("Effect Spore")],
        }),
        308 => Some(SpeciesInfo {
            name: "Spinda",
            national_dex: 327,
            abilities: [Some("Own Tempo"), Some("Own Tempo")],
        }),
        309 => Some(SpeciesInfo {
            name: "Wingull",
            national_dex: 278,
            abilities: [Some("Keen Eye"), Some("Keen Eye")],
        }),
        310 => Some(SpeciesInfo {
            name: "Pelipper",
            national_dex: 279,
            abilities: [Some("Keen Eye"), Some("Keen Eye")],
        }),
        311 => Some(SpeciesInfo {
            name: "Surskit",
            national_dex: 283,
            abilities: [Some("Swift Swim"), Some("Swift Swim")],
        }),
        312 => Some(SpeciesInfo {
            name: "Masquerain",
            national_dex: 284,
            abilities: [Some("Intimidate"), Some("Intimidate")],
        }),
        313 => Some(SpeciesInfo {
            name: "Wailmer",
            national_dex: 320,
            abilities: [Some("Water Veil"), Some("Oblivious")],
        }),
        314 => Some(SpeciesInfo {
            name: "Wailord",
            national_dex: 321,
            abilities: [Some("Water Veil"), Some("Oblivious")],
        }),
        315 => Some(SpeciesInfo {
            name: "Skitty",
            national_dex: 300,
            abilities: [Some("Cute Charm"), Some("Cute Charm")],
        }),
        316 => Some(SpeciesInfo {
            name: "Delcatty",
            national_dex: 301,
            abilities: [Some("Cute Charm"), Some("Cute Charm")],
        }),
        317 => Some(SpeciesInfo {
            name: "Kecleon",
            national_dex: 352,
            abilities: [Some("Color Change"), Some("Color Change")],
        }),
        318 => Some(SpeciesInfo {
            name: "Baltoy",
            national_dex: 343,
            abilities: [Some("Levitate"), Some("Levitate")],
        }),
        319 => Some(SpeciesInfo {
            name: "Claydol",
            national_dex: 344,
            abilities: [Some("Levitate"), Some("Levitate")],
        }),
        320 => Some(SpeciesInfo {
            name: "Nosepass",
            national_dex: 299,
            abilities: [Some("Sturdy"), Some("Magnet Pull")],
        }),
        321 => Some(SpeciesInfo {
            name: "Torkoal",
            national_dex: 324,
            abilities: [Some("White Smoke"), Some("White Smoke")],
        }),
        322 => Some(SpeciesInfo {
            name: "Sableye",
            national_dex: 302,
            abilities: [Some("Keen Eye"), Some("Keen Eye")],
        }),
        323 => Some(SpeciesInfo {
            name: "Barboach",
            national_dex: 339,
            abilities: [Some("Oblivious"), Some("Oblivious")],
        }),
        324 => Some(SpeciesInfo {
            name: "Whiscash",
            national_dex: 340,
            abilities: [Some("Oblivious"), Some("Oblivious")],
        }),
        325 => Some(SpeciesInfo {
            name: "Luvdisc",
            national_dex: 370,
            abilities: [Some("Swift Swim"), Some("Swift Swim")],
        }),
        326 => Some(SpeciesInfo {
            name: "Corphish",
            national_dex: 341,
            abilities: [Some("Hyper Cutter"), Some("Shell Armor")],
        }),
        327 => Some(SpeciesInfo {
            name: "Crawdaunt",
            national_dex: 342,
            abilities: [Some("Hyper Cutter"), Some("Shell Armor")],
        }),
        328 => Some(SpeciesInfo {
            name: "Feebas",
            national_dex: 349,
            abilities: [Some("Swift Swim"), Some("Swift Swim")],
        }),
        329 => Some(SpeciesInfo {
            name: "Milotic",
            national_dex: 350,
            abilities: [Some("Marvel Scale"), Some("Marvel Scale")],
        }),
        330 => Some(SpeciesInfo {
            name: "Carvanha",
            national_dex: 318,
            abilities: [Some("Rough Skin"), Some("Rough Skin")],
        }),
        331 => Some(SpeciesInfo {
            name: "Sharpedo",
            national_dex: 319,
            abilities: [Some("Rough Skin"), Some("Rough Skin")],
        }),
        332 => Some(SpeciesInfo {
            name: "Trapinch",
            national_dex: 328,
            abilities: [Some("Hyper Cutter"), Some("Arena Trap")],
        }),
        333 => Some(SpeciesInfo {
            name: "Vibrava",
            national_dex: 329,
            abilities: [Some("Levitate"), Some("Levitate")],
        }),
        334 => Some(SpeciesInfo {
            name: "Flygon",
            national_dex: 330,
            abilities: [Some("Levitate"), Some("Levitate")],
        }),
        335 => Some(SpeciesInfo {
            name: "Makuhita",
            national_dex: 296,
            abilities: [Some("Thick Fat"), Some("Guts")],
        }),
        336 => Some(SpeciesInfo {
            name: "Hariyama",
            national_dex: 297,
            abilities: [Some("Thick Fat"), Some("Guts")],
        }),
        337 => Some(SpeciesInfo {
            name: "Electrike",
            national_dex: 309,
            abilities: [Some("Static"), Some("Lightning Rod")],
        }),
        338 => Some(SpeciesInfo {
            name: "Manectric",
            national_dex: 310,
            abilities: [Some("Static"), Some("Lightning Rod")],
        }),
        339 => Some(SpeciesInfo {
            name: "Numel",
            national_dex: 322,
            abilities: [Some("Oblivious"), Some("Oblivious")],
        }),
        340 => Some(SpeciesInfo {
            name: "Camerupt",
            national_dex: 323,
            abilities: [Some("Magma Armor"), Some("Magma Armor")],
        }),
        341 => Some(SpeciesInfo {
            name: "Spheal",
            national_dex: 363,
            abilities: [Some("Thick Fat"), Some("Thick Fat")],
        }),
        342 => Some(SpeciesInfo {
            name: "Sealeo",
            national_dex: 364,
            abilities: [Some("Thick Fat"), Some("Thick Fat")],
        }),
        343 => Some(SpeciesInfo {
            name: "Walrein",
            national_dex: 365,
            abilities: [Some("Thick Fat"), Some("Thick Fat")],
        }),
        344 => Some(SpeciesInfo {
            name: "Cacnea",
            national_dex: 331,
            abilities: [Some("Sand Veil"), Some("Sand Veil")],
        }),
        345 => Some(SpeciesInfo {
            name: "Cacturne",
            national_dex: 332,
            abilities: [Some("Sand Veil"), Some("Sand Veil")],
        }),
        346 => Some(SpeciesInfo {
            name: "Snorunt",
            national_dex: 361,
            abilities: [Some("Inner Focus"), Some("Inner Focus")],
        }),
        347 => Some(SpeciesInfo {
            name: "Glalie",
            national_dex: 362,
            abilities: [Some("Inner Focus"), Some("Inner Focus")],
        }),
        348 => Some(SpeciesInfo {
            name: "Lunatone",
            national_dex: 337,
            abilities: [Some("Levitate"), Some("Levitate")],
        }),
        349 => Some(SpeciesInfo {
            name: "Solrock",
            national_dex: 338,
            abilities: [Some("Levitate"), Some("Levitate")],
        }),
        350 => Some(SpeciesInfo {
            name: "Azurill",
            national_dex: 298,
            abilities: [Some("Thick Fat"), Some("Huge Power")],
        }),
        351 => Some(SpeciesInfo {
            name: "Spoink",
            national_dex: 325,
            abilities: [Some("Thick Fat"), Some("Own Tempo")],
        }),
        352 => Some(SpeciesInfo {
            name: "Grumpig",
            national_dex: 326,
            abilities: [Some("Thick Fat"), Some("Own Tempo")],
        }),
        353 => Some(SpeciesInfo {
            name: "Plusle",
            national_dex: 311,
            abilities: [Some("Plus"), Some("Plus")],
        }),
        354 => Some(SpeciesInfo {
            name: "Minun",
            national_dex: 312,
            abilities: [Some("Minus"), Some("Minus")],
        }),
        355 => Some(SpeciesInfo {
            name: "Mawile",
            national_dex: 303,
            abilities: [Some("Hyper Cutter"), Some("Intimidate")],
        }),
        356 => Some(SpeciesInfo {
            name: "Meditite",
            national_dex: 307,
            abilities: [Some("Pure Power"), Some("Pure Power")],
        }),
        357 => Some(SpeciesInfo {
            name: "Medicham",
            national_dex: 308,
            abilities: [Some("Pure Power"), Some("Pure Power")],
        }),
        358 => Some(SpeciesInfo {
            name: "Swablu",
            national_dex: 333,
            abilities: [Some("Natural Cure"), Some("Natural Cure")],
        }),
        359 => Some(SpeciesInfo {
            name: "Altaria",
            national_dex: 334,
            abilities: [Some("Natural Cure"), Some("Natural Cure")],
        }),
        360 => Some(SpeciesInfo {
            name: "Wynaut",
            national_dex: 360,
            abilities: [Some("Shadow Tag"), Some("Shadow Tag")],
        }),
        361 => Some(SpeciesInfo {
            name: "Duskull",
            national_dex: 355,
            abilities: [Some("Levitate"), Some("Levitate")],
        }),
        362 => Some(SpeciesInfo {
            name: "Dusclops",
            national_dex: 356,
            abilities: [Some("Pressure"), Some("Pressure")],
        }),
        363 => Some(SpeciesInfo {
            name: "Roselia",
            national_dex: 315,
            abilities: [Some("Natural Cure"), Some("Poison Point")],
        }),
        364 => Some(SpeciesInfo {
            name: "Slakoth",
            national_dex: 287,
            abilities: [Some("Truant"), Some("Truant")],
        }),
        365 => Some(SpeciesInfo {
            name: "Vigoroth",
            national_dex: 288,
            abilities: [Some("Vital Spirit"), Some("Vital Spirit")],
        }),
        366 => Some(SpeciesInfo {
            name: "Slaking",
            national_dex: 289,
            abilities: [Some("Truant"), Some("Truant")],
        }),
        367 => Some(SpeciesInfo {
            name: "Gulpin",
            national_dex: 316,
            abilities: [Some("Liquid Ooze"), Some("Sticky Hold")],
        }),
        368 => Some(SpeciesInfo {
            name: "Swalot",
            national_dex: 317,
            abilities: [Some("Liquid Ooze"), Some("Sticky Hold")],
        }),
        369 => Some(SpeciesInfo {
            name: "Tropius",
            national_dex: 357,
            abilities: [Some("Chlorophyll"), Some("Chlorophyll")],
        }),
        370 => Some(SpeciesInfo {
            name: "Whismur",
            national_dex: 293,
            abilities: [Some("Soundproof"), Some("Soundproof")],
        }),
        371 => Some(SpeciesInfo {
            name: "Loudred",
            national_dex: 294,
            abilities: [Some("Soundproof"), Some("Soundproof")],
        }),
        372 => Some(SpeciesInfo {
            name: "Exploud",
            national_dex: 295,
            abilities: [Some("Soundproof"), Some("Soundproof")],
        }),
        373 => Some(SpeciesInfo {
            name: "Clamperl",
            national_dex: 366,
            abilities: [Some("Shell Armor"), Some("Shell Armor")],
        }),
        374 => Some(SpeciesInfo {
            name: "Huntail",
            national_dex: 367,
            abilities: [Some("Swift Swim"), Some("Swift Swim")],
        }),
        375 => Some(SpeciesInfo {
            name: "Gorebyss",
            national_dex: 368,
            abilities: [Some("Swift Swim"), Some("Swift Swim")],
        }),
        376 => Some(SpeciesInfo {
            name: "Absol",
            national_dex: 359,
            abilities: [Some("Pressure"), Some("Pressure")],
        }),
        377 => Some(SpeciesInfo {
            name: "Shuppet",
            national_dex: 353,
            abilities: [Some("Insomnia"), Some("Insomnia")],
        }),
        378 => Some(SpeciesInfo {
            name: "Banette",
            national_dex: 354,
            abilities: [Some("Insomnia"), Some("Insomnia")],
        }),
        379 => Some(SpeciesInfo {
            name: "Seviper",
            national_dex: 336,
            abilities: [Some("Shed Skin"), Some("Shed Skin")],
        }),
        380 => Some(SpeciesInfo {
            name: "Zangoose",
            national_dex: 335,
            abilities: [Some("Immunity"), Some("Immunity")],
        }),
        381 => Some(SpeciesInfo {
            name: "Relicanth",
            national_dex: 369,
            abilities: [Some("Swift Swim"), Some("Rock Head")],
        }),
        382 => Some(SpeciesInfo {
            name: "Aron",
            national_dex: 304,
            abilities: [Some("Sturdy"), Some("Rock Head")],
        }),
        383 => Some(SpeciesInfo {
            name: "Lairon",
            national_dex: 305,
            abilities: [Some("Sturdy"), Some("Rock Head")],
        }),
        384 => Some(SpeciesInfo {
            name: "Aggron",
            national_dex: 306,
            abilities: [Some("Sturdy"), Some("Rock Head")],
        }),
        385 => Some(SpeciesInfo {
            name: "Castform",
            national_dex: 351,
            abilities: [Some("Forecast"), Some("Forecast")],
        }),
        386 => Some(SpeciesInfo {
            name: "Volbeat",
            national_dex: 313,
            abilities: [Some("Illuminate"), Some("Swarm")],
        }),
        387 => Some(SpeciesInfo {
            name: "Illumise",
            national_dex: 314,
            abilities: [Some("Oblivious"), Some("Oblivious")],
        }),
        388 => Some(SpeciesInfo {
            name: "Lileep",
            national_dex: 345,
            abilities: [Some("Suction Cups"), Some("Suction Cups")],
        }),
        389 => Some(SpeciesInfo {
            name: "Cradily",
            national_dex: 346,
            abilities: [Some("Suction Cups"), Some("Suction Cups")],
        }),
        390 => Some(SpeciesInfo {
            name: "Anorith",
            national_dex: 347,
            abilities: [Some("Battle Armor"), Some("Battle Armor")],
        }),
        391 => Some(SpeciesInfo {
            name: "Armaldo",
            national_dex: 348,
            abilities: [Some("Battle Armor"), Some("Battle Armor")],
        }),
        392 => Some(SpeciesInfo {
            name: "Ralts",
            national_dex: 280,
            abilities: [Some("Synchronize"), Some("Trace")],
        }),
        393 => Some(SpeciesInfo {
            name: "Kirlia",
            national_dex: 281,
            abilities: [Some("Synchronize"), Some("Trace")],
        }),
        394 => Some(SpeciesInfo {
            name: "Gardevoir",
            national_dex: 282,
            abilities: [Some("Synchronize"), Some("Trace")],
        }),
        395 => Some(SpeciesInfo {
            name: "Bagon",
            national_dex: 371,
            abilities: [Some("Rock Head"), Some("Rock Head")],
        }),
        396 => Some(SpeciesInfo {
            name: "Shelgon",
            national_dex: 372,
            abilities: [Some("Rock Head"), Some("Rock Head")],
        }),
        397 => Some(SpeciesInfo {
            name: "Salamence",
            national_dex: 373,
            abilities: [Some("Intimidate"), Some("Intimidate")],
        }),
        398 => Some(SpeciesInfo {
            name: "Beldum",
            national_dex: 374,
            abilities: [Some("Clear Body"), Some("Clear Body")],
        }),
        399 => Some(SpeciesInfo {
            name: "Metang",
            national_dex: 375,
            abilities: [Some("Clear Body"), Some("Clear Body")],
        }),
        400 => Some(SpeciesInfo {
            name: "Metagross",
            national_dex: 376,
            abilities: [Some("Clear Body"), Some("Clear Body")],
        }),
        401 => Some(SpeciesInfo {
            name: "Regirock",
            national_dex: 377,
            abilities: [Some("Clear Body"), Some("Clear Body")],
        }),
        402 => Some(SpeciesInfo {
            name: "Regice",
            national_dex: 378,
            abilities: [Some("Clear Body"), Some("Clear Body")],
        }),
        403 => Some(SpeciesInfo {
            name: "Registeel",
            national_dex: 379,
            abilities: [Some("Clear Body"), Some("Clear Body")],
        }),
        404 => Some(SpeciesInfo {
            name: "Kyogre",
            national_dex: 382,
            abilities: [Some("Drizzle"), Some("Drizzle")],
        }),
        405 => Some(SpeciesInfo {
            name: "Groudon",
            national_dex: 383,
            abilities: [Some("Drought"), Some("Drought")],
        }),
        406 => Some(SpeciesInfo {
            name: "Rayquaza",
            national_dex: 384,
            abilities: [Some("Air Lock"), Some("Air Lock")],
        }),
        407 => Some(SpeciesInfo {
            name: "Latias",
            national_dex: 380,
            abilities: [Some("Levitate"), Some("Levitate")],
        }),
        408 => Some(SpeciesInfo {
            name: "Latios",
            national_dex: 381,
            abilities: [Some("Levitate"), Some("Levitate")],
        }),
        409 => Some(SpeciesInfo {
            name: "Jirachi",
            national_dex: 385,
            abilities: [Some("Serene Grace"), Some("Serene Grace")],
        }),
        410 => Some(SpeciesInfo {
            name: "Deoxys",
            national_dex: 386,
            abilities: [Some("Pressure"), Some("Pressure")],
        }),
        411 => Some(SpeciesInfo {
            name: "Chimecho",
            national_dex: 358,
            abilities: [None, None],
        }),
        _ => None,
    }
}

pub fn ability_name(species: u16, ability_number: u8) -> &'static str {
    let Some(info) = species_info(species) else {
        return "Unknown";
    };

    let index = if ability_number == 1 { 1 } else { 0 };

    info.abilities[index]
        .or(info.abilities[0])
        .unwrap_or("Unknown")
}

pub fn nature_name(nature_id: u8) -> &'static str {
    match nature_id {
        0 => "Hardy",
        1 => "Lonely",
        2 => "Brave",
        3 => "Adamant",
        4 => "Naughty",
        5 => "Bold",
        6 => "Docile",
        7 => "Relaxed",
        8 => "Impish",
        9 => "Lax",
        10 => "Timid",
        11 => "Hasty",
        12 => "Serious",
        13 => "Jolly",
        14 => "Naive",
        15 => "Modest",
        16 => "Mild",
        17 => "Quiet",
        18 => "Bashful",
        19 => "Rash",
        20 => "Calm",
        21 => "Gentle",
        22 => "Sassy",
        23 => "Careful",
        24 => "Quirky",
        _ => "Unknown",
    }
}
