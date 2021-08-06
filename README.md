# simple_wet_bulb
Port of [SimpleWetBulb](https://github.com/Dr-Kristau/SimpleWetBulb#readme) to Rust

The color-coding for extreme risk is different:
- blue text: extreme risk (>90ºF, >32.22ºC)

To build an executable move to the simple_wet_bulb folder and run:
```bash
cargo build --release
```

It should now be available:
```bash
simple_wet_bulb -t 30 -h 30 -i ºC
```
