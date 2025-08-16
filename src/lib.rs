// Values that define WGS84 ellipsoid model of the Earth in meters.
const EQUATORIAL_RADIUS: f64 = 6378137.0;
const FLATTENING: f64 = 1.0 / 298.257223563;
const SQUARED_ECCENTRICITY: f64 = FLATTENING * (2.0 - FLATTENING);

/// A coordinate in (latitude, longitude) format.
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
///
/// let heading = proj.heading((55.704141722528554, 13.191304107330561), (55.60330902847681, 13.001973666557435));
/// assert_eq!(heading as u32, 226);
/// ```
#[derive(Clone)]
pub struct PlaneProjection {
    /// Meters per degree of longitude.
    lon_scale: f64,
    /// Meters per degree of latitude.
    lat_scale: f64,
}

impl PlaneProjection {
    /// Creates a plane projection to the Earth at provided latitude.
    pub fn new(latitude: f64) -> Self {
        // `cosf32` gives sufficient precision (adds approx. 0.0001 meter error) with much better performance
        let cos_lat = (latitude as f32).to_radians().cos() as f64;

        // Based on https://en.wikipedia.org/wiki/Earth_radius#Meridional
        let w2 = 1.0 / (1.0 - SQUARED_ECCENTRICITY * (1.0 - cos_lat * cos_lat));
        let w = w2.sqrt();
        let lon_scale = (EQUATORIAL_RADIUS * w * cos_lat).to_radians(); // based on normal radius of curvature
        let lat_scale = (EQUATORIAL_RADIUS * w * w2 * (1.0 - SQUARED_ECCENTRICITY)).to_radians(); // based on meridonal radius of curvature

        Self {
            lon_scale,
            lat_scale,
        }
    }

    /// Projects a coordinate from (latitude, longitude) to the plane projection space.
    ///
    /// This function is intended for low-level coordinate manipulation (like vector math) in the projection space
    /// and should not be used unless the built-in methods like [`PlaneProjection::distance()`] and
    /// [`PlaneProjection::distance_to_segment()`] are insufficient for your use case.
    #[inline(always)]
    pub fn project(&self, ll: LatLon) -> (f64, f64) {
        (ll.0 * self.lat_scale, ll.1 * self.lon_scale)
    }

    /// Square distance in meters between two points in (lat, lon) format.
    #[inline(always)]
    pub fn square_distance(&self, a: LatLon, b: LatLon) -> f64 {
        let lat_dist = (a.0 - b.0) * self.lat_scale;
        let lon_dist = lon_diff(a.1, b.1) * self.lon_scale;
        lat_dist * lat_dist + lon_dist * lon_dist
    }

    /// Distance in meters between two points in (lat, lon) format.
    #[inline(always)]
    pub fn distance(&self, a: LatLon, b: LatLon) -> f64 {
        self.square_distance(a, b).sqrt()
    }

    /// Square distance in meters from point to the segment.
    pub fn square_distance_to_segment(&self, point: LatLon, segment: (LatLon, LatLon)) -> f64 {
        // Convert point and segment to projected space with origin at segment start
        let mut point = (
            (point.0 - segment.0.0) * self.lat_scale,
            lon_diff(point.1, segment.0.1) * self.lon_scale,
        );
        let segment = (
            (segment.1.0 - segment.0.0) * self.lat_scale,
            lon_diff(segment.1.1, segment.0.1) * self.lon_scale,
        );
        if segment.0 != 0.0 || segment.1 != 0.0 {
            let projection = (point.0 * segment.0 + point.1 * segment.1)
                / (segment.0 * segment.0 + segment.1 * segment.1);
            if projection > 1.0 {
                point.0 -= segment.0;
                point.1 -= segment.1;
            } else if projection > 0.0 {
                point.0 -= segment.0 * projection;
                point.1 -= segment.1 * projection;
            }
        }
        point.0 * point.0 + point.1 * point.1
    }

    /// Distance in meters from point to the segment.
    #[inline(always)]
    pub fn distance_to_segment(&self, point: LatLon, segment: (LatLon, LatLon)) -> f64 {
        self.square_distance_to_segment(point, segment).sqrt()
    }

    /// Heading (azimuth) in degrees from point `a` to point `b` in the range [0.0, 360.0) degrees,
    /// measured clockwise from North: 0.0 is North, 90.0 is East, 180.0 is South and 270.0 is West.
    #[inline(always)]
    pub fn heading(&self, a: LatLon, b: LatLon) -> f32 {
        // Convert to f32 for better `atan2` performance while maintaining sufficient precision
        let dx = ((a.0 - b.0) * self.lat_scale) as f32;
        let dy = (lon_diff(b.1, a.1) * self.lon_scale) as f32;

        // Together with inverted `dx` this converts (-180, 180] `atan2` range into [0, 360) without branching
        180.0 - dy.atan2(dx).to_degrees()
    }
}

/// Returns the difference between two longitudes in range [-180.0, 180.0] degrees.
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

    const MALMO_C: LatLon = (55.60330902847681, 13.001973666557435);
    const LUND_C: LatLon = (55.704141722528554, 13.191304107330561);
    const STOCKHOLM_C: LatLon = (59.33036105663399, 18.058682977850953);

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
        let proj = PlaneProjection::new(55.65);
        assert_eq!(proj.distance(MALMO_C, LUND_C).round() as u32, 16374);

        // Geodesic distance is between Malmo and Stockholm is 513_861m and the best precision from
        // the plane projection is when halfway latitude is used.
        let proj = PlaneProjection::new((MALMO_C.0 + STOCKHOLM_C.0) * 0.5);
        assert_eq!(proj.distance(MALMO_C, STOCKHOLM_C).round() as u32, 514_168); // 0.06% error
        let proj = PlaneProjection::new(MALMO_C.0);
        assert_eq!(proj.distance(MALMO_C, STOCKHOLM_C).round() as u32, 523_230); // 1.8% error
        let proj = PlaneProjection::new(STOCKHOLM_C.0);
        assert_eq!(proj.distance(MALMO_C, STOCKHOLM_C).round() as u32, 505_217); // 1.7% error
    }

    #[test]
    fn distance_to_segment_test() {
        let proj = PlaneProjection::new(0.0);
        assert_eq!(
            proj.distance_to_segment((0.0, 0.0), ((0.0, 0.0), (0.0, 1.0))),
            0.0
        );
        assert_eq!(
            proj.distance_to_segment((0.0, 0.5), ((0.0, 0.0), (0.0, 1.0))),
            0.0
        );
        assert_eq!(
            proj.distance_to_segment((0.0, 1.0), ((0.0, 0.0), (0.0, 1.0))),
            0.0
        );
        assert_eq!(
            proj.distance_to_segment((0.0, 0.0), ((0.0, 1.0), (0.0, 2.0))),
            proj.distance((0.0, 0.0), (0.0, 1.0))
        );
        assert_eq!(
            proj.distance_to_segment((0.0, 4.5), ((0.0, 1.0), (0.0, 3.0))),
            proj.distance((0.0, 4.5), (0.0, 3.0))
        );

        // zero-length segment
        assert_eq!(
            proj.distance_to_segment((1.0, 2.0), ((3.0, 3.0), (3.0, 3.0))),
            proj.distance((1.0, 2.0), (3.0, 3.0))
        );

        let proj = PlaneProjection::new(55.65);
        assert_eq!(
            proj.distance_to_segment(MALMO_C, (LUND_C, LUND_C)) as u32,
            16373
        );

        // This is why it's tricky - point get projected not to a straight line in the lat/lon space,
        // but to a segment in the projected space.
        assert_eq!(
            proj.distance_to_segment((55.67817981392954, 13.058789566271836), (MALMO_C, LUND_C))
                as u32,
            3615
        );
    }

    #[test]
    fn heading_test() {
        let proj = PlaneProjection::new(55.65);
        assert_eq!(proj.heading((55.70, 13.19), (55.80, 13.19)) as i32, 0);
        assert_eq!(proj.heading((55.70, 13.19), (55.60, 13.19)) as i32, 180);
        assert_eq!(proj.heading((55.70, 13.19), (55.70, 13.29)) as i32, 90);
        assert_eq!(proj.heading((55.70, 13.19), (55.70, 13.09)) as i32, 270);

        assert_eq!(proj.heading(MALMO_C, LUND_C,) as i32, 46);
        assert_eq!(proj.heading(LUND_C, MALMO_C,) as i32, 180 + 46);
    }
}
