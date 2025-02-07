//! Time string parser with nom

use std::time::Duration;

use nom::{
    character::complete::{alpha1, digit1, multispace0},
    combinator::map_res,
    multi::many1,
    sequence::{delimited, pair},
    IResult, Parser,
};

/// The time scale of the duration
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TimeScale {
    /// Milliseconds timescale, 1/1000 of a second
    Milliseconds,
    /// Seconds timescale, a.k.a we will have no changes
    Seconds,
    /// Minutes timescale, multiplied by 60 seconds
    Minutes,
    /// Hours timescale, multiplied by 3600 seconds
    Hours,
    /// Days timescale, assumed to be 24 hours long
    Days,
    /// Weeks timescale, assumed to be 7 days long
    Weeks,
    /// Months timescale, assumed to be 30 days long
    Months,
    /// Years timescale, asssumed to be 365 days long
    Years,
}

/// A simple time tuple that wraps a number in seconds and the time scale.
#[derive(Debug, Clone, Copy)]
pub struct TimeTuple(u16, TimeScale);

impl TimeTuple {
    /// Create a new instance of [`TimeTuple`]
    pub fn make(time: u16, scale: TimeScale) -> Self {
        TimeTuple(time, scale)
    }

    /// The time data
    pub fn time(&self) -> u16 {
        self.0
    }

    /// The scaling used on this
    pub fn scale(&self) -> TimeScale {
        self.1
    }

    /// A duration that represents this time tuple
    pub fn as_duration(&self) -> std::time::Duration {
        match self.1 {
            TimeScale::Milliseconds => std::time::Duration::from_millis(self.0 as u64),
            TimeScale::Seconds => std::time::Duration::from_secs(self.0 as u64),
            TimeScale::Minutes => std::time::Duration::from_secs(self.0 as u64 * 60),
            TimeScale::Hours => std::time::Duration::from_secs(self.0 as u64 * 60 * 60),
            TimeScale::Days => std::time::Duration::from_secs(self.0 as u64 * 60 * 60 * 24),
            TimeScale::Weeks => std::time::Duration::from_secs(self.0 as u64 * 60 * 60 * 24 * 7),
            TimeScale::Months => std::time::Duration::from_secs(self.0 as u64 * 60 * 60 * 24 * 30),
            TimeScale::Years => std::time::Duration::from_secs(self.0 as u64 * 60 * 60 * 24 * 365),
        }
    }
}

impl PartialEq for TimeTuple {
    fn eq(&self, other: &Self) -> bool {
        self.time() == other.time() && self.scale() == other.scale()
    }
}

fn parse_number(input: &str) -> IResult<&str, u16> {
    map_res(digit1, str::parse).parse(input)
}

fn parse_unit(input: &str) -> IResult<&str, &str> {
    delimited(multispace0, alpha1, multispace0).parse(input)
}

fn parse_time_component(input: &str) -> IResult<&str, TimeTuple> {
    let (rest, (number, unit)) = pair(parse_number, parse_unit).parse(input)?;

    let time_unit = match unit {
        "y" | "year" | "years" | "t" | "thn" | "tahun" => TimeScale::Years,
        "M" | "mo" | "month" | "months" | "b" | "bln" | "bulan" => TimeScale::Months,
        "w" | "wk" | "week" | "weeks" | "mng" | "minggu" => TimeScale::Weeks,
        "d" | "day" | "days" | "hari" => TimeScale::Days,
        "h" | "hr" | "hrs" | "hour" | "hours" | "j" | "jam" => TimeScale::Hours,
        "m" | "min" | "mins" | "minute" | "minutes" | "mnt" | "menit" => TimeScale::Minutes,
        "s" | "sec" | "secs" | "second" | "seconds" | "dtk" | "detik" => TimeScale::Seconds,
        "ms" | "mil" | "mill" | "millis" | "milli" | "msec" | "msecs" | "milisec"
        | "miliseconds" | "milisecond" => TimeScale::Milliseconds,
        _ => {
            return Err(nom::Err::Failure(nom::error::make_error(
                unit,
                nom::error::ErrorKind::Tag,
            )));
        }
    };

    Ok((rest, TimeTuple(number, time_unit)))
}

/// Parse a time string format into a [`Duration`]
///
/// If you want to see all the raw time units data, use [`parse_timestring`] instead.
///
/// # Note
/// - We only support number up to `65535` on each time scale.
pub fn parse_timestring_as_duration(input: &str) -> IResult<&str, Duration> {
    let (input, time_units) = parse_timestring(input)?;

    // Check if all None
    let duration = time_units
        .into_iter()
        .fold(Duration::default(), |acc, unit| {
            let dur = unit.as_duration();
            acc.saturating_add(dur)
        });

    Ok((input, duration))
}

/// Parse a time string format into a a list ot [`TimeTuple`]
///
/// If you want to see it as accumulated number, please use [`parse_timestring_as_duration`] instead.
///
/// # Note
/// - We only support number up to `65535` on each time scale.
pub fn parse_timestring(input: &str) -> IResult<&str, Vec<TimeTuple>> {
    many1(parse_time_component).parse(input)
}
