use std::time::Duration;

use fastnomicon::timestring::{TimeScale, TimeTuple};

#[test]
fn test_standard() {
    let (_, time) = fastnomicon::timestring::parse_timestring("1h30m").unwrap();
    assert_eq!(
        time,
        vec![
            TimeTuple::make(1, TimeScale::Hours),
            TimeTuple::make(30, TimeScale::Minutes)
        ]
    );

    let (_, time) = fastnomicon::timestring::parse_timestring("30m 1 hours").unwrap();
    assert_eq!(
        time,
        vec![
            TimeTuple::make(30, TimeScale::Minutes),
            TimeTuple::make(1, TimeScale::Hours)
        ]
    );

    let (_, time) = fastnomicon::timestring::parse_timestring("50 detik").unwrap();
    assert_eq!(time, vec![TimeTuple::make(50, TimeScale::Seconds)]);

    let (_, time) = fastnomicon::timestring::parse_timestring("50s500ms").unwrap();
    assert_eq!(
        time,
        vec![
            TimeTuple::make(50, TimeScale::Seconds),
            TimeTuple::make(500, TimeScale::Milliseconds)
        ]
    );

    let (_, time) = fastnomicon::timestring::parse_timestring("1 jam 30 menit 500 millis").unwrap();
    assert_eq!(
        time,
        vec![
            TimeTuple::make(1, TimeScale::Hours),
            TimeTuple::make(30, TimeScale::Minutes),
            TimeTuple::make(500, TimeScale::Milliseconds),
        ]
    );
}

#[test]
fn test_standard_duration() {
    let (_, time) = fastnomicon::timestring::parse_timestring_as_duration("1h30m").unwrap();
    assert_eq!(time, Duration::from_secs(5400));
    let (_, time) = fastnomicon::timestring::parse_timestring_as_duration("30m 1 hours").unwrap();
    assert_eq!(time, Duration::from_secs(5400));
    let (_, time) = fastnomicon::timestring::parse_timestring_as_duration("50 detik").unwrap();
    assert_eq!(time, Duration::from_secs(50));

    let (_, time) = fastnomicon::timestring::parse_timestring_as_duration("50s500ms").unwrap();
    assert_eq!(time, Duration::from_secs(50) + Duration::from_millis(500));
}

#[test]
fn test_fails() {
    fastnomicon::timestring::parse_timestring("1h30xxxx").unwrap_err();
}
