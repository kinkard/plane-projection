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
plane-projection = "0.3"
```

## Example

```rust
use plane_projection::PlaneProjection;

let proj = PlaneProjection::new(55.65);
let distance = proj.distance((55.7041417, 13.1913041), (55.6033090, 13.0019737));
assert_eq!(distance as u32, 16373);

let distance = proj.distance_to_segment((55.6781798, 13.0587896), ((55.7041417, 13.1913041), (55.6033090, 13.0019737)));
assert_eq!(distance as u32, 3615);

let heading = proj.heading((55.7041417, 13.1913041), (55.6033090, 13.0019737));
assert_eq!(heading as u32, 226);
```

## License

Dual-licensed under [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE) at your option.
