pub struct SimResult {
    pub x: usize,
    pub compavg: f64,
    pub comps: Vec<usize>,
    pub swapavg: f64,
    pub swaps: Vec<usize>,
}

impl SimResult {
    pub fn new(x: usize) -> Self {
        return SimResult {
            x,
            compavg: 0.0,
            comps: Vec::new(),
            swapavg: 0.0,
            swaps: Vec::new(),
        };
    }

    pub fn add(&mut self, comps: usize, swaps: usize) {
        self.compavg += comps as f64;
        self.comps.push(comps);
        self.swapavg += swaps as f64;
        self.swaps.push(swaps);
    }

    pub fn average(&mut self, times: usize) {
        self.compavg /= times as f64;
        self.swapavg /= times as f64;
    }
}