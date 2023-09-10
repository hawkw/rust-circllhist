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
    EmptyHistogram,
    OutOfBounds(f64),
}

#[derive(Debug)]
#[non_exhaustive]
pub enum QuantilesError {
    Quantile(QuantileError),
    NotSorted,
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
    pub fn record(&mut self, val: f64) -> Result<&mut Self, RecordError> {
        self.record_f64s(val, 1)
    }

    pub fn record_f64s(&mut self, val: f64, n: i64) -> Result<&mut Self, RecordError> {
        self.insert(Bin::from_f64(val), n);
        Ok(self)
    }

    pub fn approx_quantiles<const QUANTILES: usize>(
        &self,
        quantiles: &[f64; QUANTILES],
    ) -> Result<[f64; QUANTILES], QuantilesError> {
        // qOut := make([]float64, len(qIn))
        let mut out = [0.0; QUANTILES];
        // if len(qIn) == 0 {
        //     return qOut, nil
        // }
        if QUANTILES == 0 {
            return Ok(out);
        }
        // iq, ib := 0, uint16(0)
        // totalCnt, binWidth, binLeft, lowerCnt, upperCnt := 0.0, 0.0, 0.0, 0.0, 0.0
        // // Make sure the requested quantiles are in order
        // for iq = 1; iq < len(qIn); iq++ {
        //     if qIn[iq-1] > qIn[iq] {
        //         return nil, fmt.Errorf("out of order") //nolint:goerr113
        //     }
        // }
        if !is_sorted(quantiles) {
            return Err(QuantilesError::NotSorted);
        }
        // // Add up the bins
        // for ib = 0; ib < h.used; ib++ {
        //     if !h.bvs[ib].isNaN() {
        //         totalCnt += float64(h.bvs[ib].count)
        //     }
        // }
        // if totalCnt == 0.0 {
        //     return nil, fmt.Errorf("empty_histogram") //nolint:goerr113
        // }
        let total_count: u64 = self.bins.iter().filter_map(Bin::count).sum();
        if total_count == 0 {
            return Err(QuantilesError::Quantile(QuantileError::EmptyHistogram));
        }
        // for iq = 0; iq < len(qIn); iq++ {
        //     if qIn[iq] < 0.0 || qIn[iq] > 1.0 {
        //         return nil, fmt.Errorf("out of bound quantile") //nolint:goerr113
        //     }
        //     qOut[iq] = totalCnt * qIn[iq]
        // }

        for (&in_q, out_q) in quantiles.iter().zip(out.iter_mut()) {
            if !(0.0..=1.0).contains(&in_q) {
                return Err(QuantilesError::Quantile(QuantileError::OutOfBounds(in_q)));
            }
            *out_q = total_count as f64 * in_q;
        }

        // for ib = 0; ib < h.used; ib++ {
        //     if h.bvs[ib].isNaN() {
        //         continue
        //     }
        //     binWidth = h.bvs[ib].binWidth()
        //     binLeft = h.bvs[ib].left()
        //     lowerCnt = upperCnt
        //     upperCnt = lowerCnt + float64(h.bvs[ib].count)
        //     break
        // }
        let mut lower_cnt = 0.0;
        let mut bins = self.bins.iter().skip_while(|bin| bin.is_nan());
        let (mut bin_width, mut bin_left, mut upper_cnt) = {
            let bin = bins
                .next()
                .ok_or(QuantilesError::Quantile(QuantileError::EmptyHistogram))?;
            (bin.bin_width(), bin.left(), bin.count as f64)
        };
        // for iq = 0; iq < len(qIn); iq++ {
        for out_q in out.iter_mut() {
            // for ib < (h.used-1) && upperCnt < qOut[iq] {
            //     ib++
            //     binWidth = h.bvs[ib].binWidth()
            //     binLeft = h.bvs[ib].left()
            //     lowerCnt = upperCnt
            //     upperCnt = lowerCnt + float64(h.bvs[ib].count)
            // }
            while upper_cnt < *out_q {
                let Some(bin) = bins.next() else { break };
                bin_width = bin.bin_width();
                bin_left = bin.left();
                lower_cnt = upper_cnt;
                upper_cnt = lower_cnt + bin.count as f64;
            }
            // switch {
            // case lowerCnt == qOut[iq]:
            //     qOut[iq] = binLeft
            // case upperCnt == qOut[iq]:
            //     qOut[iq] = binLeft + binWidth
            // default:
            //     if binWidth == 0 {
            //         qOut[iq] = binLeft
            //     } else {
            //         qOut[iq] = binLeft + (qOut[iq]-lowerCnt)/(upperCnt-lowerCnt)*binWidth
            //     }
            // }
            *out_q = match *out_q {
                q if q == lower_cnt => bin_left,
                q if q == upper_cnt => bin_left + bin_width,
                _ if bin_width == 0.0 => bin_left,
                q => bin_left + (q - lower_cnt) / (upper_cnt - lower_cnt) * bin_width,
            }
        }
        // return qOut, nil
        Ok(out)
    }

    /// Returns the recorded value at the given quantile (0..1).
    pub fn quantile(&self, quantile: f64) -> Result<f64, QuantileError> {
        match self.approx_quantiles(&[quantile]) {
            Ok([q]) => Ok(q),
            Err(QuantilesError::NotSorted) => {
                unreachable!("there's only one quantile, so it must be sorted")
            }
            Err(QuantilesError::Quantile(e)) => Err(e),
        }
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
        debug_assert!(is_sorted(&self.bins));
        match self.bins.binary_search(&bin) {
            // if `binary_search` returns `Ok`, an existing bin matches, so
            // insert there.
            Ok(idx) => self.bins[idx].update(count),
            // if `binary_search` returns `Err`, then we need to either insert
            // before or after the existing bin.
            Err(mut idx) => {
                bin.update(count);
                // index is past the last bin, push to the end without having to
                // first check.
                if idx >= self.bins.len() {
                    self.bins.push(bin);
                    return;
                }

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
        let delim = if f.alternate() { "\n" } else { ", " };
        let mut bins = self.bins.iter();
        if let Some(bin) = bins.next() {
            fmt::Display::fmt(bin, f)?;

            for bin in bins {
                write!(f, "{delim}{bin}")?;
            }
        }

        Ok(())
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
        if self.lookup_tables {
            // TODO(eliza): implement LUTs
        }

        Histogram {
            bins: Vec::with_capacity(self.bins),
        }
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

fn is_sorted<T: PartialOrd>(slice: impl AsRef<[T]>) -> bool {
    slice.as_ref().windows(2).all(|w| w[0] <= w[1])
}
