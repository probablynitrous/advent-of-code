use structopt::StructOpt;
use std::process::Command;
use std::path::PathBuf;

#[derive(Debug, StructOpt)]
struct Cli {
    day: String,
    part: String,
}

fn build_path(day: &str, part: &str) -> PathBuf {
    // Since we're reading from the build directory, we need to do some
    // footwork to get to the right directory
    let mut cwd = PathBuf::from(&std::env::current_exe().unwrap());

    // Step back three times so that we're at the root of the project
    cwd.pop();
    cwd.pop();
    cwd.pop();

    // Then add the file name so we can reference it
    cwd.push("src/days");
    cwd.push(day);
    cwd.push("parts");
    cwd.push(part);
    cwd.push("Cargo.toml");

    cwd
}

fn main() {
    let args = Cli::from_args();

    let path = build_path(&args.day, &args.part);
    println!("           .--._.--.
          ( O     O )
          /   . .   \\
         .`._______.'.
        /(           )\\
      _/  \\  \\   /  /  \\_
   .~   `  \\  \\ /  /  '   ~.
  {{    -.   \\  V  /   .-    }}
_ _`.    \\  |  |  |  /    .'_ _
>_       _}} |  |  | {{_       _<
 /. - ~ ,_-'  .^.  `-_, ~ - .\\
         '-'|/   \\|`-`
You have been greeted by the frog of destiny");
    println!("Running Day {}, Part {}", &args.day, &args.part);

    let mut process = Command::new("sh")
        .arg("-c")
        .arg(format!("cargo run -q --release --manifest-path {:?}", path))
        .spawn()
        .expect("failed to execute process");

   let ecode = process.wait().expect("failed to wait on child");

   assert!(ecode.success());
}
