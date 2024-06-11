use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchTimerStatus {
    Stopped,
    Running,
}

#[derive(Debug, Clone, Copy)]
pub enum SearchDuration {
    Finite(u128),
    Infinite,
}

pub struct SearchTimer {
    pub start_time: Option<Instant>,
    pub allowed_duration: SearchDuration,
    pub status: SearchTimerStatus,
}

impl Default for SearchTimer {
    fn default() -> Self {
        Self {
            start_time: None,
            allowed_duration: SearchDuration::Infinite,
            status: SearchTimerStatus::Stopped,
        }
    }
}

impl SearchTimer {
    pub fn initialize(
        &mut self,
        time_remaining: Option<u128>,
        increment: u128,
        moves_to_go: Option<u64>,
    ) {
        let moves_to_go = moves_to_go.unwrap_or(30);

        let duration = match time_remaining {
            Some(time) => SearchDuration::Finite(
                (time as f64 / moves_to_go as f64).round() as u128 + increment - 50,
            ),
            None => SearchDuration::Infinite,
        };

        self.start_time = None;
        self.allowed_duration = duration;
    }

    pub fn start(&mut self) {
        self.status = SearchTimerStatus::Running;
        self.start_time = Some(Instant::now());
    }

    pub fn check(&mut self) {
        let is_time_up = match self.allowed_duration {
            SearchDuration::Finite(duration) => self.elapsed_ms() >= duration,
            SearchDuration::Infinite => false,
        };

        if is_time_up {
            self.status = SearchTimerStatus::Stopped;
        }
    }

    pub fn is_stopped(&self) -> bool {
        self.status == SearchTimerStatus::Stopped
    }

    fn elapsed_ms(&self) -> u128 {
        match self.start_time {
            Some(time) => time.elapsed().as_millis(),
            None => 0,
        }
    }
}
