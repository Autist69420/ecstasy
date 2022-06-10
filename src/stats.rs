#[derive(Clone, Debug, Default)]
pub struct StatsContainer {
    pub ok: u64,
    pub failed: u64,
    pub skipped: u64,
    pub inherited: u64,
    pub size: f64,
}

impl StatsContainer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn describe(&self) -> String {
        format!(
            "Ok: {} | Err: {} | Skp: {} | Cpd: {}",
            &self.ok, &self.failed, &self.skipped, &self.inherited
        )
    }

    pub fn count(&self) -> u64 {
        self.ok + self.failed + self.skipped + self.inherited
    }

    pub fn add_ok(&mut self) -> &'static str {
        self.ok += 1;
        "\x1b[92mDOWNLOADED\x1b[0m"
    }

    pub fn add_failed(&mut self) -> &'static str {
        self.failed += 1;
        "\x1b[91mFAILED\x1b[0m"
    }

    pub fn add_skipped(&mut self) -> &'static str {
        self.skipped += 1;
        "\x1b[36mSKIPPED\x1b[0m"
    }

    pub fn add_inherited(&mut self) -> &'static str {
        self.inherited += 1;
        "\x1b[95mINHERITED\x1b[0m"
    }

    pub fn add_size(&mut self, amount: f64) {
        self.size += amount;
    }
}
