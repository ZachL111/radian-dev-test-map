use radian_dev_test_map::domain_review::{review_lane, review_score, DomainCase};

#[test]
fn domain_review_case_is_stable() {
    let case = DomainCase { signal: 61, slack: 22, drag: 19, confidence: 69 };
    assert_eq!(review_score(case), 156);
    assert_eq!(review_lane(case), "ship");
}
