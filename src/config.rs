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

    // The BIOS files for the core if applicable
    pub bios_files: &'static [&'static PocketCoreBios],

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

pub struct PocketCoreBios {
    // The destination filepath for the BIOS file
    pub path: &'static str,

    // The URL to download the BIOS file
    pub url: &'static str,

    // If the BIOS is zipped this is the file to extract
    pub path_in_zip: Option<&'static str>,
}

pub const POCKET_CORES: [PocketCore; 53] = [
    PocketCore {
        name: "Amiga 500",
        author: "Mazamars312",
        repo: "https://github.com/Mazamars312/Analogue-Amiga",
        bios_files: &[],
        download_url: None,
    },
    PocketCore {
        name: "Arduboy for Analogue Pocket",
        author: "agg23",
        repo: "https://github.com/agg23/openfpga-arduboy",
        bios_files: &[],
        download_url: None,
    },
    PocketCore {
        name: "Asteroids for Analogue Pocket",
        author: "ericlewis",
        repo: "https://github.com/ericlewis/openfpga-asteroids",
        bios_files: &[],
        download_url: None,
    },
    PocketCore {
        name: "Atari's 1972 Pong",
        author: "agg23",
        repo: "https://github.com/agg23/openfpga-pong",
        bios_files: &[],
        download_url: None,
    },
    PocketCore {
        name: "Bank Panic",
        author: "opengateware",
        repo: "https://github.com/opengateware/arcade-bankp",
        bios_files: &[],
        download_url: None,
    },
    // TODO: psomashekar's releases are all under one singular repo as dated files.
    // Need to figure out how to handle that better than hard coding them.
    PocketCore {
        name: "Batrider",
        author: "psomashekar",
        repo: "https://github.com/psomashekar/pram0d-pocket-dist-public",
        bios_files: &[],
        download_url: Some("https://github.com/psomashekar/pram0d-pocket-dist-public/raw/develop/releases/pram0d.batrider_20221218.zip"),
    },
    PocketCore {
        name: "Battle Bakraid",
        author: "psomashekar",
        repo: "https://github.com/psomashekar/pram0d-pocket-dist-public",
        bios_files: &[],
        download_url: Some("https://github.com/psomashekar/pram0d-pocket-dist-public/raw/develop/releases/pram0d.bakraid_20221218.zip"),
    },
    PocketCore {
        name: "Battle Garegga",
        author: "psomashekar",
        repo: "https://github.com/psomashekar/pram0d-pocket-dist-public",
        bios_files: &[],
        download_url: Some("https://github.com/psomashekar/pram0d-pocket-dist-public/raw/develop/releases/pram0d.garegga_20221030.zip"),
    },
    PocketCore {
        name: "Congo Bongo",
        author: "opengateware",
        repo: "https://github.com/opengateware/arcade-congo",
        bios_files: &[],
        download_url: None,
    },
    PocketCore {
        name: "Contra",
        author: "jotego",
        repo: "https://github.com/jotego/jtbin",
        bios_files: &[],
        download_url: Some("https://github.com/jotego/jtbin/raw/master/pocket/zips/jotego.jtcontra.zip"),
    },
    PocketCore {
        name: "Dig Dug",
        author: "opengateware",
        repo: "https://github.com/opengateware/arcade-digdug",
        bios_files: &[],
        download_url: None,
    },
    PocketCore {
        name: "Dominos for Analogue Pocket",
        author: "ericlewis",
        repo: "https://github.com/ericlewis/openfpga-dominos",
        bios_files: &[],
        download_url: None,
    },
    // TODO: This is a pre-release so it doesn't show up using the GitHub latest release API. Work around this!
    PocketCore {
        name: "Donkey Kong",
        author: "ericlewis",
        repo: "https://github.com/ericlewis/openFPGA-DonkeyKong",
        bios_files: &[],
        download_url: None,
    },
    PocketCore {
        name: "Double Dragon",
        author: "jotego",
        repo: "https://github.com/jotego/jtbin",
        bios_files: &[],
        download_url: Some("https://github.com/jotego/jtbin/raw/master/pocket/zips/jotego.jtdd.zip"),
    },
    PocketCore {
        name: "Double Dragon II",
        author: "jotego",
        repo: "https://github.com/jotego/jtbin",
        bios_files: &[],
        download_url: Some("https://github.com/jotego/jtbin/raw/master/pocket/zips/jotego.jtdd2.zip"),
    },
    PocketCore {
        name: "Galaga",
        author: "opengateware",
        repo: "https://github.com/opengateware/arcade-galaga",
        bios_files: &[],
        download_url: None,
    },
    PocketCore {
        name: "Genesis for Analogue Pocket",
        author: "opengateware",
        repo: "https://github.com/opengateware/openfpga-genesis",
        bios_files: &[],
        download_url: None,
    },
    PocketCore {
        name: "Ghosts 'n Goblins",
        author: "jotego",
        repo: "https://github.com/jotego/jtbin",
        bios_files: &[],
        download_url: Some("https://github.com/jotego/jtbin/raw/master/pocket/zips/jotego.jtgng.zip"),
    },
    PocketCore {
        name: "Green Beret",
        author: "opengateware",
        repo: "https://github.com/opengateware/arcade-gberet",
        bios_files: &[],
        download_url: None,
    },
    PocketCore {
        name: "Kicker",
        author: "jotego",
        repo: "https://github.com/jotego/jtbin",
        bios_files: &[],
        download_url: Some("https://github.com/jotego/jtbin/raw/master/pocket/zips/jotego.jtkicker.zip"),
    },
    PocketCore {
        name: "Kingdom Grandprix",
        author: "psomashekar",
        repo: "https://github.com/psomashekar/pram0d-pocket-dist-public",
        bios_files: &[],
        download_url: Some("https://github.com/psomashekar/pram0d-pocket-dist-public/raw/develop/releases/pram0d.garegga_20221030.zip"),
    },
    PocketCore {
        name: "Konami's Ping Pong",
        author: "jotego",
        repo: "https://github.com/jotego/jtbin",
        bios_files: &[],
        download_url: Some("https://github.com/jotego/jtbin/raw/master/pocket/zips/jotego.jtpinpon.zip"),
    },
    PocketCore {
        name: "Lunar Lander for Analogue Pocket",
        author: "ericlewis",
        repo: "https://github.com/ericlewis/openfpga-lunarlander",
        bios_files: &[],
        download_url: None,
    },
    PocketCore {
        name: "Mikie",
        author: "jotego",
        repo: "https://github.com/jotego/jtbin",
        bios_files: &[],
        download_url: Some("https://github.com/jotego/jtbin/raw/master/pocket/zips/jotego.jtmikie.zip"),
    },
    PocketCore {
        name: "Nekketsu Kouha Kunio-kun",
        author: "jotego",
        repo: "https://github.com/jotego/jtbin",
        bios_files: &[],
        download_url: Some("https://github.com/jotego/jtbin/raw/master/pocket/zips/jotego.jtkunio.zip"),
    },
    PocketCore {
        name: "Neo Geo",
        author: "Mazamars312",
        repo: "https://github.com/Mazamars312/Analogue_Pocket_Neogeo",
        bios_files: &[
            &PocketCoreBios {
                path: "Assets/ng/common/uni-bios_4_0.rom",
                url: "http://unibios.free.fr/download/uni-bios-40.zip",
                path_in_zip: Some("uni-bios.rom"),
            },
            &PocketCoreBios {
                path: "Assets/ng/common/000-lo.lo",
                url: "https://archive.org/download/mister-console-bios-pack_theypsilon/NeoGeo.zip/000-lo.lo",
                path_in_zip: None,
            },
            &PocketCoreBios {
                path: "Assets/ng/common/sfix.sfix",
                url: "https://archive.org/download/mister-console-bios-pack_theypsilon/NeoGeo.zip/sfix.sfix",
                path_in_zip: None,
            },
        ],
        download_url: None,
    },
    PocketCore {
        name: "Neo Geo (Overdrive)",
        author: "Mazamars312",
        repo: "https://github.com/Mazamars312/Analogue_Pocket_Neogeo_Overdrive",
        bios_files: &[
            &PocketCoreBios {
                path: "Assets/ng/common/uni-bios_4_0.rom",
                url: "http://unibios.free.fr/download/uni-bios-40.zip",
                path_in_zip: Some("uni-bios.rom"),
            },
            &PocketCoreBios {
                path: "Assets/ng/common/000-lo.lo",
                url: "https://archive.org/download/mister-console-bios-pack_theypsilon/NeoGeo.zip/000-lo.lo",
                path_in_zip: None,
            },
            &PocketCoreBios {
                path: "Assets/ng/common/sfix.sfix",
                url: "https://archive.org/download/mister-console-bios-pack_theypsilon/NeoGeo.zip/sfix.sfix",
                path_in_zip: None,
            },
        ],
        download_url: None,
    },
    PocketCore {
        name: "NES for Analogue Pocket",
        author: "agg23",
        repo: "https://github.com/agg23/openfpga-NES",
        bios_files: &[],
        download_url: None,
    },
    PocketCore {
        name: "Pang!",
        author: "jotego",
        repo: "https://github.com/jotego/jtbin",
        bios_files: &[],
        download_url: Some("https://raw.githubusercontent.com/jotego/jtbin/master/pocket/zips/jotego.jtpang.zip"),
    },
    PocketCore {
        name: "PC Engine CD",
        author: "Mazamars312",
        repo: "https://github.com/Mazamars312/openfpga-pcengine-cd",
        bios_files: &[],
        download_url: None,
    },
    PocketCore {
        name: "PC Engine for Analogue Pocket",
        author: "agg23",
        repo: "https://github.com/agg23/openfpga-pcengine",
        bios_files: &[],
        download_url: None,
    },
    PocketCore {
        name: "PDP-1",
        author: "spacemen3",
        repo: "https://github.com/spacemen3/PDP-1",
        bios_files: &[],
        download_url: None,
    },
    PocketCore {
        name: "Performan",
        author: "antongale",
        repo: "https://github.com/antongale/arcade-performan",
        bios_files: &[],
        download_url: None,
    },
    PocketCore {
        name: "Pokemon Mini for Analogue Pocket",
        author: "agg23",
        repo: "https://github.com/agg23/openfpga-pokemonmini",
        bios_files: &[],
        download_url: None,
    },
    PocketCore {
        name: "Pooyan",
        author: "opengateware",
        repo: "https://github.com/opengateware/arcade-pooyan",
        bios_files: &[],
        download_url: None,
    },
    PocketCore {
        name: "Q*Bert",
        author: "ericlewis",
        repo: "https://github.com/ericlewis/openfpga-qbert",
        bios_files: &[],
        download_url: None,
    },
    PocketCore {
        name: "Radar Scope",
        author: "ericlewis",
        repo: "https://github.com/ericlewis/openFPGA-RadarScope",
        bios_files: &[],
        download_url: None,
    },
    PocketCore {
        name: "Road Fighter",
        author: "jotego",
        repo: "https://github.com/jotego/jtbin",
        bios_files: &[],
        download_url: Some("https://raw.githubusercontent.com/jotego/jtbin/master/pocket/zips/jotego.jtroadf.zip"),
    },
    PocketCore {
        name: "Roc'n Rope",
        author: "jotego",
        repo: "https://github.com/jotego/jtbin",
        bios_files: &[],
        download_url: Some("https://raw.githubusercontent.com/jotego/jtbin/master/pocket/zips/jotego.jtroc.zip"),
    },
    PocketCore {
        name: "Slap Fight",
        author: "antongale",
        repo: "https://github.com/antongale/arcade-slapfight",
        bios_files: &[],
        download_url: None,
    },
    PocketCore {
        name: "SNES for Analogue Pocket",
        author: "agg23",
        repo: "https://github.com/agg23/openfpga-SNES",
        bios_files: &[],
        download_url: None,
    },
    PocketCore {
        name: "Snow Bros. 2",
        author: "psomashekar",
        repo: "https://github.com/psomashekar/pram0d-pocket-dist-public",
        bios_files: &[],
        download_url: Some("https://raw.githubusercontent.com/psomashekar/pram0d-pocket-dist-public/develop/releases/pram0d.snowbros2_20221019.zip"),
    },
    PocketCore {
        name: "Space Race for Analogue Pocket",
        author: "ericlewis",
        repo: "https://github.com/ericlewis/openfpga-spacerace",
        bios_files: &[],
        download_url: None,
    },
    PocketCore {
        name: "Spiritualized Adventure Vision",
        author: "spiritualized1997",
        repo: "https://github.com/spiritualized1997/openFPGA-Adventure-Vision",
        bios_files: &[],
        download_url: None,
    },
    PocketCore {
        name: "Spiritualized Arcadia",
        author: "spiritualized1997",
        repo: "https://github.com/spiritualized1997/openFPGA-Arcadia",
        bios_files: &[],
        download_url: None,
    },
    PocketCore {
        name: "Spiritualized Atari 2600",
        author: "spiritualized1997",
        repo: "https://github.com/spiritualized1997/openFPGA-2600",
        bios_files: &[],
        download_url: None,
    },
    PocketCore {
        name: "Spiritualized Atari 7800",
        author: "spiritualized1997",
        repo: "https://github.com/spiritualized1997/openFPGA-7800",
        bios_files: &[],
        download_url: None,
    },
    PocketCore {
        name: "Spiritualized Channel F",
        author: "spiritualized1997",
        repo: "https://github.com/spiritualized1997/openFPGA-Channel-F",
        bios_files: &[],
        download_url: None,
    },
    PocketCore {
        name: "Spiritualized Coleco",
        author: "spiritualized1997",
        repo: "https://github.com/spiritualized1997/openFPGA-Coleco",
        bios_files: &[],
        download_url: None,
    },
    PocketCore {
        name: "Spiritualized Creativision",
        author: "spiritualized1997",
        repo: "https://github.com/spiritualized1997/openFPGA-Creativision",
        bios_files: &[],
        download_url: None,
    },
    PocketCore {
        name: "Spiritualized Gamate",
        author: "spiritualized1997",
        repo: "https://github.com/spiritualized1997/openFPGA-Gamate",
        bios_files: &[],
        download_url: None,
    },
    PocketCore {
        name: "Spiritualized Game King",
        author: "spiritualized1997",
        repo: "https://github.com/spiritualized1997/openFPGA-Game-King",
        bios_files: &[],
        download_url: None,
    },
    PocketCore {
        name: "Spiritualized GB",
        author: "spiritualized1997",
        repo: "https://github.com/spiritualized1997/openFPGA-GB-GBC",
        bios_files: &[],
        download_url: None,
    },
];
