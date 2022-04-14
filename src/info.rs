use std::process;

pub fn print_info(){
    let string ="Name: chargeto\n \
    Synopsis: sudo chargeto 60\n \
    Description:\n \
        chargeto is a tool to control the maximum charge level of a laptop battery on linux devices.\n \
        It requires linux kernel 5.4+ and systemd.\n \
        Optional arguments: \n \
            -s, --show\n \
                Show current settings.\n \
            -no-systemd : Skip installation of chargeto systemd service\n \
            -uninstall-systemd : Uninstall chargeto systemd service";
    println!("{}", string);


    
    process::exit(1);
}
