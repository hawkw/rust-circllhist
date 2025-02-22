use circllhist::Histogram;
mod util;
// package circonusllhist_test

// import (
// 	"math"
// 	"math/rand"
// 	"testing"
// 	"time"

// 	hist "github.com/openhistogram/circonusllhist"
// )

// func fuzzyEquals(expected, actual float64) bool {
// 	delta := math.Abs(expected / 100000.0)
// 	if actual >= expected-delta && actual <= expected+delta {
// 		return true
// 	}
// 	return false
// }

// var s1 = []float64{0.123, 0, 0.43, 0.41, 0.415, 0.2201, 0.3201, 0.125, 0.13}
const S1: &[f64] = &[0.123, 0.0, 0.43, 0.41, 0.415, 0.2201, 0.3201, 0.125, 0.13];
const STRINGS: &[&str] = &[
    "H[0.0e+00]=1",
    "H[1.2e-01]=2",
    "H[1.3e-01]=1",
    "H[2.2e-01]=1",
    "H[3.2e-01]=1",
    "H[4.1e-01]=2",
    "H[4.3e-01]=1",
];

// func TestDecStrings(t *testing.T) {
// 	h := hist.New()
// 	for _, sample := range s1 {
// 		_ = h.RecordValue(sample)
// 	}
// 	out := h.DecStrings()
// 	expect := []string{"H[0.0e+00]=1", "H[1.2e-01]=2", "H[1.3e-01]=1",
// 		"H[2.2e-01]=1", "H[3.2e-01]=1", "H[4.1e-01]=2",
// 		"H[4.3e-01]=1"}
// 	for i, str := range expect {
// 		if str != out[i] {
// 			t.Errorf("DecString '%v' != '%v'", out[i], str)
// 		}
// 	}
// }
#[test]
fn display_buckets() {
    // `fmt::lowerExp` formats things *slightly* differently than Go's `%e` format,
    // but it should still parse.
    const RUST_STRINGS: &[&str] = &[
        "H[0.0e0]=1",
        "H[1.2e-1]=2",
        "H[1.3e-1]=1",
        "H[2.2e-1]=1",
        "H[3.2e-1]=1",
        "H[4.1e-1]=2",
        "H[4.3e-1]=1",
    ];
    let mut histogram = Histogram::default();
    for sample in S1 {
        histogram
            .record(*sample)
            .expect("value should be recorded successfully");
    }

    let mut bins = histogram.display_bins();
    for &expected in RUST_STRINGS {
        let formatted = bins.next().map(|bin| bin.to_string());
        assert_eq!(Some(dbg!(expected)), dbg!(formatted).as_deref());
    }

    assert!(bins.next().is_none());
}

// func TestNewFromStrings(t *testing.T) {
// 	strings := []string{"H[0.0e+00]=1", "H[1.2e-01]=2", "H[1.3e-01]=1",
// 		"H[2.2e-01]=1", "H[3.2e-01]=1", "H[4.1e-01]=2", "H[4.3e-01]=1"}

// 	// hist of single set of strings
// 	singleHist, err := hist.NewFromStrings(strings, false)
// 	if err != nil {
// 		t.Errorf("error creating hist from strings '%v'", err)
// 	}

// 	// hist of multiple sets of strings
// 	strings = append(strings, strings...)
// 	doubleHist, err := hist.NewFromStrings(strings, false)
// 	if err != nil {
// 		t.Errorf("error creating hist from strings '%v'", err)
// 	}

// 	// sanity check the sums are doubled
// 	if singleHist.ApproxSum()*2 != doubleHist.ApproxSum() {
// 		t.Error("aggregate histogram approxSum failure")
// 	}

// 	if singleHist.Equals(doubleHist) {
// 		t.Error("histograms should not be equal")
// 	}
// }
#[test]
fn from_strs() {
    let single_hist = Histogram::from_strs(STRINGS.iter()).expect("histogram should parse");

    let double_hist = Histogram::from_strs(STRINGS.iter().chain(STRINGS.iter()))
        .expect("doubled histogram should parse");
    eprintln!("single_hist:\n{single_hist:#}\ndouble_hist:\n{double_hist:#}");

    assert_approx_eq!(single_hist.approx_sum() * 2.0, double_hist.approx_sum());
    assert_ne!(single_hist, double_hist)
}

// func TestMean(t *testing.T) {
#[test]
fn mean() {
    // 	h := hist.New()
    let mut histogram = Histogram::default();
    // 	for _, sample := range s1 {
    // 		_ = h.RecordValue(sample)
    // 	}
    for sample in S1 {
        histogram
            .record(*sample)
            .expect("value should be recorded successfully");
    }
    // 	mean := h.ApproxMean()
    // 	if !fuzzyEquals(0.2444444444, mean) {
    // 		t.Errorf("mean() -> %v != %v", mean, 0.24444)
    // 	}
    assert_approx_eq!(0.2444444444, histogram.approx_mean());
}

// func TestQuantiles(t *testing.T) {
//  helpQTest(t, []float64{1}, []float64{0, 0.25, 0.5, 1}, []float64{1, 1.025, 1.05, 1.1})
#[test]
fn quantiles1() {
    util::test_quantiles(&[1.0], [0.0, 0.25, 0.5, 1.0], [1.0, 1.025, 1.05, 1.1]);
}

// 	helpQTest(t, s1, []float64{0, 0.95, 0.99, 1.0}, []float64{0, 0.4355, 0.4391, 0.44})
#[test]
fn quantiles2() {
    util::test_quantiles(S1, [0.0, 0.95, 0.99, 1.0], [0.0, 0.4355, 0.4391, 0.44]);
}

#[test]
fn quantiles3() {
    // 	helpQTest(t, []float64{1.0, 2.0}, []float64{0.5}, []float64{1.1})
    util::test_quantiles(&[1.0, 2.0], [0.5], [1.1]);
}

#[test]
fn quantiles4() {
    // 	helpQTest(t, []float64{1.0, 1e200}, []float64{0, 1}, []float64{1.0, 1.1})
    util::test_quantiles(&[1.0, 1e200], [0.0, 1.0], [1.0, 1.1]);
}

#[test]
fn quantiles5() {
    // 	helpQTest(t, []float64{1e200, 1e200, 1e200, 0, 0, 1e-20, 1e-20, 1e-20, 1e-10}, []float64{0, 1},
    // 		[]float64{0, 1.1e-10})
    util::test_quantiles(
        &[1e200, 1e200, 1e200, 0.0, 0.0, 1e-20, 1e-20, 1e-20, 1e-10],
        [0.0, 1.0],
        [0.0, 1.1e-10],
    );
}

#[test]
fn quantiles6() {
    // 	helpQTest(t, []float64{0, 1}, []float64{0, 0.1}, []float64{0, 0})
    util::test_quantiles(&[0.0, 0.1], [0.0, 0.1], [0.0, 0.0])
}

// func TestCompare(t *testing.T) {
// 	// var h1, h2 *Bin
// }

// func TestConcurrent(t *testing.T) {
// 	h := hist.New()
// 	for r := 0; r < 100; r++ {
// 		go func(t *testing.T) {
// 			for j := 0; j < 100; j++ {
// 				for i := 50; i < 100; i++ {
// 					if err := h.RecordValue(float64(i)); err != nil {
// 						t.Error(err)
// 						return
// 					}
// 				}
// 			}
// 		}(t)
// 	}
// }

// func TestRang(t *testing.T) {
// 	h1 := hist.New()
// 	rnd := rand.New(rand.NewSource(time.Now().UnixNano()))
// 	for i := 0; i < 1000000; i++ {
// 		_ = h1.RecordValue(rnd.Float64() * 10)
// 	}
// }

// func TestEquals(t *testing.T) {
// 	h1 := hist.New()
// 	for i := 0; i < 1000000; i++ {
// 		if err := h1.RecordValue(float64(i)); err != nil {
// 			t.Fatal(err)
// 		}
// 	}

// 	h2 := hist.New()
// 	for i := 0; i < 10000; i++ {
// 		if err := h1.RecordValue(float64(i)); err != nil {
// 			t.Fatal(err)
// 		}
// 	}

// 	if h1.Equals(h2) {
// 		t.Error("Expected Histograms to not be equivalent")
// 	}

// 	h1.Reset()
// 	h2.Reset()

// 	if !h1.Equals(h2) {
// 		t.Error("Expected Histograms to be equivalent")
// 	}
// }

// func TestMinMaxMean(t *testing.T) {
#[test]
fn min_max_mean() {
    // 	const (
    // 		minVal = 0
    // 		maxVal = 1000000
    // 	)
    const MIN: usize = 0;
    const MAX: usize = 1000000;
    // 	h := hist.New()
    // 	for i := minVal; i < maxVal; i++ {
    // 		if err := h.RecordValue(float64(i)); err != nil {
    // 			t.Fatal(err)
    // 		}
    // 	}
    let mut h = Histogram::default();
    for i in MIN..MAX {
        h.record(i as f64)
            .expect("value should be recorded successfully");
    }

    // 	if h.Min() > minVal {
    // 		t.Error("incorrect min value")
    // 	}
    assert_eq!(h.min(), MIN as f64, "incorrect `min` value");
    // 	if h.Max() < maxVal {
    // 		t.Error("incorrect max value")
    // 	}
    assert_eq!(h.max(), MAX as f64, "incorrect `max` value");
    // 	round := func(val float64) int {
    // 		if val < 0 {
    // 			return int(val - 0.5)
    // 		}
    // 		return int(val + 0.5)
    // 	}
    let round = |val: f64| -> i64 {
        let r = 0.5 * val.signum();
        (val + r) as i64
    };
    // 	if round(h.Mean()) != round(maxVal/2) {
    // 		t.Errorf("incorrect mean value")
    // 	}
    assert_eq!(round(h.approx_mean()), round(MAX as f64 / 2.0));
}

// func TestCopy(t *testing.T) {
// 	h1 := hist.New()
// 	for i := 0; i < 1000000; i++ {
// 		if err := h1.RecordValue(float64(i)); err != nil {
// 			t.Fatal(err)
// 		}
// 	}

// 	h2 := h1.Copy()
// 	if !h2.Equals(h1) {
// 		t.Errorf("expected copy: %v to equal original: %v", h2, h1)
// 	}
// }

// func TestFullReset(t *testing.T) {
// 	h1 := hist.New()
// 	for i := 0; i < 1000000; i++ {
// 		if err := h1.RecordValue(float64(i)); err != nil {
// 			t.Fatal(err)
// 		}
// 	}

// 	h1.Reset()
// 	h2 := hist.New()
// 	if !h2.Equals(h1) {
// 		t.Errorf("expected reset value: %v to equal new value: %v", h1, h2)
// 	}
// }

// func TestMerge(t *testing.T) {
#[test]
fn merge() {
    // 	h1 := hist.New()
    // 	h2 := hist.New()
    // 	expect := hist.New()
    let mut h1 = Histogram::default();
    let mut h2 = Histogram::default();
    let mut expect = Histogram::default();

    // 	// record 0-100 values in both h1 and h2.
    // 	for i := 0; i < 100; i++ {
    // 		if err := h1.RecordValues(float64(i), 1); err != nil {
    // 			t.Fatal(err)
    // 		}
    // 		if err := h2.RecordValues(float64(i), 2); err != nil {
    // 			t.Fatal(err)
    // 		}
    // 		if err := expect.RecordValues(float64(i), 3); err != nil {
    // 			t.Fatal(err)
    // 		}
    // 	}
    for i in 0..100 {
        let i = i as f64;
        h1.record_f64s(i, 1).unwrap();
        h2.record_f64s(i, 2).unwrap();
        expect.record_f64s(i, 3).unwrap();
    }

    // 	// record 100-200 values in h1.
    // 	for i := 100; i < 200; i++ {
    // 		if err := h1.RecordValues(float64(i), 1); err != nil {
    // 			t.Fatal(err)
    // 		}
    // 		if err := expect.RecordValues(float64(i), 1); err != nil {
    // 			t.Fatal(err)
    // 		}
    // 	}
    for i in 100..200 {
        let i = i as f64;
        h1.record(i).unwrap();
        expect.record(i).unwrap();
    }
    // 	// record 400-600 values in h2.
    // 	for i := 400; i < 600; i++ {
    // 		if err := h2.RecordValues(float64(i), 1); err != nil {
    // 			t.Fatal(err)
    // 		}
    // 		if err := expect.RecordValues(float64(i), 1); err != nil {
    // 			t.Fatal(err)
    // 		}
    // 	}
    for i in 400..600 {
        let i = i as f64;
        h2.record(i).unwrap();
        expect.record(i).unwrap();
    }

    h1.merge_from(&h2);
    // 	h1.Merge(h2)
    // 	if !h1.Equals(expect) {
    // 		t.Error("Expected histograms to be equivalent")
    // 	}
    assert_eq!(h1, expect);
    // }
}

// func BenchmarkHistogramMerge(b *testing.B) {
// 	b.Run("random", func(b *testing.B) {
// 		rand.New(rand.NewSource(time.Now().UnixNano()))
// 		b.ReportAllocs()
// 		for i := 0; i < b.N; i++ {
// 			h1 := hist.New()
// 			for i := 0; i < 500; i++ {
// 				_ = h1.RecordIntScale(rand.Int63n(1000), 0)
// 			}
// 			h2 := hist.New()
// 			for i := 0; i < 500; i++ {
// 				_ = h2.RecordIntScale(rand.Int63n(1000), 0)
// 			}
// 			h1.Merge(h2)
// 		}
// 	})

// 	b.Run("large insert", func(b *testing.B) {
// 		b.ReportAllocs()
// 		for i := 0; i < b.N; i++ {
// 			h1 := hist.New()
// 			_ = h1.RecordIntScale(1, 0)
// 			_ = h1.RecordIntScale(1000, 0)
// 			h2 := hist.New()
// 			for i := 10; i < 1000; i++ {
// 				_ = h2.RecordIntScale(int64(i), 0)
// 			}
// 			h1.Merge(h2)
// 		}
// 	})
// }
