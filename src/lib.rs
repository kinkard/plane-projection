// Values that define WGS84 ellipsoid model of the Earth
const EQUATORIAL_RADIUS: f64 = 6378137.0;
const FLATTENING: f64 = 1.0 / 298.257223563;
const SQUARED_ECCENTRICITY: f64 = FLATTENING * (2.0 - FLATTENING);

/// A coordinate in (latitude, longitude) format
pub type LatLon = (f64, f64);

/// A plane projection, useful for blazingly fast approximate distance calculations.
/// Based on WGS84 ellipsoid model of the Earth, plane projection provides 0.1% precision
/// on distances under 500km at latitudes up to the 65°.
/// See https://blog.mapbox.com/fast-geodesic-approximations-with-cheap-ruler-106f229ad016
/// for more details about the principle and formulas behind.
///
/// ```
/// use plane_projection::PlaneProjection;
///
/// let proj = PlaneProjection::new(55.65);
/// let distance = proj.distance((55.704141722528554, 13.191304107330561), (55.60330902847681, 13.001973666557435));
/// assert_eq!(distance as u32, 16373);
/// ```
pub struct PlaneProjection {
    lon_scale: f64,
    lat_scale: f64,
}

impl PlaneProjection {
    /// Creates a plane projection to the Earth at provided latitude
    pub fn new(latitude: f64) -> Self {
        // Based on https://en.wikipedia.org/wiki/Earth_radius#Meridional
        let cos_lat = latitude.to_radians().cos();
        let w2 = 1.0 / (1.0 - SQUARED_ECCENTRICITY * (1.0 - cos_lat * cos_lat));
        let w = w2.sqrt();

        // multipliers for converting longitude and latitude degrees into distance
        let lon_scale = (EQUATORIAL_RADIUS * w * cos_lat).to_radians(); // based on normal radius of curvature
        let lat_scale = (EQUATORIAL_RADIUS * w * w2 * (1.0 - SQUARED_ECCENTRICITY)).to_radians(); // based on meridonal radius of curvature

        Self {
            lon_scale,
            lat_scale,
        }
    }

    /// Sqare distance in meters between two points in (lat, lon) format
    #[inline(always)]
    pub fn square_distance(&self, a: LatLon, b: LatLon) -> f64 {
        let lat_dist = (a.0 - b.0) * self.lat_scale;
        let lon_dist = lon_diff(a.1, b.1) * self.lon_scale;
        lat_dist * lat_dist + lon_dist * lon_dist
    }

    /// Distance in meters between two points in (lat, lon) format
    #[inline(always)]
    pub fn distance(&self, a: LatLon, b: LatLon) -> f64 {
        self.square_distance(a, b).sqrt()
    }
}

/// Returns the difference between two longitudes in range [-180.0, 180.0] degrees
#[inline(always)]
fn lon_diff(a: f64, b: f64) -> f64 {
    let mut lon_diff = a - b;
    if lon_diff > 180.0 {
        lon_diff -= 360.0;
    } else if lon_diff < -180.0 {
        lon_diff += 360.0;
    }
    lon_diff
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lon_diff_test() {
        assert_eq!(lon_diff(0.0, 0.0), 0.0);
        assert_eq!(lon_diff(100.0, 0.0), 100.0);
        assert_eq!(lon_diff(100.0, -100.0), -160.0);
        assert_eq!(lon_diff(177.0, -177.0), -6.0);
        assert_eq!(lon_diff(358.0, 0.0), -2.0);
        assert_eq!(lon_diff(0.0, 358.0), 2.0);
        assert_eq!(lon_diff(0.0, -180.0), 180.0);
        assert_eq!(lon_diff(1.0, -180.0), -179.0);
        assert_eq!(lon_diff(180.0, 0.0), 180.0);
        assert_eq!(lon_diff(180.0, -1.0), -179.0);
        assert_eq!(lon_diff(180.0, -180.0), 0.0);
    }

    #[test]
    fn distance_test() {
        // From Lund C to Malmo C
        let proj = PlaneProjection::new(55.65);
        assert_eq!(
            proj.distance(
                (55.704141722528554, 13.191304107330561),
                (55.60330902847681, 13.001973666557435)
            ) as u32,
            16373
        );

        let proj = PlaneProjection::new(51.05);
        assert_eq!(
            proj.distance((50.823194, 6.186389), (51.301389, 6.953333)) as u32,
            75646
        );
    }
}
