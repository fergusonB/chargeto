use std::process;

pub fn enable_chargeto_service(){
    //run command 'sudo systemctl enable chargeto.service'
    let _contents = process::Command::new("sudo")
        .arg("systemctl")
        .arg("enable")
        .arg("chargeto.service")
        .output()
        .expect("failed to execute process");
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
