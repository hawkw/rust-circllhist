use core::{cmp, fmt, num, str::FromStr};

#[must_use = "a DisplayBin does nothing unless formatted"]
pub struct DisplayBin<'hist>(pub(crate) &'hist Bin);

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Bin {
    pub(crate) count: u64,
    val: i8,
    exp: i8,
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum ParseBinError {
    ParseCount(num::ParseIntError),
    ParseBin(num::ParseFloatError),
    NoBin,
    NoCount,
    Expected(&'static str),
}

/// Lookup table for f64 powers of 10.
const POWS_OF_TEN: [f64; 256] = [
    1.0, 10.0, 100.0, 1000.0, 10000.0, 100000.0, 1e+06, 1e+07, 1e+08, 1e+09, 1e+10, 1e+11, 1e+12,
    1e+13, 1e+14, 1e+15, 1e+16, 1e+17, 1e+18, 1e+19, 1e+20, 1e+21, 1e+22, 1e+23, 1e+24, 1e+25,
    1e+26, 1e+27, 1e+28, 1e+29, 1e+30, 1e+31, 1e+32, 1e+33, 1e+34, 1e+35, 1e+36, 1e+37, 1e+38,
    1e+39, 1e+40, 1e+41, 1e+42, 1e+43, 1e+44, 1e+45, 1e+46, 1e+47, 1e+48, 1e+49, 1e+50, 1e+51,
    1e+52, 1e+53, 1e+54, 1e+55, 1e+56, 1e+57, 1e+58, 1e+59, 1e+60, 1e+61, 1e+62, 1e+63, 1e+64,
    1e+65, 1e+66, 1e+67, 1e+68, 1e+69, 1e+70, 1e+71, 1e+72, 1e+73, 1e+74, 1e+75, 1e+76, 1e+77,
    1e+78, 1e+79, 1e+80, 1e+81, 1e+82, 1e+83, 1e+84, 1e+85, 1e+86, 1e+87, 1e+88, 1e+89, 1e+90,
    1e+91, 1e+92, 1e+93, 1e+94, 1e+95, 1e+96, 1e+97, 1e+98, 1e+99, 1e+100, 1e+101, 1e+102, 1e+103,
    1e+104, 1e+105, 1e+106, 1e+107, 1e+108, 1e+109, 1e+110, 1e+111, 1e+112, 1e+113, 1e+114, 1e+115,
    1e+116, 1e+117, 1e+118, 1e+119, 1e+120, 1e+121, 1e+122, 1e+123, 1e+124, 1e+125, 1e+126, 1e+127,
    1e-128, 1e-127, 1e-126, 1e-125, 1e-124, 1e-123, 1e-122, 1e-121, 1e-120, 1e-119, 1e-118, 1e-117,
    1e-116, 1e-115, 1e-114, 1e-113, 1e-112, 1e-111, 1e-110, 1e-109, 1e-108, 1e-107, 1e-106, 1e-105,
    1e-104, 1e-103, 1e-102, 1e-101, 1e-100, 1e-99, 1e-98, 1e-97, 1e-96, 1e-95, 1e-94, 1e-93, 1e-92,
    1e-91, 1e-90, 1e-89, 1e-88, 1e-87, 1e-86, 1e-85, 1e-84, 1e-83, 1e-82, 1e-81, 1e-80, 1e-79,
    1e-78, 1e-77, 1e-76, 1e-75, 1e-74, 1e-73, 1e-72, 1e-71, 1e-70, 1e-69, 1e-68, 1e-67, 1e-66,
    1e-65, 1e-64, 1e-63, 1e-62, 1e-61, 1e-60, 1e-59, 1e-58, 1e-57, 1e-56, 1e-55, 1e-54, 1e-53,
    1e-52, 1e-51, 1e-50, 1e-49, 1e-48, 1e-47, 1e-46, 1e-45, 1e-44, 1e-43, 1e-42, 1e-41, 1e-40,
    1e-39, 1e-38, 1e-37, 1e-36, 1e-35, 1e-34, 1e-33, 1e-32, 1e-31, 1e-30, 1e-29, 1e-28, 1e-27,
    1e-26, 1e-25, 1e-24, 1e-23, 1e-22, 1e-21, 1e-20, 1e-19, 1e-18, 1e-17, 1e-16, 1e-15, 1e-14,
    1e-13, 1e-12, 1e-11, 1e-10, 1e-09, 1e-08, 1e-07, 1e-06, 1e-05, 0.0001, 0.001, 0.01, 0.1,
];

// === impl DisplayBin ===

impl fmt::Display for DisplayBin<'_> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.0, f)
    }
}

impl fmt::LowerExp for DisplayBin<'_> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::LowerExp::fmt(self.0, f)
    }
}

impl fmt::UpperExp for DisplayBin<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::UpperExp::fmt(self.0, f)
    }
}

// === impl Bin ===

impl Bin {
    pub(crate) fn count(&self) -> Option<u64> {
        if self.is_nan() {
            return None;
        }

        Some(self.count)
    }
    // func (hb *bin) setFromFloat64(d float64) *bin { //nolint:unparam
    fn set_f64(&mut self, mut f: f64) -> &mut Self {
        // hb.val = -1
        self.val = -1;
        // if math.IsInf(d, 0) || math.IsNaN(d) {
        //     return hb
        // }
        if f.is_infinite() || f.is_nan() {
            return self;
        }
        // if d == 0.0 {
        //     hb.val = 0
        //     return hb
        // }
        if f == 0.0 {
            self.val = 0;
            return self;
        }
        // sign := 1
        // if math.Signbit(d) {
        //     sign = -1
        // }
        let sign = if f.is_sign_negative() { -1 } else { 1 };
        // d = math.Abs(d)
        f = f.abs();
        // bigExp := int(math.Floor(math.Log10(d)))
        let big_exp = f.log10().floor() as i64;
        // hb.exp = int8(bigExp)
        let exp = big_exp as i8;
        // if int(hb.exp) != bigExp { // rolled
        //     hb.exp = 0
        //     if bigExp < 0 {
        //         hb.val = 0
        //     }
        //     return hb
        // }
        if exp as i64 != big_exp {
            self.exp = 0;
            if big_exp < 0 {
                self.val = 0;
            }
            return self;
        } else {
            self.exp = exp;
        }
        // d /= hb.powerOfTen()
        f /= self.pow_10();
        // d *= 10
        f *= 10.0;
        // hb.val = int8(sign * int(math.Floor(d+1e-13)))
        self.val = (sign * ((f + 1e-13).floor() as i64)) as i8;
        // if hb.val == 100 || hb.val == -100 {
        //     if hb.exp < 127 {
        //         hb.val /= 10
        //         hb.exp++
        //     } else {
        //         hb.val = 0
        //         hb.exp = 0
        //     }
        // }
        if self.val.abs() == 100 {
            if self.exp < 127 {
                self.val /= 10;
                self.exp += 1;
            } else {
                self.val = 0;
                self.exp = 0
            }
        }
        // if hb.val == 0 {
        //     hb.exp = 0
        //     return hb
        // }
        if self.val == 0 {
            self.exp = 0;
            return self;
        }
        // if !((hb.val >= 10 && hb.val < 100) ||
        //     (hb.val <= -10 && hb.val > -100)) {
        //     hb.val = -1
        //     hb.exp = 0
        // }
        if !(self.val >= 10 && self.val < 100) || (self.val <= -10 && self.val > -100) {
            self.val = -1;
            self.exp = 0
        }
        self
    }

    // func (h *Histogram) updateOldBinAt(idx uint16, count int64) uint64 {
    pub(crate) fn update(&mut self, count: i64) {
        // var newval uint64
        // if count >= 0 {
        //     newval = h.bvs[idx].count + uint64(count)
        // } else {
        //     newval = h.bvs[idx].count - uint64(-count)
        // }
        // if newval < h.bvs[idx].count { // rolled
        //     newval = ^uint64(0)
        // }
        // h.bvs[idx].count = newval
        // return newval - h.bvs[idx].count
        self.count = self.count.saturating_add_signed(count)
    }

    pub(crate) fn from_f64(f: f64) -> Self {
        let mut this = Self {
            count: 0,
            val: 0,
            exp: 0,
        };
        this.set_f64(f);
        this
    }

    pub(crate) fn from_int_scale(mut val: i64, mut scale: i32) -> Self {
        if val == 0 {
            return Bin {
                count: 0,
                val: 0,
                exp: 0,
            };
        }
        let sign = val.signum();
        val = val.abs();
        if val < 10 {
            val *= 10;
        } else {
            scale += 1;
        }

        if val >= 100 {
            let log10 = val.checked_ilog10().unwrap_or(0);
            val = val.checked_rem(10 * log10 as i64).unwrap_or(val);
            scale += log10 as i32;
        }

        if scale > 127 {
            val = 0xff;
            scale = 0;
        } else if scale < -128 {
            val = 0;
            scale = 0;
        }

        val *= sign;

        Bin {
            count: 0,
            val: val as i8,
            exp: scale as i8,
        }
    }

    pub(crate) fn midpoint(&self) -> f64 {
        // if hb.isNaN() {
        //     return math.NaN()
        // }
        // out := hb.value()
        // if out == 0 {
        //     return 0
        // }
        let val = self.value();
        if val.is_nan() || val == 0.0 {
            return val;
        }

        // interval := hb.binWidth()
        // if out < 0 {
        //     interval *= -1
        // }
        let interval = self.bin_width() * val.signum();

        // return out + interval/2.0
        val + interval / 2.0
    }

    fn pow_10(&self) -> f64 {
        POWS_OF_TEN[self.exp as u8 as usize]
    }

    // func (hb *bin) isNaN() bool {
    pub(crate) fn is_nan(&self) -> bool {
        // aval := hb.val
        // if aval < 0 {
        // 	aval = -aval
        // }
        match self.val.abs() {
            // if 99 < aval { // in [100... ]: nan
            // 	return true
            // }
            val if 99 < val => true,
            // if 9 < aval { // in [10 - 99]: valid range
            // 	return false
            // }
            val if 9 < val => false,
            // if 0 < aval { // in [1  - 9 ]: nan
            // 	return true
            // }
            val if 0 < val => true,
            // if 0 == aval { // in [0]      : zero bucket
            // 	return false
            // }
            0 => false,
            _ => false,
            // return false
        }
    }

    // func (hb *bin) value() float64 {
    pub(crate) fn value(&self) -> f64 {
        // if hb.isNaN() {
        //     return math.NaN()
        // }
        if self.is_nan() {
            return f64::NAN;
        }
        // if hb.val < 10 && hb.val > -10 {
        //     return 0.0
        // }
        if self.val < 10 && self.val > -10 {
            return 0.0;
        }
        // return (float64(hb.val) / 10.0) * hb.powerOfTen()
        (self.val as f64 / 10.0) * self.pow_10()
    }

    // func (hb *bin) left() float64 {
    pub(crate) fn left(&self) -> f64 {
        // if hb.isNaN() {
        //     return math.NaN()
        // }
        // out := hb.value()
        // if out >= 0 {
        //     return out
        // }
        // return out - hb.binWidth()
        match self.value() {
            v if v.is_nan() => f64::NAN,
            v if v >= 0.0 => v,
            v => v - self.bin_width(),
        }
    }

    // func (hb *bin) binWidth() float64 {
    pub(crate) fn bin_width(&self) -> f64 {
        // if hb.isNaN() {
        //     return math.NaN()
        // }
        if self.is_nan() {
            return f64::NAN;
        }
        // if hb.val < 10 && hb.val > -10 {
        //     return 0.0
        // }
        if self.val < 10 && self.val > -10 {
            return 0.0;
        }
        // return hb.powerOfTen() / 10.0
        self.pow_10() / 10.0
    }

    // // func (hb *bin) compare(h2 *bin) int {
    // pub(crate) fn difference(&self, other: &Self) -> isize {
    // //     var v1, v2 int

    // //     // 1) slide exp positive
    // //     // 2) shift by size of val multiple by (val != 0)
    // //     // 3) then add or subtract val accordingly

    // //     if hb.val >= 0 {
    // //         v1 = ((int(hb.exp)+256)<<8)*(((int(hb.val)|(^int(hb.val)+1))>>8)&1) + int(hb.val)
    // //     } else {
    // //         v1 = ((int(hb.exp)+256)<<8)*(((int(hb.val)|(^int(hb.val)+1))>>8)&1) - int(hb.val)
    // //     }
    //     let v1 = if self.val >= 0 {
    //             let v = self.val as isize;
    //             (self.exp as isize + 256) << 8 * ((v | (!v + 1)) >> 8) & 1) + int
    //     }
    // //     if h2.val >= 0 {
    // //         v2 = ((int(h2.exp)+256)<<8)*(((int(h2.val)|(^int(h2.val)+1))>>8)&1) + int(h2.val)
    // //     } else {
    // //         v2 = ((int(h2.exp)+256)<<8)*(((int(h2.val)|(^int(h2.val)+1))>>8)&1) - int(h2.val)
    // //     }

    // //     // return the difference
    // //     return v2 - v1
    // }
}

impl FromStr for Bin {
    type Err = ParseBinError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // H[0.0e+00]=1
        let s = s.trim();

        let (bin_str, count_str) = {
            let mut parts = s.split('=');
            let bin_str = parts.next().ok_or(ParseBinError::NoBin)?;
            let count_str = parts.next().ok_or(ParseBinError::NoCount)?;
            if parts.next().is_some() {
                return Err(ParseBinError::Expected(
                    "expected only one `=`, but found multiple",
                ));
            }

            (bin_str, count_str)
        };

        // H[ <0.0 e+00> ]=1
        let bin = bin_str
            .strip_prefix("H[")
            .ok_or(ParseBinError::Expected("bin to start with `H[`"))?
            .strip_suffix(']')
            .ok_or(ParseBinError::Expected("bin to end with `]`"))?
            .parse::<f64>()
            .map_err(ParseBinError::ParseBin)?;

        let count = count_str
            .parse::<u64>()
            .map_err(ParseBinError::ParseCount)?;

        Ok(Self {
            count,
            ..Self::from_f64(bin)
        })
    }
}

impl fmt::Display for Bin {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::LowerExp::fmt(self, f)
    }
}

impl fmt::LowerExp for Bin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // H[ <0.0 e+00> ]=1
        write!(f, "H[{:3.1e}]={}", self.value(), self.count)
    }
}

impl fmt::UpperExp for Bin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // H[ <0.0 e+00> ]=1
        write!(f, "H[{:3.1E}]={}", self.value(), self.count)
    }
}

impl PartialOrd for Bin {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Bin {
    fn cmp(&self, other: &Bin) -> cmp::Ordering {
        // if (exp == o.exp && val == o.val) return 0;
        if self.exp == other.exp && self.val == other.val {
            return cmp::Ordering::Equal;
        }

        // NaNs always compare less, unless both values are equal.
        if self.is_nan() {
            return cmp::Ordering::Less;
        }
        if other.is_nan() {
            return cmp::Ordering::Greater;
        }
        // `val` determines the sign, so if self's sign is different from
        // other's, it's always greater/less, regardless of the exponent.
        self.val.signum().cmp(&other.val.signum()).then_with(|| {
            // if the two values have the same signs, compare the exponent...
            self.exp
                .cmp(&other.exp)
                // and if they have the same exponent, finally compare the value
                .then_with(|| self.val.cmp(&other.val))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::{prop_assert_eq, proptest};

    proptest! {
        #[test]
        fn from_int_scale_matches_go(val: i64, scale: i32) {
            prop_assert_eq!(Bin::from_int_scale(val, scale), from_int_scale_go(val, scale))
        }

    }

    fn from_int_scale_go(mut val: i64, mut scale: i32) -> Bin {
        let mut sign = 1;
        // if val == 0 {
        // 	scale = 0
        // } else {
        // 	scale++
        // 	if val < 0 {
        // 		val = 0 - val
        // 		sign = -1
        // 	}
        // 	if val < 10 {
        // 		val *= 10
        // 		scale--
        // 	}
        // 	for val >= 100 {
        // 		val /= 10
        // 		scale++
        // 	}
        // }
        if val == 0 {
            scale = 0;
        } else {
            // TODO(eliza): this is just a modulo, right?
            scale += 1;
            if val < 0 {
                val = 0 - val;
                sign = -1;
            }
            if val < 10 {
                val *= 10;
                scale -= 1;
            }
            while val >= 100 {
                val /= 10;
                scale += 1;
            }
        }
        // if scale < -128 {
        // 	val = 0
        // 	scale = 0
        // } else if scale > 127 {
        // 	val = 0xff
        // 	scale = 0
        // }
        if scale < -128 {
            scale = 0;
            val = 0;
        } else if scale > 127 {
            val = 0xff;
            scale = 0;
        }
        val *= sign;
        Bin {
            val: val as i8,
            exp: scale as i8,
            count: 0,
        }
    }
}
