use radian_dev_test_map::{classify, score, Signal};
#[test]
fn fixture_decisions() {
    let signal = Signal { demand: 77, capacity: 98, latency: 11, risk: 24, weight: 6 };
    assert_eq!(score(signal), 104);
    assert_eq!(classify(signal), "review");
    let signal = Signal { demand: 79, capacity: 85, latency: 12, risk: 24, weight: 12 };
    assert_eq!(score(signal), 111);
    assert_eq!(classify(signal), "review");
    let signal = Signal { demand: 66, capacity: 86, latency: 10, risk: 20, weight: 6 };
    assert_eq!(score(signal), 96);
    assert_eq!(classify(signal), "review");
}
