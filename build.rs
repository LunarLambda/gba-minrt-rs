/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

fn var(s: &str) -> String { std::env::var(s).unwrap() }

fn cwd() -> std::path::PathBuf { std::env::current_dir().unwrap() }

fn assemble_and_link<P: AsRef<std::path::Path>>(s: P) {
    let s = s.as_ref();

    cc::Build::new()
        // CC doesn't know the thumbv4t-none-eabi target.
        .compiler("arm-none-eabi-gcc")
        .archiver("arm-none-eabi-ar")
        .no_default_flags(true)
        .include("gba-minrt/rt")
        .flag("-xassembler")
        .flag("-mcpu=arm7tdmi")
        .flag("-mthumb-interwork")
        .file(&s)
        .compile("minrt");

    println!("cargo:rerun-if-changed={}", s.display());
}

const MINRT_PATH: &str = "gba-minrt/rt";

fn main() {

    let crt0 = format!("{}/crt0.s", MINRT_PATH);
    let ld = format!("{}/rom.ld", MINRT_PATH);
    let out = var("OUT_DIR");

    assemble_and_link(crt0);

    std::fs::copy(
        &ld,
        format!("{}/link.x", &out)).unwrap();

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed={}", &ld);
    println!("cargo:rustc-link-search={}", &out);
    println!("cargo:rustc-link-search={}/{}", cwd().display(), MINRT_PATH);
}
