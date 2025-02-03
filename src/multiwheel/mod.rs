pub mod action;
pub mod time;

use action::Action;
use time::Time;

pub struct Timer<const D: usize> {
    time: Time,
    milliseconds: [Vec<Action>; 100],
    seconds: [Vec<Action>; 60],
    minutes: [Vec<Action>; 60],
    hours: [Vec<Action>; 24],
    days: [Vec<Action>; D],
}

impl<const D: usize> Timer<D> {
    pub const fn new() -> Self {
        Self::with_time(Time::from_inner(0))
    }

    pub const fn with_time(time: Time) -> Self {
        assert!(
            D <= u16::MAX as usize,
            "<D> must occupy 16 bits or less (<= 65535)"
        );

        Self {
            time,
            milliseconds: [const { Vec::new() }; 100],
            seconds: [const { Vec::new() }; 60],
            minutes: [const { Vec::new() }; 60],
            hours: [const { Vec::new() }; 24],
            days: [const { Vec::new() }; D],
        }
    }

    pub fn start_timer(&mut self, id: usize, time: Time) {
        let action = Action::new(id, time);

        let milliseconds = time.milliseconds();
        let seconds = time.seconds();
        let minutes = time.minutes();
        let hours = time.hours();
        let days = time.days();

        if days > 0 {
            self.days[days as usize].push(action);
        } else if hours > 0 {
            self.hours[hours as usize].push(action);
        } else if minutes > 0 {
            self.minutes[minutes as usize].push(action);
        } else if seconds > 0 {
            self.seconds[seconds as usize].push(action);
        } else if milliseconds > 0 {
            self.milliseconds[milliseconds as usize].push(action);
        }
    }

    pub fn tick(&mut self) -> Option<Vec<usize>> {
        let (roll_seconds, roll_minutes, roll_hours, roll_days, halt) =
            self.time.increment_checked();

        if halt {
            return None;
        }

        if roll_days {
            self.days[self.time.days() as usize]
                .drain(..)
                .for_each(|action| self.hours[action.time().hours() as usize].push(action));
        }

        if roll_hours {
            self.hours[self.time.hours() as usize]
                .drain(..)
                .for_each(|action| self.minutes[action.time().minutes() as usize].push(action));
        }

        if roll_minutes {
            self.minutes[self.time.minutes() as usize]
                .drain(..)
                .for_each(|action| self.seconds[action.time().seconds() as usize].push(action));
        }

        if roll_seconds {
            self.seconds[self.time.seconds() as usize]
                .drain(..)
                .for_each(|action| {
                    self.milliseconds[action.time().milliseconds() as usize].push(action)
                });
        }

        Some(
            self.milliseconds[self.time.milliseconds() as usize]
                .drain(..)
                .map(|action| action.id())
                .collect(),
        )
    }
}

impl<const D: usize> Iterator for Timer<D> {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        self.tick()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vibe_check() {
        let mut timer = Timer::<1>::new();

        timer.start_timer(1, Time::new(1, 0, 0, 0, 0));

        assert_eq!(timer.tick(), Some(vec![1]));
    }

    #[test]
    fn test_vibe_check_2() {
        let mut timer = Timer::<1>::new();

        timer.start_timer(1, Time::new(2, 0, 0, 0, 0));

        timer.tick();

        assert_eq!(timer.tick(), Some(vec![1]));
    }

    #[test]
    fn test_stuff() {
        let mut timer = Timer::<1>::new();

        timer.start_timer(1, Time::new(1, 0, 0, 0, 0));
        timer.start_timer(2, Time::new(2, 0, 0, 0, 0));
        timer.start_timer(3, Time::new(3, 0, 0, 0, 0));
        timer.start_timer(4, Time::new(4, 0, 0, 0, 0));
        timer.start_timer(5, Time::new(5, 0, 0, 0, 0));
        timer.start_timer(6, Time::new(6, 0, 0, 0, 0));
        timer.start_timer(7, Time::new(7, 0, 0, 0, 0));
        timer.start_timer(8, Time::new(7, 0, 0, 0, 0));

        assert_eq!(timer.tick(), Some(vec![1]));
        assert_eq!(timer.tick(), Some(vec![2]));
        assert_eq!(timer.tick(), Some(vec![3]));
        assert_eq!(timer.tick(), Some(vec![4]));
        assert_eq!(timer.tick(), Some(vec![5]));
        assert_eq!(timer.tick(), Some(vec![6]));
        assert_eq!(timer.tick(), Some(vec![7, 8]));
    }
}
