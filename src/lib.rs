#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

use alloc::vec::Vec;
use core::{convert::TryInto, fmt, str::FromStr};

mod bin;
pub use bin::DisplayBin;
use bin::{Bin, Bucket};

#[derive(Debug, Clone, Default)]
pub struct Histogram {
    bins: alloc::vec::Vec<Bucket>,
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

impl Histogram {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn with_capacity(bins: usize) -> Self {
        Self {
            bins: Vec::with_capacity(bins),
        }
    }

    #[must_use]
    pub fn max(&self) -> f64 {
        self.quantile(1.0).unwrap_or(f64::NAN)
    }

    #[must_use]
    pub fn min(&self) -> f64 {
        self.quantile(0.0).unwrap_or(f64::NAN)
    }

    // #[must_use]
    // pub fn mean(&self) -> f64 {
    //     todo!()
    // }

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
        if !is_sorted_by(quantiles, |q| *q) {
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
        let total_count: u64 = self.bins.iter().filter_map(Bucket::count).sum();
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
        let mut bins = self.bins.iter().skip_while(|bucket| bucket.bin.is_nan());
        let (mut bin_width, mut bin_left, mut upper_cnt) = {
            let bucket = bins
                .next()
                .ok_or(QuantilesError::Quantile(QuantileError::EmptyHistogram))?;
            (
                bucket.bin.bin_width(),
                bucket.bin.left(),
                bucket.count as f64,
            )
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
                let Some(bucket) = bins.next() else { break };
                bin_width = bucket.bin.bin_width();
                bin_left = bucket.bin.left();
                lower_cnt = upper_cnt;
                upper_cnt = lower_cnt + bucket.count as f64;
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

    // func (h *Histogram) ApproxMean() float64 {
    pub fn approx_mean(&self) -> f64 {
        // if h.useLocks {
        //     h.mutex.RLock()
        //     defer h.mutex.RUnlock()
        // }
        // divisor := 0.0
        // sum := 0.0
        let mut divisor = 0.0;
        // for i := uint16(0); i < h.used; i++ {
        //     midpoint := h.bvs[i].midpoint()
        //     cardinality := float64(h.bvs[i].count)
        //     divisor += cardinality
        //     sum += midpoint * cardinality
        // }
        let sum: f64 = self
            .bins
            .iter()
            .map(|bucket| {
                let cardinality = bucket.count as f64;
                divisor += cardinality;
                bucket.bin.midpoint() * cardinality
            })
            .sum();
        // if divisor == 0.0 {
        //     return math.NaN()
        // }
        if divisor == 0.0 {
            return f64::NAN;
        }
        // return sum / divisor
        sum / divisor
    }

    // func (h *Histogram) ApproxSum() float64 {
    pub fn approx_sum(&self) -> f64 {
        // if h.useLocks {
        //     h.mutex.RLock()
        //     defer h.mutex.RUnlock()
        // }
        // sum := 0.0
        // for i := uint16(0); i < h.used; i++ {
        //     midpoint := h.bvs[i].midpoint()
        //     cardinality := float64(h.bvs[i].count)
        //     sum += midpoint * cardinality
        // }
        self.bins
            .iter()
            .map(|&Bucket { bin, count }| bin.midpoint() * count as f64)
            .sum()
        // return sum
    }

    pub fn merge_from(&mut self, other: &Self) {
        // the Go impl does a much more complicated thing, but this should also
        // work...
        for &Bucket { count, bin } in &other.bins {
            let count = count.try_into().unwrap_or(i64::MAX);
            self.insert(bin, count);
        }
    }

    pub fn display_bins(&self) -> impl Iterator<Item = DisplayBin<'_>> + '_ {
        self.bins.iter().map(DisplayBin)
    }

    pub fn from_strs<A: AsRef<str>>(
        strs: impl IntoIterator<Item = A>,
    ) -> Result<Histogram, ParseError> {
        let strs = strs.into_iter();
        let sz = match strs.size_hint() {
            (_, Some(sz)) => sz,
            (sz, None) => sz,
        };
        // the input may include multiple bins of the same value, with
        // potentially differing counts. so, rather than parsing bins and
        // `collect`ing into a `Vec<Bin>`, we create a new `Histogram` and
        // `insert` into it. this way, multiple bins of the same value are
        // coalesced.
        let mut histogram = Self::with_capacity(sz);
        for (i, bin) in strs.enumerate() {
            let (bin, count) = Bin::from_str(bin.as_ref()).map_err(|bin| ParseError { bin, i })?;
            histogram.insert(bin, count);
        }
        Ok(histogram)
    }

    fn insert(&mut self, bin: Bin, count: i64) {
        debug_assert!(is_sorted_by(&self.bins, |bucket| bucket.bin));
        match self.bins.binary_search_by_key(&bin, |bucket| bucket.bin) {
            // if `binary_search` returns `Ok`, an existing bin matches, so
            // insert there.
            Ok(idx) => self.bins[idx].update(count),
            // if `binary_search` returns `Err`, then we need to either insert
            // before or after the existing bin.
            Err(mut idx) => {
                if count < 0 {
                    return;
                }
                // index is past the last bin, push to the end without having to
                // first check.
                if idx >= self.bins.len() {
                    self.bins.push(Bucket {
                        bin,
                        count: count as u64,
                    });
                    return;
                }

                let partition = &self.bins[idx];
                // if the new bin is greater than the bin at `index`, insert after.
                if bin > partition.bin {
                    idx += 1;
                }
                self.bins.insert(
                    idx,
                    Bucket {
                        bin,
                        count: count as u64,
                    },
                );
            }
        }
    }
}

impl PartialEq for Histogram {
    fn eq(&self, other: &Self) -> bool {
        self.bins
            .iter()
            .zip(other.bins.iter())
            .all(|(my_bin, their_bin)| my_bin == their_bin)
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

impl FromStr for Histogram {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, ParseError> {
        let strs = s.trim().split(|c| c == ',' || c == '\n');
        Self::from_strs(strs)
    }
}

fn is_sorted_by<T, U: PartialOrd>(slice: impl AsRef<[T]>, f: impl Fn(&T) -> U) -> bool {
    slice.as_ref().windows(2).all(|w| f(&w[0]) <= f(&w[1]))
}
