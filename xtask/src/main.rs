use std::process::Command;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let task = args.get(1).map(|s| s.as_str()).unwrap_or("help");
    let release = args.iter().any(|a| a == "--release");

    match task {
        "build" => {
            if release {
                run_cargo(&["build", "--release"]);
            } else {
                run_cargo(&["build"]);

            }
        }

        "install" => {
            run_cargo(&["install", "--path", "./eck"]);
        }

        "test" => {
            run_cargo(&["test", "--test=eck_tests"]);
        }

        "fuzz" => {
            let args: Vec<String> = std::env::args().skip(2).collect();

            if args.is_empty() || args.iter().any(|a| a == "--help" || a == "-h") {
                run_cargo_in("eck", &["fuzz", "list"]);
            } else {
                let mut cmd: Vec<&str> = vec!["fuzz", "run", args[0].as_str()];
                if args.len() > 1 {
                    cmd.push("--");
                    cmd.extend(args[1..].iter().map(String::as_str));
                }
                run_cargo_in("eck", &cmd);
            }
        }

        "fmt" => {
            run_cargo(&["fmt"]);
        }

        "help" | "--help" | "-h" => {
            print_help();
        }

        "doc" | "rustdoc" | "rsdoc" => {
            run_cargo(&["doc", "--open" , "--document-private-items"]);
        } 
        _ => {
            eprintln!("Unknown task: {task}");
            print_help();
            std::process::exit(1);
        }
    }
}

fn print_help() {
    eprintln!(
        "\
cargo xtask — dev helpers for Enkryptit!

USAGE:
  cargo xtask <COMMAND> [--release]

BUILD COMMANDS:
  build                  Build eck (Enkryptit!'s binary)
  install                Install eck

TEST:
  test                   Run workspace tests
  fuzz                   List fuzz targets (libFuzzer, via cargo-fuzz)
  fuzz <TARGET> [FLAGS]  Run a fuzz target; extra flags go to libFuzzer
                         (e.g. -runs=1000, -max_total_time=60)             
                                                                
OTHER:
  fmt                    Format code                                                           
                                                        "
    );
}

pub fn run_cargo(args: &[&str]) {
    run_cargo_in(".", args);
}

pub fn run_cargo_in(dir: &str, args: &[&str]) {
    let status = Command::new("cargo")
        .args(args)
        .current_dir(dir)
        .status()
        .expect("Unable to run cargo");
    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }
}

