# chargeto
## About
chargeto is a *simple* command line tool for controling the max charge level of a battery in debian-based distributions.
It will stop a laptop charging beyond the limit it is given, and the changes persist through reboot with a systemd service.

## Usage
`sudo chargeto [battery percentage]` e.g. `sudo chargeto 90`
## Installation
Download the binary from the releases and run with elevated permissions, or compile it yourself.
### Compiling
Requires [Rust](https://rustup.rs/).
* Clone repo
* `cd` into the cloned repo
* `cargo build --release`


