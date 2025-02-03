use super::time::Time;

pub struct Action {
    inner: u64,
}

impl Action {
    pub const fn new(id: usize, time: Time) -> Self {
        // why allow a `usize` despite only allowing 16 bit occupancy?
        //
        // bc `usize` is the type for indexing arrays and the `id` is meant to
        // be used as an index, though it seems unreasonable to have more than
        // 65535 distinct actions for a given timer
        assert!(id <= 0xFFFF, "`id` must occupy 16 bits or less (<= 65535)");

        let inner = time.inner() | ((id as u64) << 48);

        Self { inner }
    }

    pub const fn id(&self) -> usize {
        ((self.inner >> 48) & 0xFFFF) as usize
    }

    pub const fn time(&self) -> Time {
        Time::from_inner(self.inner & 0xFFFFFFFFFFFF)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_action() {
        let time = Time::from_inner(123456789);
        let action = Action::new(42, time);

        assert_eq!(action.id(), 42);
        assert_eq!(action.time().inner(), 123456789);
    }
}
