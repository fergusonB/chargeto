use std::env;
use std::fs;
use std::process;
fn main() {
    let args: Vec<String> = env::args().collect();


    if args.len() != 2 {
        eprintln!("Usage: chargeto 80");
        process::exit(1);
    }

    let charge_level = &args[1];

    let charge_level_int = charge_level.parse::<i32>().unwrap_or(0);
    if charge_level_int < 20 || charge_level_int > 100 {
        eprintln!("Charge level must be an integer between 20 and 100");
        process::exit(1);
    } else {
        let _contents = fs::write(
            "/sys/class/power_supply/BAT0/charge_control_end_threshold",
            charge_level,
        );
        if _contents.is_err() {
            eprintln!("Could not write to charge_control_end_threshold, try running with elevated privileges or check to see if you have the file /sys/class/power_supply/BAT0/charge_control_end_threshold");
            process::exit(1);
        }
        println!("Charging to {} percent", charge_level);
    }
}
