//! Time string parser with nom

use std::time::Duration;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, multispace0, space0},
    combinator::{map_res, recognize},
    multi::many1,
    sequence::{delimited, pair},
    IResult,
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
    map_res(digit1, str::parse)(input)
}

fn parse_6char_or_more_unit(input: &str) -> IResult<&str, &str> {
    alt((
        recognize(tag("miliseconds")),
        recognize(tag("milisecond")),
        recognize(tag("minutes")),
        recognize(tag("seconds")),
        recognize(tag("milisec")),
        recognize(tag("minute")),
        recognize(tag("second")),
        recognize(tag("months")),
        recognize(tag("minggu")),
        recognize(tag("millis")),
    ))(input)
}

fn parse_5char_unit(input: &str) -> IResult<&str, &str> {
    alt((
        recognize(tag("years")),
        recognize(tag("tahun")),
        recognize(tag("month")),
        recognize(tag("bulan")),
        recognize(tag("hours")),
        recognize(tag("menit")),
        recognize(tag("detik")),
        recognize(tag("milli")),
        recognize(tag("msecs")),
    ))(input)
}

fn parse_4char_unit(input: &str) -> IResult<&str, &str> {
    alt((
        recognize(tag("year")),
        recognize(tag("week")),
        recognize(tag("days")),
        recognize(tag("hari")),
        recognize(tag("hour")),
        recognize(tag("mins")),
        recognize(tag("secs")),
        recognize(tag("mili")),
        recognize(tag("msec")),
    ))(input)
}

fn parse_3char_unit(input: &str) -> IResult<&str, &str> {
    alt((
        recognize(tag("thn")),
        recognize(tag("bln")),
        recognize(tag("mng")),
        recognize(tag("day")),
        recognize(tag("hrs")),
        recognize(tag("jam")),
        recognize(tag("min")),
        recognize(tag("mnt")),
        recognize(tag("sec")),
        recognize(tag("dtk")),
        recognize(tag("mil")),
    ))(input)
}

fn parse_2char_unit(input: &str) -> IResult<&str, &str> {
    alt((
        recognize(tag("mo")), // Month
        recognize(tag("wk")), // Week
        recognize(tag("hr")), // Hour
        recognize(tag("ms")), // Milisecond
    ))(input)
}

fn parse_1char_unit(input: &str) -> IResult<&str, &str> {
    alt((
        recognize(char('y')), // Year
        recognize(char('t')), // Year
        recognize(char('M')), // Month
        recognize(char('b')), // Month
        recognize(char('w')), // Week
        recognize(char('d')), // Day
        recognize(char('h')), // Hour
        recognize(char('j')), // Hour
        recognize(char('m')), // Minute
        recognize(char('s')), // Second
    ))(input)
}

fn parse_unit(input: &str) -> IResult<&str, &str> {
    alt((
        parse_6char_or_more_unit,
        parse_5char_unit,
        parse_4char_unit,
        parse_3char_unit,
        parse_2char_unit,
        parse_1char_unit,
    ))(input)
}

fn parse_time_component(input: &str) -> IResult<&str, TimeTuple> {
    let (rest, (number, unit)) =
        pair(parse_number, delimited(space0, parse_unit, multispace0))(input)?;

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
            return Err(nom::Err::Error(nom::error::make_error(
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
    let (input, time_units) = many1(parse_time_component)(input)?;

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
    many1(parse_time_component)(input)
}
