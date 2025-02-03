#[derive(Clone, Copy)]
pub struct Time {
    inner: u64,
}

impl Time {
    pub const fn new(milliseconds: u8, seconds: u8, minutes: u8, hours: u8, days: u16) -> Self {
        let inner = (milliseconds as u64)
            | ((seconds as u64) << 8)
            | ((minutes as u64) << 16)
            | ((hours as u64) << 24)
            | ((days as u64) << 32);

        Self { inner }
    }

    pub const fn increment(&mut self) {
        let milliseconds = self.milliseconds() + 1;
        let seconds = self.seconds() + (milliseconds == 100) as u8;
        let minutes = self.minutes() + (seconds == 60) as u8;
        let hours = self.hours() + (minutes == 60) as u8;
        let days = u16::wrapping_add(self.days(), (hours == 24) as u16);

        self.inner = Self::new(
            milliseconds % 100,
            seconds % 60,
            minutes % 60,
            hours % 24,
            days,
        )
        .inner;
    }

    pub const fn increment_checked(&mut self) -> (bool, bool, bool, bool, bool) {
        let milliseconds = self.milliseconds() + 1;
        let roll_seconds = milliseconds == 100;

        let seconds = self.seconds() + roll_seconds as u8;
        let roll_minutes = seconds == 60;

        let minutes = self.minutes() + roll_minutes as u8;
        let roll_hours = minutes == 60;

        let hours = self.hours() + roll_hours as u8;
        let roll_days = hours == 24;

        let days = self.days();
        let roll_clock = roll_days && days == u16::MAX;
        let days = days.wrapping_add(roll_days as u16);

        self.inner = Self::new(
            milliseconds % 100,
            seconds % 60,
            minutes % 60,
            hours % 60,
            days,
        )
        .inner;

        (roll_seconds, roll_minutes, roll_hours, roll_days, roll_clock)
    }

    pub const fn from_inner(inner: u64) -> Self {
        Self { inner }
    }

    pub const fn inner(&self) -> u64 {
        self.inner
    }

    pub const fn milliseconds(&self) -> u8 {
        (self.inner & 0xFF) as u8
    }

    pub const fn seconds(&self) -> u8 {
        ((self.inner >> 8) & 0xFF) as u8
    }

    pub const fn minutes(&self) -> u8 {
        ((self.inner >> 16) & 0xFF) as u8
    }

    pub const fn hours(&self) -> u8 {
        ((self.inner >> 24) & 0xFF) as u8
    }

    pub const fn days(&self) -> u16 {
        ((self.inner >> 32) & 0xFFFF) as u16
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let time = Time::new(1, 2, 3, 4, 5);

        assert_eq!(time.milliseconds(), 1);
        assert_eq!(time.seconds(), 2);
        assert_eq!(time.minutes(), 3);
        assert_eq!(time.hours(), 4);
        assert_eq!(time.days(), 5);
    }

    #[test]
    fn test_increment() {
        let mut time = Time::new(0, 0, 0, 0, 0);

        assert_eq!(time.milliseconds(), 0);
        assert_eq!(time.seconds(), 0);
        assert_eq!(time.minutes(), 0);
        assert_eq!(time.hours(), 0);
        assert_eq!(time.days(), 0);

        time.increment();

        assert_eq!(time.milliseconds(), 1);
        assert_eq!(time.seconds(), 0);
        assert_eq!(time.minutes(), 0);
        assert_eq!(time.hours(), 0);
        assert_eq!(time.days(), 0);
    }

    #[test]
    fn test_increment_overflow() {
        let mut time = Time::new(99, 59, 59, 23, 0xFFFF);

        time.increment();

        assert_eq!(time.milliseconds(), 0);
        assert_eq!(time.seconds(), 0);
        assert_eq!(time.minutes(), 0);
        assert_eq!(time.hours(), 0);
        assert_eq!(time.days(), 0);
    }

    #[test]
    fn test_increment_checked_overflow() {
        let mut time = Time::new(99, 59, 59, 23, 0xFFFF);

        let (roll_seconds, roll_minutes, roll_hours, roll_days, halt) = time.increment_checked();

        assert!(roll_seconds);
        assert!(roll_minutes);
        assert!(roll_hours);
        assert!(roll_days);
        assert!(halt);
    }

    #[test]
    fn test_increment_checked_no_overflow() {
        let mut time = Time::new(0, 0, 0, 0, 0);

        let (roll_seconds, roll_minutes, roll_hours, roll_days, halt) = time.increment_checked();

        assert!(!roll_seconds);
        assert!(!roll_minutes);
        assert!(!roll_hours);
        assert!(!roll_days);
        assert!(!halt);

        assert_eq!(time.milliseconds(), 1);
        assert_eq!(time.seconds(), 0);
        assert_eq!(time.minutes(), 0);
        assert_eq!(time.hours(), 0);
        assert_eq!(time.days(), 0);
    }
}
