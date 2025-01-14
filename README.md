# About

A plane projection, useful for blazingly fast approximate distance calculations.
Based on WGS84 ellipsoid model of the Earth, plane projection provides 0.1% precision
on distances under 500km at latitudes up to the 65°.
See the [article about Cheap Ruler](https://blog.mapbox.com/fast-geodesic-approximations-with-cheap-ruler-106f229ad016)
for more details about the principle and formulas behind.

Comparing to another Rust crates that provide the same functionality,
[cheap-ruler-rs](https://github.com/vipera/cheap-ruler-rs) and [flat-projection](https://github.com/Turbo87/flat-projection-rs),
this crate has zero dependencies and minimalistic API.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
plane-projection = "0.1"
```

## Example

```rust
use plane_projection::PlaneProjection;

let proj = PlaneProjection::new(55.65);
let distance = proj.distance(&(55.704141722528554, 13.191304107330561), &(55.60330902847681, 13.001973666557435));
assert_eq!(distance as u32, 16373);
```

## License

All code in this project is dual-licensed under either:

- [MIT license](https://opensource.org/licenses/MIT) ([`LICENSE-MIT`](LICENSE-MIT))
- [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0) ([`LICENSE-APACHE`](LICENSE-APACHE))

at your option.
