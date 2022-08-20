use std::time::{Duration, Instant};

#[derive(Clone, Copy)]
pub struct PerformanceStats {
    pub ray_checks: u32,
    pub ray_counter: u32,
    pub aabb_intersections: u32,
    pub object_intersections: u32,
    timer: Instant,
}

impl PerformanceStats {
    pub fn new() -> PerformanceStats {
        PerformanceStats {
            ray_checks: 0,
            ray_counter: 0,
            aabb_intersections: 0,
            object_intersections: 0,
            timer: Instant::now(),
        }
    }

    pub fn increment_checks(&mut self) {
        self.ray_checks += 1;
    }

    pub fn increment_rays(&mut self) {
        self.ray_counter += 1;
    }

    pub fn increment_obj_inters(&mut self) {
        self.object_intersections += 1;
    }

    pub fn increment_aabb_inters(&mut self) { self.aabb_intersections += 1; }

    pub fn time_elapsed(self) -> Duration {
        self.timer.elapsed()
    }
}