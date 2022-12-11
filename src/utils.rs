use crate::prelude::*;

// 2022-03-04 05:06:07 hardcoded time for tests
#[allow(dead_code)]
const HARDCODED_TEST_TIME: i64 = 1646370367;

pub struct Utils;
impl Utils {
    pub fn i16ify(array: &[u8], offset: usize) -> i16 {
        i16::from_le_bytes([array[offset], array[offset + 1]])
    }
    pub fn u16ify(array: &[u8], offset: usize) -> u16 {
        u16::from_le_bytes([array[offset], array[offset + 1]])
    }

    pub fn opt_le_i16_div10(input: &[u8]) -> nom::IResult<&[u8], Option<f64>> {
        let (input, num) = nom::number::complete::le_i16(input)?;
        Ok((input, Some(num as f64 / 10.0)))
    }
    pub fn le_i16_div10(input: &[u8]) -> nom::IResult<&[u8], f64> {
        let (input, num) = nom::number::complete::le_i16(input)?;
        Ok((input, num as f64 / 10.0))
    }
    pub fn opt_le_i16_div100(input: &[u8]) -> nom::IResult<&[u8], Option<f64>> {
        let (input, num) = nom::number::complete::le_i16(input)?;
        Ok((input, Some(num as f64 / 100.0)))
    }
    pub fn le_i16_div100(input: &[u8]) -> nom::IResult<&[u8], f64> {
        let (input, num) = nom::number::complete::le_i16(input)?;
        Ok((input, num as f64 / 100.0))
    }
    pub fn le_i16_div1000(input: &[u8]) -> nom::IResult<&[u8], f64> {
        let (input, num) = nom::number::complete::le_i16(input)?;
        Ok((input, num as f64 / 1000.0))
    }

    pub fn le_u32_div10(input: &[u8]) -> nom::IResult<&[u8], f64> {
        let (input, num) = nom::number::complete::le_u32(input)?;
        Ok((input, num as f64 / 10.0))
    }

    pub fn current_time_for_nom(input: &[u8]) -> nom::IResult<&[u8], UnixTime> {
        Ok((input, UnixTime::now()))
    }

    #[cfg(not(feature = "mocks"))]
    pub fn utc() -> chrono::DateTime<chrono::Utc> {
        chrono::Utc::now()
    }

    #[cfg(feature = "mocks")]
    pub fn utc() -> chrono::DateTime<chrono::Utc> {
        use chrono::TimeZone;
        // [22, 3, 4, 5, 6, 7] hardcoded for tests
        chrono::Utc.timestamp(HARDCODED_TEST_TIME, 0)
    }

    #[cfg(not(feature = "mocks"))]
    pub fn localtime() -> chrono::DateTime<chrono::Local> {
        chrono::Local::now()
    }

    #[cfg(feature = "mocks")]
    pub fn localtime() -> chrono::DateTime<chrono::Local> {
        use chrono::TimeZone;
        // [22, 3, 4, 5, 6, 7] hardcoded for tests
        chrono::Local.timestamp(HARDCODED_TEST_TIME, 0)
    }
}
