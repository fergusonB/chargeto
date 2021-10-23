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
        edit_crontab(charge_level_int);
        println!("Charging to {} percent", charge_level);
    }
}

fn edit_crontab( charge_level_int: i32) {
    let _contents = fs::read_to_string("/etc/crontab");
    if _contents.is_err() {
        eprintln!("Could not read /etc/crontab, try running with elevated privileges or check to see if you have the file /etc/crontab");
        process::exit(1);
    }
    let contents = _contents.unwrap();
    let mut lines = contents.lines();
    let mut new_crontab = String::new();
    let mut found_line = false;
    while let Some(line) = lines.next() {
        if line.contains("/sys/class/power_supply/BAT0/charge_control_end_threshold") {
            found_line = true;
            // update the line with the new charge level @reboot echo {charge_level_int} > /sys/class/power_supply/BAT0/charge_control_end_threshold
            new_crontab.push_str(&format!("@reboot echo {charge_level_int} > /sys/class/power_supply/BAT0/charge_control_end_threshold\n", charge_level_int = charge_level_int));
        } else {
            new_crontab.push_str(line);
            new_crontab.push_str("\n");
        }
    }
    if !found_line {
        new_crontab.push_str("@reboot echo ");
        new_crontab.push_str(&charge_level_int.to_string());
        new_crontab.push_str(" > /sys/class/power_supply/BAT0/charge_control_end_threshold\n");
    }
    let _contents = fs::write("/etc/crontab", new_crontab);
    if _contents.is_err() {
        eprintln!("Could not write to /etc/crontab, try running with elevated privileges or check to see if you have the file /etc/crontab");
        process::exit(1);
    }


}