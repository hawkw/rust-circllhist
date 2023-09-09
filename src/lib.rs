#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

use alloc::vec::Vec;
use core::fmt;

mod bin;
use bin::Bin;
pub use bin::DisplayBin;

#[derive(Debug)]
pub struct Histogram {
    bins: Vec<Bin>,
}

#[derive(Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum RecordError {}

#[derive(Debug)]
#[non_exhaustive]
pub enum QuantileError {
    Empty,
    InvalidQuantile(f64),
}

#[derive(Debug, Eq, PartialEq)]
#[non_exhaustive]
pub struct ParseError {
    bin: bin::ParseBinError,
    i: usize,
}

#[derive(Debug, Clone)]
#[must_use = "a HistogramBuilder does nothing unless used to construct a histogram"]
pub struct HistogramBuilder {
    lookup_tables: bool,
    bins: usize,
}

impl Histogram {
    pub fn build() -> HistogramBuilder {
        HistogramBuilder::default()
    }

    #[must_use]
    pub fn max(&self) -> f64 {
        todo!()
    }

    #[must_use]
    pub fn min(&self) -> f64 {
        todo!()
    }

    #[must_use]
    pub fn mean(&self) -> f64 {
        todo!()
    }

    /// Returns the total number of recorded values.
    #[must_use]
    pub fn len(&self) -> usize {
        self.bins.len()
    }

    /// Returns the number of bins in the histogram.
    #[must_use]
    pub fn bin_count(&self) -> usize {
        todo!()
    }

    pub fn clear(&mut self) {
        todo!()
    }

    /// Records an integer scalar value.
    pub fn record_int_scale(&mut self, val: i64, scale: i32) -> Result<&mut Self, RecordError> {
        self.record_int_scales(val, scale, 1)
    }

    /// Records `n` occurances of an integer scalar value.
    pub fn record_int_scales(
        &mut self,
        val: i64,
        scale: i32,
        n: i64,
    ) -> Result<&mut Self, RecordError> {
        self.insert(Bin::from_int_scale(val, scale), n);
        Ok(self)
    }

    /// Record a floating point value.
    pub fn record_f64(&mut self, val: f64) -> Result<&mut Self, RecordError> {
        self.record_f64s(val, 1)
    }

    pub fn record_f64s(&mut self, val: f64, n: i64) -> Result<&mut Self, RecordError> {
        self.insert(Bin::from_f64(val), n);
        Ok(self)
    }

    pub fn approx_quantiles<const QUANTILES: usize>(
        &self,
        quantiles: &[f64; QUANTILES],
    ) -> Result<[f64; QUANTILES], QuantileError> {
        todo!()
    }

    /// Returns the recorded value at the given quantile (0..1).
    pub fn quantile(&self, quantile: f64) -> Result<f64, QuantileError> {
        self.approx_quantiles(&[quantile]).map(|q| q[0])
    }

    pub fn approx_mean(&self) -> f64 {
        todo!()
    }

    pub fn approx_sum(&self) -> f64 {
        todo!()
    }

    pub fn display_bins(&self) -> impl Iterator<Item = DisplayBin<'_>> + '_ {
        self.bins.iter().map(DisplayBin)
    }

    pub fn from_strs<A: AsRef<str>>(
        strs: impl IntoIterator<Item = A>,
    ) -> Result<Histogram, ParseError> {
        HistogramBuilder::default().from_strs(strs)
    }

    fn insert(&mut self, mut bin: Bin, count: i64) {
        match self.bins.binary_search(&bin) {
            // if `binary_search` returns `Ok`, an existing bin matches, so
            // insert there.
            Ok(idx) => self.bins[idx].update(count),
            // if `binary_search` returns `Err`, then we need to either insert
            // before or after the existing bin.
            Err(mut idx) => {
                bin.update(count);
                let partition = &self.bins[idx];
                // if the new bin is greater than the bin at `index`, insert after.
                if &bin > partition {
                    idx += 1;
                }
                self.bins.insert(idx, bin);
            }
        }
    }
}

impl PartialEq for Histogram {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl Eq for Histogram {}

impl fmt::Display for Histogram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl Default for Histogram {
    fn default() -> Self {
        HistogramBuilder::default().build()
    }
}

// === impl HistogramBuilder ===

impl HistogramBuilder {
    pub const DEFAULT_SIZE: usize = 100;

    /// Sets whether or not lookup tables are used by the [`Histogram`] being
    /// constructed.
    pub const fn with_lookup_tables(self, lookup_tables: bool) -> Self {
        Self {
            lookup_tables,
            ..self
        }
    }

    /// Sets the number of bins used by the [`Histogram`] being constructed.
    pub const fn bins(self, bins: usize) -> Self {
        Self { bins, ..self }
    }

    #[must_use]
    pub fn build(&self) -> Histogram {
        todo!()
    }

    pub fn from_strs<A: AsRef<str>>(
        &self,
        strs: impl IntoIterator<Item = A>,
    ) -> Result<Histogram, ParseError> {
        let bins = strs
            .into_iter()
            .enumerate()
            .map(|(i, bin)| {
                bin.as_ref()
                    .parse::<Bin>()
                    .map_err(|bin| ParseError { bin, i })
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Histogram { bins })
    }
}

impl Default for HistogramBuilder {
    fn default() -> Self {
        Self {
            lookup_tables: true,
            bins: Self::DEFAULT_SIZE,
        }
    }
}
