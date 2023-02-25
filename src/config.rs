use heck::ToKebabCase;

// https://developer.gnome.org/documentation/tutorials/application-id.html
pub const APP_ID: &str = "com.radioactivehamster.pocket_up";

pub const APP_NAME: &str = "PocketUp";

pub const COLUMN_WIDTH: usize = 80;

pub struct PocketCore {
    // The name of the core as specified on https://openfpga-cores-inventory.github.io/analogue-pocket/
    pub name: &'static str,

    // The author of the core
    pub author: &'static str,

    // The core's repository URL
    pub repo: &'static str,

    // For cores that don't use GitHub releases which have a static download URL
    pub download_url: Option<&'static str>,
}

impl PocketCore {
    pub fn description_markup(&self) -> String {
        format!(r#"<b>{}</b> by <i>{}</i>"#, self.name, self.author)
    }

    pub fn settings_name(&self) -> String {
        format!("Download Core {} by {}", self.name, self.author).to_kebab_case()
    }
}

pub const POCKET_CORES: [PocketCore; 35] = [
    PocketCore {
        name: "Amiga 500",
        author: "Mazamars312",
        repo: "https://github.com/Mazamars312/Analogue-Amiga",
        download_url: None,
    },
    PocketCore {
        name: "Arduboy for Analogue Pocket",
        author: "agg23",
        repo: "https://github.com/agg23/openfpga-arduboy",
        download_url: None,
    },
    PocketCore {
        name: "Asteroids for Analogue Pocket",
        author: "ericlewis",
        repo: "https://github.com/ericlewis/openfpga-asteroids",
        download_url: None,
    },
    PocketCore {
        name: "Atari's 1972 Pong",
        author: "agg23",
        repo: "https://github.com/agg23/openfpga-pong",
        download_url: None,
    },
    PocketCore {
        name: "Bank Panic",
        author: "opengateware",
        repo: "https://github.com/opengateware/arcade-bankp",
        download_url: None,
    },
    // TODO: psomashekar's releases are all under one singular repo. Need to
    // figure out how to handle that.
    PocketCore {
        name: "Batrider",
        author: "psomashekar",
        repo: "https://github.com/psomashekar/pram0d-pocket-dist-public",
        download_url: Some("https://github.com/psomashekar/pram0d-pocket-dist-public/raw/develop/releases/pram0d.batrider_20221218.zip"),
    },
    PocketCore {
        name: "Battle Bakraid",
        author: "psomashekar",
        repo: "https://github.com/psomashekar/pram0d-pocket-dist-public",
        download_url: Some("https://github.com/psomashekar/pram0d-pocket-dist-public/raw/develop/releases/pram0d.bakraid_20221218.zip"),
    },
    PocketCore {
        name: "Battle Garegga",
        author: "psomashekar",
        repo: "https://github.com/psomashekar/pram0d-pocket-dist-public",
        download_url: Some("https://github.com/psomashekar/pram0d-pocket-dist-public/raw/develop/releases/pram0d.garegga_20221030.zip"),
    },
    PocketCore {
        name: "Congo Bongo",
        author: "opengateware",
        repo: "https://github.com/opengateware/arcade-congo",
        download_url: None,
    },
    PocketCore {
        name: "Contra",
        author: "jotego",
        repo: "https://github.com/jotego/jtbin",
        download_url: Some("https://github.com/jotego/jtbin/raw/master/pocket/zips/jotego.jtcontra.zip"),
    },
    PocketCore {
        name: "Dig Dug",
        author: "opengateware",
        repo: "https://github.com/opengateware/arcade-digdug",
        download_url: None,
    },
    PocketCore {
        name: "Dominos for Analogue Pocket",
        author: "ericlewis",
        repo: "https://github.com/ericlewis/openfpga-dominos",
        download_url: None,
    },
    // TODO: This is a pre-release so it doesn't show up using the GitHub latest release API. Work around this!
    PocketCore {
        name: "Donkey Kong",
        author: "ericlewis",
        repo: "https://github.com/ericlewis/openFPGA-DonkeyKong",
        download_url: None,
    },
    PocketCore {
        name: "Double Dragon",
        author: "jotego",
        repo: "https://github.com/jotego/jtbin",
        download_url: Some("https://github.com/jotego/jtbin/raw/master/pocket/zips/jotego.jtdd.zip"),
    },
    PocketCore {
        name: "Double Dragon II",
        author: "jotego",
        repo: "https://github.com/jotego/jtbin",
        download_url: Some("https://github.com/jotego/jtbin/raw/master/pocket/zips/jotego.jtdd2.zip"),
    },
    PocketCore {
        name: "Galaga",
        author: "opengateware",
        repo: "https://github.com/opengateware/arcade-galaga",
        download_url: None,
    },
    PocketCore {
        name: "Genesis for Analogue Pocket",
        author: "opengateware",
        repo: "https://github.com/opengateware/openfpga-genesis",
        download_url: None,
    },
    PocketCore {
        name: "Ghosts 'n Goblins",
        author: "jotego",
        repo: "https://github.com/jotego/jtbin",
        download_url: Some("https://github.com/jotego/jtbin/raw/master/pocket/zips/jotego.jtgng.zip"),
    },
    PocketCore {
        name: "Green Beret",
        author: "opengateware",
        repo: "https://github.com/opengateware/arcade-gberet",
        download_url: None,
    },
    PocketCore {
        name: "Kicker",
        author: "jotego",
        repo: "https://github.com/jotego/jtbin",
        download_url: Some("https://github.com/jotego/jtbin/raw/master/pocket/zips/jotego.jtkicker.zip"),
    },
    PocketCore {
        name: "Kingdom Grandprix",
        author: "psomashekar",
        repo: "https://github.com/psomashekar/pram0d-pocket-dist-public",
        download_url: Some("https://github.com/psomashekar/pram0d-pocket-dist-public/raw/develop/releases/pram0d.garegga_20221030.zip"),
    },
    PocketCore {
        name: "Konami's Ping Pong",
        author: "jotego",
        repo: "https://github.com/jotego/jtbin",
        download_url: Some("https://github.com/jotego/jtbin/raw/master/pocket/zips/jotego.jtpinpon.zip"),
    },
    PocketCore {
        name: "Lunar Lander for Analogue Pocket",
        author: "ericlewis",
        repo: "https://github.com/ericlewis/openfpga-lunarlander",
        download_url: None,
    },
    PocketCore {
        name: "Mikie",
        author: "jotego",
        repo: "https://github.com/jotego/jtbin",
        download_url: Some("https://github.com/jotego/jtbin/raw/master/pocket/zips/jotego.jtmikie.zip"),
    },
    PocketCore {
        name: "Nekketsu Kouha Kunio-kun",
        author: "jotego",
        repo: "https://github.com/jotego/jtbin",
        download_url: Some("https://github.com/jotego/jtbin/raw/master/pocket/zips/jotego.jtkunio.zip"),
    },
    PocketCore {
        name: "Neo Geo",
        author: "Mazamars312",
        repo: "https://github.com/Mazamars312/Analogue_Pocket_Neogeo",
        download_url: None,
    },
    PocketCore {
        name: "Neo Geo (Overdrive)",
        author: "Mazamars312",
        repo: "https://github.com/Mazamars312/Analogue_Pocket_Neogeo_Overdrive",
        download_url: None,
    },
    PocketCore {
        name: "NES for Analogue Pocket",
        author: "agg23",
        repo: "https://github.com/agg23/openfpga-NES",
        download_url: None,
    },
    PocketCore {
        name: "Pang!",
        author: "jotego",
        repo: "https://github.com/jotego/jtbin",
        download_url: Some("https://raw.githubusercontent.com/jotego/jtbin/master/pocket/zips/jotego.jtpang.zip"),
    },
    PocketCore {
        name: "PC Engine CD",
        author: "Mazamars312",
        repo: "https://github.com/Mazamars312/openfpga-pcengine-cd",
        download_url: None,
    },
    PocketCore {
        name: "PC Engine for Analogue Pocket",
        author: "agg23",
        repo: "https://github.com/agg23/openfpga-pcengine",
        download_url: None,
    },
    PocketCore {
        name: "PDP-1",
        author: "spacemen3",
        repo: "https://github.com/spacemen3/PDP-1",
        download_url: None,
    },
    PocketCore {
        name: "Performan",
        author: "antongale",
        repo: "https://github.com/antongale/arcade-performan",
        download_url: None,
    },
    PocketCore {
        name: "Pokemon Mini for Analogue Pocket",
        author: "agg23",
        repo: "https://github.com/agg23/openfpga-pokemonmini",
        download_url: None,
    },
    PocketCore {
        name: "Pooyan",
        author: "opengateware",
        repo: "https://github.com/opengateware/arcade-pooyan",
        download_url: None,
    },
];
