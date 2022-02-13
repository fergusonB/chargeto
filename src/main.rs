use std::env;
use std::fs;
use std::io::Write;
use std::process;
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
        create_systemd_service_file(&charge_level_int);
        println!("Charging to {} percent", charge_level);
    }
}

// create systemd service file
fn create_systemd_service_file(charge_level: &i32) {
    let mut file = fs::File::create("/etc/systemd/system/chargeto.service").unwrap();
    let contents = format!(
        "[Unit]
        Description=Set battery charge threshold
        After=multi-user.target
        StartLimitBurst =0

        [Service]
        Type=oneshot
        Restart=on-failure
        ExecStart=/bin/bash -c 'echo {} > /sys/class/power_supply/BAT0/charge_control_end_threshold'

        [Install]
        WantedBy=multi-user.target",charge_level);

    file.write_all(contents.as_bytes()).unwrap();
}

//#todo - run commands - sudo systemctl enable chargeto.service -- print to stdout
//#todo allow user to not implement service or delete service file
//#todo check for non bat0 battery