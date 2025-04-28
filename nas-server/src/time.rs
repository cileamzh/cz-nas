use chrono::{DateTime, Datelike, FixedOffset, NaiveDate, NaiveDateTime, Timelike, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Time {
    pub(crate) year: u32,
    pub(crate) mounth: u32,
    pub(crate) day: u32,
    pub(crate) hour: u32,
    pub(crate) min: u32,
    pub(crate) sec: u32,
}

impl Time {
    const FORMATS: &'static [&'static str] = &[
        "%Y-%m-%d %H:%M:%S",     // 标准格式
        "%Y/%m/%d %H:%M:%S",     // 斜杠格式
        "%d-%m-%Y %H:%M:%S",     // 欧洲格式
        "%Y-%m-%dT%H:%M:%S",     // ISO 8601 (无时区)
        "%Y-%m-%dT%H:%M:%S%.fZ", // ISO 8601 (带毫秒和 Z 结尾)
        "%m/%d/%Y %I:%M %p",     // 12 小时制 (AM/PM)
    ];

    pub fn from_str(time_str: &str) -> Result<Self, String> {
        for format in Self::FORMATS {
            if let Ok(dt) = NaiveDateTime::parse_from_str(time_str, format) {
                return Ok(Self {
                    year: dt.year() as u32,
                    mounth: dt.month(),
                    day: dt.day(),
                    hour: dt.hour(),
                    min: dt.minute(),
                    sec: dt.second(),
                });
            }
        }
        Err("Invalid time format. Supported formats: YYYY-MM-DD HH:MM:SS, YYYY/MM/DD HH:MM:SS, etc.".to_string())
    }
    pub fn is_valid_time(time_str: &str) -> bool {
        Self::FORMATS
            .iter()
            .any(|&format| NaiveDateTime::parse_from_str(time_str, format).is_ok())
    }

    pub fn is_between(&self, start: &Self, end: &Self) -> bool {
        let self_naive = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(self.year as i32, self.mounth, self.day).unwrap(),
            chrono::NaiveTime::from_hms_opt(self.hour, self.min, self.sec).unwrap(),
        );
        let start_naive = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(start.year as i32, start.mounth, start.day).unwrap(),
            chrono::NaiveTime::from_hms_opt(start.hour, start.min, start.sec).unwrap(),
        );
        let end_naive = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(end.year as i32, end.mounth, end.day).unwrap(),
            chrono::NaiveTime::from_hms_opt(end.hour, end.min, end.sec).unwrap(),
        );

        start_naive <= self_naive && self_naive <= end_naive
    }
    pub fn ymd(&self) -> String {
        format!("{:04}-{:02}-{:02}", self.year, self.mounth, self.day)
    }
}

use chrono::NaiveTime;

#[derive(Serialize, Deserialize)]
pub struct TimeHM {
    pub(crate) hour: u32,
    pub(crate) min: u32,
}

impl TimeHM {
    const FORMATS: &'static [&'static str] = &[
        "%H:%M",    // 24 小时制: 14:30
        "%I:%M %p", // 12 小时制: 02:30 PM
    ];

    /// 解析字符串为 TimeHM 结构体
    pub fn from_str(time_str: &str) -> Result<Self, String> {
        for format in Self::FORMATS {
            if let Ok(nt) = NaiveTime::parse_from_str(time_str, format) {
                return Ok(Self {
                    hour: nt.hour(),
                    min: nt.minute(),
                });
            }
        }
        Err("Invalid time format. Supported formats: HH:MM, HH:MM AM/PM".to_string())
    }

    pub fn from_hm(h: u32, m: u32) -> Self {
        return Self { hour: h, min: m };
    }

    /// 判断是否是有效的 "时:分" 时间字符串
    pub fn is_valid_time(time_str: &str) -> bool {
        Self::FORMATS
            .iter()
            .any(|&format| NaiveTime::parse_from_str(time_str, format).is_ok())
    }

    /// 判断当前时间是否在两个 TimeHM 之间
    pub fn is_between(&self, start: &Self, end: &Self) -> bool {
        let self_time = NaiveTime::from_hms_opt(self.hour, self.min, 0).unwrap();
        let start_time = NaiveTime::from_hms_opt(start.hour, start.min, 0).unwrap();
        let end_time = NaiveTime::from_hms_opt(end.hour, end.min, 0).unwrap();

        // 判断时间是否在区间内，考虑循环的情况（如 23:00 - 02:00）
        if start_time <= end_time {
            return self_time >= start_time && self_time <= end_time;
        } else {
            // 跨越午夜的情况
            self_time >= start_time || self_time <= end_time
        }
    }
}

pub fn nowstr() -> String {
    let beijing_offset = FixedOffset::east_opt(8 * 3600).unwrap(); // 8 小时偏移
    let t = Utc::now()
        .with_timezone(&beijing_offset)
        .format("%Y-%m-%d %H:%M:%S")
        .to_string();
    return t;
}

pub fn nowfos() -> DateTime<FixedOffset> {
    let beijing_offset = FixedOffset::east_opt(8 * 3600).unwrap(); // 8 小时偏移
    let t = Utc::now().with_timezone(&beijing_offset);
    return t;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TimeYMD {
    pub(crate) year: u32,
    pub(crate) month: u32,
    pub(crate) day: u32,
}

impl TimeYMD {
    const FORMATS: &'static [&'static str] = &[
        "%Y-%m-%d", // Standard format: 2025-03-30
        "%d-%m-%Y", // European format: 30-03-2025
        "%m/%d/%Y", // US format: 03/30/2025
    ];

    /// Parses a string to create a `TimeYMD` struct.
    pub fn from_str(date_str: &str) -> Result<Self, String> {
        for format in Self::FORMATS {
            if let Ok(nd) = NaiveDate::parse_from_str(date_str, format) {
                return Ok(Self {
                    year: nd.year() as u32,
                    month: nd.month(),
                    day: nd.day(),
                });
            }
        }
        Err(
            "Invalid date format. Supported formats: YYYY-MM-DD, DD-MM-YYYY, MM/DD/YYYY."
                .to_string(),
        )
    }

    /// Check if the given date string matches any of the supported formats.
    pub fn is_valid_date(date_str: &str) -> bool {
        Self::FORMATS
            .iter()
            .any(|&format| NaiveDate::parse_from_str(date_str, format).is_ok())
    }

    /// Returns a formatted string: "YYYY-MM-DD".
    pub fn ymd(&self) -> String {
        format!("{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }

    /// Compares if this date is between two other dates.
    pub fn is_between(&self, start: &Self, end: &Self) -> bool {
        let self_naive = NaiveDate::from_ymd_opt(self.year as i32, self.month, self.day).unwrap();
        let start_naive =
            NaiveDate::from_ymd_opt(start.year as i32, start.month, start.day).unwrap();
        let end_naive = NaiveDate::from_ymd_opt(end.year as i32, end.month, end.day).unwrap();

        start_naive <= self_naive && self_naive <= end_naive
    }
}
