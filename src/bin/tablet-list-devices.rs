fn main() {
    for (p, device) in evdev::enumerate() {
        println!("{:?} {:?}", p, device.name());
    }
}
