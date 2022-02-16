use std::env;
use std::fs;
use std::process;

mod commands;
mod systemd;

fn main() {
    // Get and parse args
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
        //write to file
        let _contents = fs::write(
            "/sys/class/power_supply/BAT0/charge_control_end_threshold",
            charge_level,
        );
        if _contents.is_err() {
            eprintln!("Could not write to charge_control_end_threshold, try running with elevated privileges or check to see if you have the file /sys/class/power_supply/BAT0/charge_control_end_threshold");
            process::exit(1);
        }
        
        systemd::create_systemd_service_file(&charge_level_int);

        if commands::check_if_service_enabled(){
            println!("Service is already enabled");
        }
        else{
            commands::enable_chargeto_service();
            println!("Service has been enabled");
        }

        println!("Charging to {} percent", charge_level);
    }
}

