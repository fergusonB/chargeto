# chargeto
## About
chargeto is a *simple* command line tool for controling the max charge level of a battery in debian-based distributions.
It will stop a laptop charging beyond the limit it is given, and the changes persist through reboot with a systemd service.

## Usage
Download the binary from the releases and run with elevated permissions, or compile it yourself.

### Binary
* Download binary from [releases](https://github.com/fergusonB/chargeto/releases)
* Allow executing file as a program 
    >`chmod +x chargeto`
* Run 
    >`sudo ./chargeto <number between 20 and 100>`

### Compiling
Requires [Rust](https://rustup.rs/).
* `git clone https://github.com/fergusonB/chargeto.git`
* `cd chargeto`
* `cargo build --release`
* `chmod +x ./target/release/chargeto`
* `sudo cp ./target/release/chargeto /usr/local/bin/chargeto`
* `sudo chargeto 80`

