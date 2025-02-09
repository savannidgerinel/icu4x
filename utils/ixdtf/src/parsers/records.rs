// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! The records that `ixdtf`'s contain the resulting values of parsing.

/// An `IxdtfParseRecord` is an intermediary record returned by `IxdtfParser`.
#[non_exhaustive]
#[derive(Default, Debug, PartialEq)]
pub struct IxdtfParseRecord<'a> {
    /// Parsed `DateRecord`
    pub date: Option<DateRecord>,
    /// Parsed `TimeRecord`
    pub time: Option<TimeRecord>,
    /// Parsed UtcOffset
    pub offset: Option<UTCOffsetRecord>,
    /// Parsed `TimeZone` annotation with critical flag and data (UTCOffset | IANA name)
    pub tz: Option<TimeZoneAnnotation<'a>>,
    /// The parsed calendar value.
    pub calendar: Option<&'a str>,
}

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
/// A record of an annotation.
pub struct Annotation<'a> {
    /// Whether this annotation is flagged as critical
    pub critical: bool,
    /// The parsed key value of the annotation
    pub key: &'a str,
    /// The parsed value of the annotation
    pub value: &'a str,
}

#[allow(clippy::exhaustive_structs)] // DateRecord only allows for a year, month, and day value.
#[derive(Default, Debug, Clone, Copy, PartialEq)]
/// The record of a parsed date.
pub struct DateRecord {
    /// Date Year
    pub year: i32,
    /// Date Month
    pub month: u8,
    /// Date Day
    pub day: u8,
}

/// Parsed Time info
#[allow(clippy::exhaustive_structs)] // TimeRecord only allows for a hour, minute, second, and sub-second value.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct TimeRecord {
    /// An hour
    pub hour: u8,
    /// A minute value
    pub minute: u8,
    /// A second value.
    pub second: u8,
    /// A nanosecond value representing all sub-second components.
    pub nanosecond: u32,
}

/// A `TimeZoneAnnotation` that represents a parsed `TimeZoneRecord` and its critical flag.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub struct TimeZoneAnnotation<'a> {
    /// Critical flag for the `TimeZoneAnnotation`.
    pub critical: bool,
    /// The parsed `TimeZoneRecord` for the annotation.
    pub tz: TimeZoneRecord<'a>,
}

/// Parsed `TimeZone` data, which can be either a UTC Offset value or IANA Time Zone Name value.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub enum TimeZoneRecord<'a> {
    /// TimeZoneIANAName
    Name(&'a str),
    /// TimeZoneOffset
    Offset(UTCOffsetRecord),
}

/// The parsed sign value, representing whether its struct is positive or negative.
#[repr(i8)]
#[allow(clippy::exhaustive_enums)] // Sign can only be positive or negative.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Sign {
    /// A negative value sign, representable as either -1 or false.
    Negative = -1,
    /// A positive value sign, representable as either 1 or true.
    Positive = 1,
}

impl From<bool> for Sign {
    fn from(value: bool) -> Self {
        match value {
            true => Self::Positive,
            false => Self::Negative,
        }
    }
}

/// A full precision `UtcOffsetRecord`
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UTCOffsetRecord {
    /// The `Sign` value of the `UtcOffsetRecord`
    pub sign: Sign,
    /// The hour value of the `UtcOffsetRecord`
    pub hour: u8,
    /// The minute value of the `UtcOffsetRecord`.
    pub minute: u8,
    /// The second value of the `UtcOffsetRecord`.
    pub second: u8,
    /// Any nanosecond value of the `UTCOffsetRecord`
    pub nanosecond: u32,
}

/// The resulting record of a `Duration` parse.
#[non_exhaustive]
#[cfg(feature = "duration")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DurationParseRecord {
    /// Duration Sign
    pub sign: Sign,
    /// The `years` value.
    pub years: u32,
    /// The `months` value.
    pub months: u32,
    /// The `weeks` value.
    pub weeks: u32,
    /// The `days` value.
    pub days: u32,
    /// The `hours` value.
    pub hours: u32,
    /// The `minutes` value.
    pub minutes: u32,
    /// The `seconds` value.
    pub seconds: u32,
    /// Any fraction part of a duration.
    pub fraction: Option<DurationFraction>,
}

/// An enum representing the fraction part of a duration.
#[non_exhaustive]
#[cfg(feature = "duration")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DurationFraction {
    /// The fraction value applied to an hour.
    Hours(u64),
    /// The fraction value applied to the minutes field.
    Minutes(u64),
    /// The fraction value applied to the seconds field.
    Seconds(u32),
}

/// A `DateDuration` Parse Node.
#[cfg(feature = "duration")]
#[derive(Default, Debug, Clone, Copy)]
pub(crate) struct DateDurationRecord {
    /// Years value.
    pub(crate) years: u32,
    /// Months value.
    pub(crate) months: u32,
    /// Weeks value.
    pub(crate) weeks: u32,
    /// Days value.
    pub(crate) days: u32,
}

/// A `TimeDuration` Parse Node
#[cfg(feature = "duration")]
#[derive(Default, Debug, Clone, Copy)]
pub(crate) struct TimeDurationRecord {
    /// Hours value.
    pub(crate) hours: u32,
    /// Minutes value.
    pub(crate) minutes: u32,
    /// Seconds value.
    pub(crate) seconds: u32,
    /// Any parsed fraction value.
    pub(crate) fraction: Option<DurationFraction>,
}
