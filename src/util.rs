use winsafe as w;

pub struct Timer(i64);

impl Timer {
	pub fn start() -> Self {
		Self(w::QueryPerformanceCounter().unwrap())
	}

	pub fn now_ms(&self) -> f64 {
		let freq = w::QueryPerformanceFrequency().unwrap();
		let t1 = w::QueryPerformanceCounter().unwrap();
		((t1 - self.0) as f64 / freq as f64) * 1000.0
	}
}
