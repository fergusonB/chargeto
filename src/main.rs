use std::env;
use std::fs;
use std::process;

mod commands;
mod systemd;



fn main() {

    let args: Vec<String> = env::args().collect();

    let mut skip_systemd = false;
    let mut uninstall_systemd = false;
    let mut charge_level = 0;

    for arg in args {
        //systemd block
        if arg == "-no-systemd" {
            if skip_systemd == false{
                skip_systemd = true;
                println!("Skipping installation of chargeto systemd service");
            }            
        }
        if arg == "-uninstall-systemd"{
            if uninstall_systemd == false{
                uninstall_systemd = true;
                skip_systemd = true;
            }
            
        }

        //charge level block
        if arg.parse::<i32>().is_ok() {
            charge_level = arg.parse::<i32>().unwrap();
            if charge_level < 20 || charge_level > 100 {
                println!("Invalid charge level: {}. Must be between 20 and 100", charge_level);
                process::exit(1);
            }
            else{
                write_charge_control_end_threshold(charge_level);
                println!("Charge level set to {}%", charge_level);
            }
            
        }
    }

    // arg options execution
    if charge_level == 0 {
        println!("No charge level specified. Using default of 100");
        write_charge_control_end_threshold(100);
    }

    if !skip_systemd {
        systemd::create_systemd_service_file(charge_level);

        if !commands::check_if_service_enabled(){
            commands::enable_chargeto_service();
            println!("Service has been enabled");
        }
    }
    if uninstall_systemd{
        commands::disable_chargeto_service();
        systemd::uninstall_systemd_service_file();
    }
        
        

    
}

fn write_charge_control_end_threshold(charge_level: i32) {
    let _contents = fs::write(
        "/sys/class/power_supply/BAT0/charge_control_end_threshold",
        charge_level.to_string()
    );
    if _contents.is_err() {
        eprintln!("Could not write to charge_control_end_threshold, try running with elevated privileges or check to see if you have the file /sys/class/power_supply/BAT0/charge_control_end_threshold");
        process::exit(1);
    }
}