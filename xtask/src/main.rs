fn main() {
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
        other => panic!("Unknown command: {}", other),
    }
}
