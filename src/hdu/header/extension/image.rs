use std::collections::HashMap;
use async_trait::async_trait;
use serde::Serialize;

use crate::card::Value;
use crate::error::Error;
use crate::hdu::header::check_for_bitpix;
use crate::hdu::header::check_for_naxis;
use crate::hdu::header::BitpixValue;
use crate::hdu::header::Xtension;

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct Image {
    // A number of bit that each pixel has
    bitpix: BitpixValue,
    // The number of axis
    naxis: usize,
    // The size of each axis
    naxisn: Vec<u64>,
}

impl Image {
    /// Get the number of axis given by the "NAXIS" card
    pub fn get_naxis(&self) -> usize {
        self.naxis
    }

    /// Get the size of an axis given by the "NAXISX" card
    pub fn get_naxisn(&self, idx: usize) -> Option<&u64> {
        // NAXIS indexes begins at 1 instead of 0
        self.naxisn.get(idx - 1)
    }

    /// Get the bitpix value given by the "BITPIX" card
    pub fn get_bitpix(&self) -> BitpixValue {
        self.bitpix
    }
}

#[async_trait(?Send)]
impl Xtension for Image {
    fn get_num_bytes_data_block(&self) -> u64 {
        let num_pixels = if self.naxisn.is_empty() {
            0
        } else {
            self.naxisn.iter().fold(1, |mut total, val| {
                total *= val;
                total
            })
        };

        let num_bits = ((self.bitpix as i32).unsigned_abs() as u64) * num_pixels;
        num_bits >> 3
    }

    fn parse(
        values: &HashMap<String, Value>
    ) -> Result<Self, Error> {
        // BITPIX
        let bitpix = check_for_bitpix(dbg!(values))?;
        // NAXIS
        let naxis = check_for_naxis(values)?;
        // The size of each NAXIS
        let naxisn = (0..naxis)
            .map(|naxis_i| {
                let naxis = format!("NAXIS{}", (naxis_i + 1));
                if let Some(Value::Integer { value, .. }) = values.get(&naxis) {
                    Ok(*value as u64)
                } else {
                    Err(Error::FailFindingKeyword(naxis))
                }
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Image {
            bitpix,
            naxis,
            naxisn,
        })
    }
}
