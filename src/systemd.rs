use std::fs;
use std::io::Write;

pub fn create_systemd_service_file(charge_level: &i32) {
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
