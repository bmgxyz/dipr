use std::ops::RangeInclusive;

use uom::si::{
    angle::degree,
    f32::{Angle, Velocity},
};

use crate::{ParseResult, inch_per_hour, utils::*};

#[derive(Clone, Debug, PartialEq, PartialOrd, Default)]
/// Precipitation rates measured in a particular direction
pub struct Radial {
    /// Bearing along which this radial points
    pub azimuth: Angle,
    /// Angle that the radar beam made with respect to horizontal
    pub elevation: Angle,
    /// Angular size of this radial
    pub width: Angle,
    /// Measured precipitation rates in each bin in this radial in ascending order of distance
    pub precip_rates: Vec<Velocity>,
}

impl Radial {
    const NAME: &'static str = "radial";
    const AZIMUTH_RANGE: RangeInclusive<f32> = (0.)..=360.;
    const ELEVATION_RANGE: RangeInclusive<f32> = (-1.)..=45.;
    const WIDTH_RANGE: RangeInclusive<f32> = (0.)..=2.;
    const NUM_BINS_RANGE: RangeInclusive<i32> = 0..=1840;
}

/// Parse Radial Information Data Structure (Figure E-4)
pub(crate) fn radial(input: &[u8]) -> ParseResult<Radial> {
    let (azimuth, tail) = take_float(input)?;
    check_range_inclusive(Radial::AZIMUTH_RANGE, azimuth, "azimuth", Radial::NAME)?;

    let (elevation, tail) = take_float(tail)?;
    check_range_inclusive(
        Radial::ELEVATION_RANGE,
        elevation,
        "elevation",
        Radial::NAME,
    )?;

    let (width, tail) = take_float(tail)?;
    check_range_inclusive(Radial::WIDTH_RANGE, width, "width", Radial::NAME)?;

    let (num_bins, tail) = take_i32(tail)?;
    check_range_inclusive(Radial::NUM_BINS_RANGE, num_bins, "num bins", Radial::NAME)?;

    let (_attributes, tail) = take_string(tail)?;
    let (_, tail) = take_bytes(tail, 4)?;
    let mut precip_rates = Vec::with_capacity(num_bins as usize);
    let (precip_rate_bytes, tail) = take_bytes(tail, (num_bins * 4) as u16)?;
    for idx in 0..num_bins {
        let buf: [u8; 2] = precip_rate_bytes[(idx * 4 + 2) as usize..(idx * 4 + 4) as usize]
            .try_into()
            .unwrap();
        precip_rates.push(Velocity::new::<inch_per_hour>(
            u16::from_be_bytes(buf) as f32 / 1000.,
        ));
    }
    Ok((
        Radial {
            azimuth: Angle::new::<degree>(azimuth),
            elevation: Angle::new::<degree>(elevation),
            width: Angle::new::<degree>(width),
            precip_rates,
        },
        tail,
    ))
}
