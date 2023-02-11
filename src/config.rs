// https://developer.gnome.org/documentation/tutorials/application-id.html
pub const APP_ID: &str = "com.radioactivehamster.pocket_up";

pub const APP_NAME: &str = "PocketUp";

pub struct PocketCore {
    pub name: &'static str,
    pub author: &'static str,
    pub repo: &'static str,
}

pub const POCKET_CORES: [PocketCore; 22] = [
    PocketCore {
        name: "Amiga 500",
        author: "Mazamars312",
        repo: "https://github.com/Mazamars312/Analogue-Amiga",
    },
    PocketCore {
        name: "Arduboy for Analogue Pocket",
        author: "agg23",
        repo: "https://github.com/agg23/openfpga-arduboy",
    },
    PocketCore {
        name: "Asteroids for Analogue Pocket",
        author: "ericlewis",
        repo: "https://github.com/ericlewis/openfpga-asteroids",
    },
    PocketCore {
        name: "Atari's 1972 Pong",
        author: "agg23",
        repo: "https://github.com/agg23/openfpga-pong",
    },
    PocketCore {
        name: "Bank Panic",
        author: "opengateware",
        repo: "https://github.com/opengateware/arcade-bankp",
    },
    // TODO: psomashekar's releases are all under one singular repo. Need to
    // figure out how to handle that.
    PocketCore {
        name: "Batrider",
        author: "psomashekar",
        repo: "https://github.com/psomashekar/pram0d-pocket-dist-public",
    },
    PocketCore {
        name: "Battle Bakraid",
        author: "psomashekar",
        repo: "https://github.com/psomashekar/pram0d-pocket-dist-public",
    },
    PocketCore {
        name: "Battle Garegga",
        author: "psomashekar",
        repo: "https://github.com/psomashekar/pram0d-pocket-dist-public",
    },
    PocketCore {
        name: "Congo Bongo",
        author: "opengateware",
        repo: "https://github.com/opengateware/arcade-congo",
    },
    // TODO: jotego's releases are all under one singular repo. Need to
    // figure out how to handle that.
    PocketCore {
        name: "Contra",
        author: "jotego",
        repo: "https://github.com/jotego/jtbin",
    },
    PocketCore {
        name: "Dig Dug",
        author: "opengateware",
        repo: "https://github.com/opengateware/arcade-digdug",
    },
    PocketCore {
        name: "Dominos for Analogue Pocket",
        author: "ericlewis",
        repo: "https://github.com/ericlewis/openfpga-dominos",
    },
    PocketCore {
        name: "Donkey Kong",
        author: "ericlewis",
        repo: "https://github.com/ericlewis/openFPGA-DonkeyKong",
    },
    PocketCore {
        name: "Double Dragon",
        author: "jotego",
        repo: "https://github.com/jotego/jtbin",
    },
    PocketCore {
        name: "Double Dragon II",
        author: "jotego",
        repo: "https://github.com/jotego/jtbin",
    },
    PocketCore {
        name: "Galaga",
        author: "opengateware",
        repo: "https://github.com/opengateware/arcade-galaga",
    },
    PocketCore {
        name: "Genesis for Analogue Pocket",
        author: "opengateware",
        repo: "https://github.com/opengateware/openfpga-genesis",
    },
    PocketCore {
        name: "Ghosts 'n Goblins",
        author: "jotego",
        repo: "https://github.com/jotego/jtbin",
    },
    PocketCore {
        name: "Green Beret",
        author: "opengateware",
        repo: "https://github.com/opengateware/arcade-gberet",
    },
    PocketCore {
        name: "Kicker",
        author: "jotego",
        repo: "https://github.com/jotego/jtbin",
    },
    PocketCore {
        name: "Kingdom Grandprix",
        author: "psomashekar",
        repo: "https://github.com/psomashekar/pram0d-pocket-dist-public",
    },
    PocketCore {
        name: "Konami's Ping Pong",
        author: "jotego",
        repo: "https://github.com/jotego/jtbin",
    },
];
