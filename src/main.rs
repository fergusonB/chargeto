use std::env;
use std::process;

mod commands;
mod info;
mod systemd;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        info::print_info();
    }

    // options
    let mut skip_systemd = false;
    let mut uninstall_systemd = false;
    let mut charge_level = commands::read_current_charge_limit();
    let charge_level_initial = commands::read_current_charge_limit();

    // parse arguments
    for arg in args {
        // show current settings
        if arg == "-s" || arg == "--show" {
           println!("Current settings:");
              println!("\tCharge control end threshold: {}%", charge_level);
                println!("\tSystemd service enabled: {}", commands::check_if_service_enabled());
            process::exit(0);
        }
        //systemd block
        if arg == "-no-systemd" {
            skip_systemd = true;
        }
        if arg == "-uninstall-systemd" {
            if uninstall_systemd == false {
                uninstall_systemd = true;
                skip_systemd = true;
            }
        }
        //charge level block
        if arg.parse::<i32>().is_ok() {
            charge_level = arg.parse::<i32>().unwrap();
            if charge_level < 20 || charge_level > 100 {
                println!(
                    "Invalid charge level: {}. Must be between 20 and 100",
                    charge_level
                );
                info::print_info();
            } else {
                commands::write_charge_control_end_threshold(charge_level);
            }
        }
    }

    args_options_execution(charge_level, skip_systemd, uninstall_systemd, charge_level_initial);
}

fn args_options_execution(charge_level: i32, skip_systemd: bool, uninstall_systemd: bool, charge_level_initial: i32) {
    if !skip_systemd {
        systemd::create_systemd_service_file(charge_level);

        if !commands::check_if_service_enabled() {
            commands::enable_chargeto_service();
        }
    }
    else if skip_systemd{
        if charge_level == charge_level_initial {
            println!("Charge control end threshold is already set to {}%, and there is no reason to skip systemd service creation.", charge_level);
            info::print_info();
        }
        else {
            println!("Skipping systemd service creation");
        }
    }
    if uninstall_systemd {
        if commands::check_if_service_enabled() {
            commands::disable_chargeto_service();
            systemd::uninstall_systemd_service_file();
        }
    }
}
