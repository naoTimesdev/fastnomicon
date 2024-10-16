use std::{
    hash::{DefaultHasher, Hash, Hasher},
    time::Duration,
};

use pyo3::{basic::CompareOp, exceptions::PyValueError, prelude::*};

/// The time scale of the duration
#[pyclass(eq, eq_int, frozen)]
#[derive(Clone, Copy, PartialEq, Hash)]
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

impl std::fmt::Display for TimeScale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Milliseconds => write!(f, "Milliseconds"),
            Self::Seconds => write!(f, "Seconds"),
            Self::Minutes => write!(f, "Minutes"),
            Self::Hours => write!(f, "Hours"),
            Self::Days => write!(f, "Days"),
            Self::Weeks => write!(f, "Weeks"),
            Self::Months => write!(f, "Months"),
            Self::Years => write!(f, "Years"),
        }
    }
}

#[pymethods]
impl TimeScale {
    fn __str__(&self) -> String {
        format!("TimeScale.{}", self)
    }

    fn __repr__(&self) -> String {
        format!("TimeScale.{}", self)
    }

    /// The value of the [`TimeScale`] in string
    #[getter]
    fn value(&self) -> String {
        format!("{}", self)
    }

    /// The name of the [`TimeScale`], this will return [`TimeScale`] again
    #[getter]
    fn name(&self) -> Self {
        self.clone()
    }
}

/// A simple time tuple that wraps a number in seconds and the time scale.
#[pyclass(frozen)]
pub struct TimeTuple {
    time: u16,
    scale: TimeScale,
}

#[pymethods]
impl TimeTuple {
    /// Create a new instance of [`TimeTuple`]
    #[new]
    #[pyo3(signature = (*, time, scale = TimeScale::Seconds))]
    pub fn new(time: u16, scale: TimeScale) -> Self {
        Self { time, scale }
    }

    /// The time data
    #[getter]
    pub fn time(&self) -> u16 {
        self.time
    }

    /// The scaling used on this
    #[getter]
    fn scale(&self) -> TimeScale {
        self.scale
    }

    /// A duration that represents this time tuple
    pub fn as_duration(&self) -> std::time::Duration {
        match self.scale {
            TimeScale::Milliseconds => std::time::Duration::from_millis(self.time() as u64),
            TimeScale::Seconds => std::time::Duration::from_secs(self.time() as u64),
            TimeScale::Minutes => std::time::Duration::from_secs(self.time() as u64 * 60),
            TimeScale::Hours => std::time::Duration::from_secs(self.time() as u64 * 60 * 60),
            TimeScale::Days => std::time::Duration::from_secs(self.time() as u64 * 60 * 60 * 24),
            TimeScale::Weeks => {
                std::time::Duration::from_secs(self.time() as u64 * 60 * 60 * 24 * 7)
            }
            TimeScale::Months => {
                std::time::Duration::from_secs(self.time() as u64 * 60 * 60 * 24 * 30)
            }
            TimeScale::Years => {
                std::time::Duration::from_secs(self.time() as u64 * 60 * 60 * 24 * 365)
            }
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "<TimeTuple time={} scale={}>",
            self.time(),
            self.scale().__repr__()
        )
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp) -> PyResult<bool> {
        match op {
            CompareOp::Lt => Ok(self.time() < other.time()),
            CompareOp::Le => Ok(self.time() <= other.time()),
            CompareOp::Gt => Ok(self.time() > other.time()),
            CompareOp::Ge => Ok(self.time() >= other.time()),
            CompareOp::Eq => Ok(self.time() == other.time() && self.scale() == other.scale()),
            CompareOp::Ne => Ok(self.time() != other.time() || self.scale() != other.scale()),
        }
    }

    fn __hash__(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.time().hash(&mut hasher);
        self.scale().hash(&mut hasher);
        hasher.finish()
    }
}

impl From<fastnomicon::timestring::TimeScale> for TimeScale {
    fn from(value: fastnomicon::timestring::TimeScale) -> Self {
        match value {
            fastnomicon::timestring::TimeScale::Years => Self::Years,
            fastnomicon::timestring::TimeScale::Months => Self::Months,
            fastnomicon::timestring::TimeScale::Weeks => Self::Weeks,
            fastnomicon::timestring::TimeScale::Days => Self::Days,
            fastnomicon::timestring::TimeScale::Hours => Self::Hours,
            fastnomicon::timestring::TimeScale::Minutes => Self::Minutes,
            fastnomicon::timestring::TimeScale::Seconds => Self::Seconds,
            fastnomicon::timestring::TimeScale::Milliseconds => Self::Milliseconds,
        }
    }
}

#[derive(Debug)]
struct TimeScaleError {
    description: String,
    input: String,
}

impl std::error::Error for TimeScaleError {}

impl std::fmt::Display for TimeScaleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Failed to parse timestring, got {}: {}",
            &self.description, &self.input
        )
    }
}

impl From<TimeScaleError> for PyErr {
    fn from(err: TimeScaleError) -> PyErr {
        PyValueError::new_err(err.to_string())
    }
}

/// Parse a time string format into a [`Duration`]
///
/// If you want to see all the raw time units data, use [`parse_timestring`] instead.
///
/// # Note
/// - We only support number up to `65535` on each time scale.
#[pyfunction]
fn parse_timestring_as_timedelta(input: String) -> Result<Duration, TimeScaleError> {
    match ::fastnomicon::timestring::parse_timestring_as_duration(&input) {
        Ok((_, dur)) => Ok(dur),
        Err(err) => Err(TimeScaleError {
            description: err.to_string(),
            input: input.clone(),
        }),
    }
}

/// Parse a time string format into a a list ot [`TimeTuple`]
///
/// If you want to see it as accumulated number, please use [`parse_timestring_as_duration`] instead.
///
/// # Note
/// - We only support number up to `65535` on each time scale.
#[pyfunction]
fn parse_timestring(input: String) -> Result<Vec<TimeTuple>, TimeScaleError> {
    match ::fastnomicon::timestring::parse_timestring(&input) {
        Ok((_, data)) => {
            let mapped = data
                .into_iter()
                .map(|d| TimeTuple::new(d.time(), d.scale().into()))
                .collect();
            Ok(mapped)
        }
        Err(err) => Err(TimeScaleError {
            description: err.to_string(),
            input: input.clone(),
        }),
    }
}

pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Function
    m.add_function(wrap_pyfunction!(parse_timestring, m)?)?;
    m.add_function(wrap_pyfunction!(parse_timestring_as_timedelta, m)?)?;

    // Classes
    m.add_class::<TimeTuple>()?;
    m.add_class::<TimeScale>()?;

    Ok(())
}
