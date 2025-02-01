use smallvec::SmallVec;

pub trait Agent {
    fn action(&mut self);
}

pub struct SliceVectorTimer<'a, const M: usize> {
    timestamp: usize,
    timers: [Vec<&'a mut dyn Agent>; M],
}

impl<'a, const M: usize> SliceVectorTimer<'a, M> {
    pub fn new() -> Self {
        Self::with_timestamp(0)
    }

    pub fn with_timestamp(timestamp: usize) -> Self {
        Self {
            timestamp,
            timers: [const{ Vec::new() }; M],
        }
    }

    pub fn start_timer(&mut self, expiry: usize, agent: &'a mut dyn Agent) -> usize {
        let index = self.timestamp + expiry % M;

        self.timers[index].push(agent);

        index
    }

    pub fn tick(&mut self) {
        self.timestamp = (self.timestamp + 1) % M;

        self.timers[self.timestamp].iter_mut().for_each(|agent| agent.action());

        self.timers[self.timestamp].clear();
    }
}

pub struct VectorVectorTimer<'a> {
    timestamp: usize,
    timers: Vec<Vec<&'a mut dyn Agent>>,
}

impl<'a> VectorVectorTimer<'a> {
    pub fn new() -> Self {
        Self::with_timestamp(0)
    }

    pub fn with_timestamp(timestamp: usize) -> Self {
        Self {
            timestamp,
            timers: Vec::new(),
        }
    }

    pub fn start_timer(&mut self, expiry: usize, agent: &'a mut dyn Agent) -> usize {
        let index = self.timestamp + expiry;

        if index >= self.timers.len() {
            self.timers.resize_with(index + 1, Vec::new);
        }

        self.timers[index].push(agent);

        index
    }

    pub fn tick(&mut self) {
        self.timestamp += 1;

        self.timers.iter_mut().for_each(|agents| agents.iter_mut().for_each(|agent| agent.action()));

        self.timers.clear();
    }
}

pub struct SliceSmallVectorTimer<'a, const M: usize, const N: usize> {
    timestamp: usize,
    timers: [SmallVec<[&'a mut dyn Agent; N]>; M],
}

impl<'a, const M: usize, const N: usize> SliceSmallVectorTimer<'a, M, N> {
    pub fn new() -> Self {
        Self::with_timestamp(0)
    }

    pub fn with_timestamp(timestamp: usize) -> Self {
        Self {
            timestamp,
            timers: [const{ SmallVec::new_const() }; M],
        }
    }

    pub fn start_timer(&mut self, expiry: usize, agent: &'a mut dyn Agent) -> usize {
        let index = self.timestamp + expiry % M;

        self.timers[index].push(agent);

        index
    }

    pub fn tick(&mut self) {
        self.timestamp = (self.timestamp + 1) % M;

        self.timers[self.timestamp].iter_mut().for_each(|agent| agent.action());

        self.timers[self.timestamp].clear();
    }
}
