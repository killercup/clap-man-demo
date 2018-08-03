#[macro_use]
extern crate clap;
extern crate clap_generate;

use std::{env, fs, path::PathBuf};

#[path="src/cli.rs"]
mod cli;

fn main() {
    let outdir = env::current_dir().expect("where am i").join("etc");
    fs::create_dir_all(&outdir).unwrap();

    use clap::IntoApp;
    let app = cli::Head::into_app();

    use clap_generate::gen_manuals;
    for man in gen_manuals(&app) {
        let name = "clap-man-demo.1"; // FIXME: Extract this from man!
        let path = PathBuf::from(&outdir).join(name);
        let mut out = fs::File::create(&path).unwrap();
        use std::io::Write;
        out.write_all(man.render().as_bytes()).unwrap();
        eprintln!("Wrote man page to {:?}", path);
    }

    // FIXME: Shell is private!
    // use clap_generate::{gen_completions, Shell};
    // gen_completions(&mut app, "clap-man-demo", Shell::Bash, outdir.clone());
    // gen_completions(&mut app, "clap-man-demo", Shell::Fish, outdir.clone());
}
