use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=resources/com.radioactivehamster.pocket_up.gschema.xml");

    let home = match env::var("HOME") {
        Ok(home) => home,
        Err(error) => return eprintln!("Could not read $HOME environment variable: {error}"),
    };
    let home_path = Path::new(&home);
    let local_schemas_path = home_path.join(".local/share/glib-2.0/schemas");
    let schema_filename = "com.radioactivehamster.pocket_up.gschema.xml";
    let resources_schema_path = Path::new("./resources").join(schema_filename);
    let local_schema_path = local_schemas_path.join(schema_filename);

    match fs::create_dir_all(&local_schemas_path) {
        Ok(_) => println!("Directory creation successful."),
        Err(error) => return eprintln!("Could not create schemas directory: {error}"),
    };

    match fs::copy(resources_schema_path, local_schema_path) {
        Ok(_) => println!("Successfully copied schema."),
        Err(error) => return eprintln!("Could not copy schema: {error}"),
    };

    match Command::new("glib-compile-schemas")
        .arg(local_schemas_path.to_str().unwrap())
        .spawn()
    {
        Ok(_) => println!("Successfully compiled schema."),
        Err(error) => eprintln!("Error while running `glib-compile-schemas`: {error}"),
    };
}
