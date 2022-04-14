# chargeto
## About
chargeto is a command line tool written in rust for controling the max charge level of a battery in debian-based distributions.
It will stop a laptop charging beyond the limit it is given, and the changes persist through reboot with a systemd service.
Careful use of this program will allow you to increase the lifespan of your lithium-ion battery.

## Requirements
- A linux based operating system that utilized systemd
- Kernel version 5.4 or greater

## Usage
Synopsis: 
> `sudo chargeto 60`

Optional Arguments:
* `-s, --show `: Show the current program settings
* `-no-systemd`: Skip the systemd service creation
* `-uninstall-systemd`: Uninstall the systemd service



### Binary
* Download binary from [releases](https://github.com/fergusonB/chargeto/releases)
* Allow executing file as a program 
    `chmod +x chargeto`
* Run 
    `sudo ./chargeto <number between 20 and 100>`

### Compiling
Requires [Rust](https://rustup.rs/).
* `git clone https://github.com/fergusonB/chargeto.git`
* `cd chargeto`
* `cargo build --release`
* `chmod +x ./target/release/chargeto`
* `sudo cp ./target/release/chargeto /usr/local/bin/chargeto`
* `sudo chargeto 80`

