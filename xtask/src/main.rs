use std::path::PathBuf;

fn main() {
    let xtask_package = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let root_package = xtask_package.parent().unwrap().to_path_buf();

    std::env::set_current_dir(root_package.clone()).unwrap();

    let args: Vec<String> = std::env::args().collect();

    let binaries = cargo_run_bin::metadata::get_binary_packages()
        .unwrap()
        .iter()
        .map(|package| (package.package.clone(), package.clone()))
        .collect::<std::collections::HashMap<_, _>>();

    let cmd = match args.get(1) {
        None => panic!("No command provided"),
        Some(cmd) => cmd.as_str(),
    };
    let args = args[2..].to_vec();

    match cmd {
        "test" => {
            let binary_package = binaries.get("cargo-nextest").unwrap().to_owned();
            let bin_path = cargo_run_bin::binary::install(binary_package).unwrap();

            let cmd = if args.is_empty() {
                vec!["run".to_string()]
            } else {
                args
            };

            cargo_run_bin::binary::run(bin_path, cmd).unwrap();
        }
        "build-website" => {
            let binary_package = binaries.get("wasm-pack").unwrap().to_owned();
            let bin_path = cargo_run_bin::binary::install(binary_package).unwrap();

            let package_dir = root_package.join("compiler_web");
            let website_dir = root_package.join("website");

            let target_dir = website_dir.join("assets/generated/compiler_web");
            if target_dir.exists() {
                std::fs::remove_dir_all(target_dir.clone()).unwrap();
            }

            let mut command = std::process::Command::new(bin_path);
            command
                .args(&[
                    "build",
                    "--target",
                    "web",
                    "--out-dir",
                    target_dir.to_str().unwrap(),
                ])
                .current_dir(package_dir);

            println!("! {:?}", command);

            command.status().unwrap();
        }
        other => panic!("Unknown command: {}", other),
    }
}
