use crate::time::Timer;

#[derive(Debug)]
pub struct BenchmarkResults {
    pub requested_duration: f32,
    pub duration: f32,
    pub execution_count: u32,
    pub avg_time: f32,
    pub per_second: f32,
}

pub fn benchmark<F>(duration: f32, mut code: F) -> BenchmarkResults 
    where F: FnMut() {
    let timer = Timer::new();
    let mut count = 0;
    while timer.elapsed() < duration {
        code();
        count += 1;
    }
    let total_duration = timer.elapsed();
    BenchmarkResults {
        requested_duration: duration,
        duration: total_duration,
        execution_count: count,
        avg_time: total_duration / (count as f32),
        per_second: (count as f32) / total_duration,
    }
}
