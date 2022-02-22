use std::process;
use std::fs;

pub fn enable_chargeto_service(){
    //run command 'sudo systemctl enable chargeto.service'
    let _contents = process::Command::new("sudo")
        .arg("systemctl")
        .arg("enable")
        .arg("chargeto.service")
        .output()
        .expect("failed to execute process");
    if _contents.status.success() {
        println!("chargeto systemd service has been enabled.");
    }
}

pub fn check_if_service_enabled()->bool{
    //run command 'systemctl is-enabled chargeto.service'
    let contents = process::Command::new("systemctl")
        .arg("is-enabled")
        .arg("chargeto.service")
        .output()
        .expect("failed to execute process");
       contents.status.success()
    
}

pub fn disable_chargeto_service(){
    //run command 'sudo systemctl disable chargeto.service'
    if check_if_service_enabled(){
        let _contents = process::Command::new("sudo")
        .arg("systemctl")
        .arg("disable")
        .arg("chargeto.service")
        .output()
        .expect("failed to execute process");
        if _contents.status.success(){
            println!("Service has been disabled");
        }
        else {
            println!("Service could not be disabled");
        }
    }

}


pub fn write_charge_control_end_threshold(charge_level: i32) {
    let _contents = fs::write(
        "/sys/class/power_supply/BAT0/charge_control_end_threshold",
        charge_level.to_string()
    );
    if _contents.is_ok() {
        println!("Charge control end threshold has been set to {}%", charge_level);
    }
    else if _contents.is_err() {
        eprintln!("Could not write to charge_control_end_threshold, try running with elevated privileges or check to see if you have the file /sys/class/power_supply/BAT0/charge_control_end_threshold");
        process::exit(1);
    }
}

pub fn read_current_charge_limit()->i32{
    let contents = fs::read_to_string("/sys/class/power_supply/BAT0/charge_control_end_threshold").unwrap();
    let current_charge_limit = contents.trim().parse::<i32>().unwrap();
    current_charge_limit
}