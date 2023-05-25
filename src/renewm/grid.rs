use super::config;
use std::time::{SystemTime, UNIX_EPOCH};

static CONF_TIME_SCALE: f32 = match config::configured_value("grid.time_scale", 0.3) {
    Ok(value) => value,
    Err(_) => 0.3,
};

static CONF_THROW_PS: Vec<u32> = match config::configured_value("grid.throw_ps", vec![1, 5, 15]) {
    Ok(value) => value,
    Err(_) => vec![1, 5, 15],
};

static CONF_MIN_DIST: f32 = match config::configured_value("grid.min_dist", 0.05) {
    Ok(value) => value,
    Err(_) => 0.05,
};

static CONF_GRIDDEBUG: bool = match config::configured_value("grid.debug", false) {
    Ok(value) => value,
    Err(_) => false,
};

pub struct Grid {
    name: String,
    left_bound: i32,
    right_bound: i32,
    bound_i: i32,
    allow_out_of_bounds: bool,
    d_ovr: f32,
    m_snap: f32,
    last_t: Option<f32>,
    last_x: Option<f32>,
    last_pos: Option<f32>,
    last_x_output: Option<f32>,
    last_pos_output: Option<f32>,
}

impl Grid {
    pub fn new(
        name: &str,
        left_bound: i32,
        right_bound: i32,
        bound_i: i32,
        d_ovr: f32,
        m_snap: f32,
    ) -> Self {
        Grid {
            name: format!(
                "{}-{}",
                name,
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis()
                    % 1000
            ),
            left_bound,
            right_bound,
            bound_i,
            allow_out_of_bounds: true,
            d_ovr,
            m_snap,
            last_t: None,
            last_x: None,
            last_pos: None,
            last_x_output: None,
            last_pos_output: None,
        }
    }

    pub fn get_bounds(&mut self, x: &f32) -> (i32, i32) {
        let (left, right) = (self.left_bound as f32, self.right_bound as f32);

        if (left..=right).contains(&x) {
            self.allow_out_of_bounds = false;
        }

        let mut new_left = self.left_bound;
        let mut new_right = self.right_bound;

        if self.allow_out_of_bounds && x < &left {
            new_left = x.floor() as i32;
        }

        if self.allow_out_of_bounds && x > &right {
            new_right = x.ceil() as i32;
        }

        (new_left, new_right)
    }

    pub fn at(&mut self, x: f32, silent: bool) -> f32 {
        let (left, right) = self.get_bounds(&x);

        let t = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs_f32();
        if !silent {
            if let (Some(last_x), Some(last_t)) = (self.last_x, self.last_t) {
                let diffx = x - last_x;
                let difft = t - last_t;
                self.last_pos = Some(diffx / difft);
            }
            self.last_x = Some(x.clone());
        }

        let mut xp = x;
        if x < left as f32 {
            if self.m_snap == 1.0 {
                if self.d_ovr > 0.0 {
                    let y = x - left as f32;
                    let y = self.d_ovr * (1.0 / (1.0 - y / self.d_ovr) - 1.0);
                    xp = left as f32 + y;
                } else {
                    xp = 0.0;
                }
            } else {
                let y = f32::max(0.0, x - left as f32 + 1.0);

                if y == 0.0 {
                    xp = left as f32 - self.d_ovr;
                } else {
                    xp = left as f32 - self.d_ovr
                        + self.d_ovr / (1.0 + ((1.0 - y) / y).powf(self.m_snap));
                }
            }
        } else if x < right as f32 {
            let y = x - x.floor();

            if y == 0.0 {
                xp = x.floor();
            } else {
                xp = x.floor() + 1.0 / (1.0 + ((1.0 - y) / y).powf(self.m_snap));
            }
        } else {
            if self.m_snap == 1.0 {
                if self.d_ovr > 0.0 {
                    let y = x - right as f32;
                    let y = self.d_ovr * (1.0 / (1.0 + y / self.d_ovr) - 1.0);
                    xp = right as f32 - y;
                }
                {
                    xp = right as f32;
                }
            } else {
                let y = f32::min(1.0, f32::max(0.0, x - right as f32));

                if y == 0.0 {
                    xp = right as f32;
                } else {
                    xp = right as f32 + self.d_ovr / (1.0 + ((1.0 - y) / y).powf(self.m_snap));
                }
            }
        }

        if !silent {
            if let (Some(last_x_output), Some(last_t)) = (self.last_x_output, self.last_t) {
                let dx = xp - last_x_output;
                let dt = t - last_t;
                self.last_pos_output = Some(dx / dt);
            }
            self.last_x_output = Some(xp);
            self.last_t = Some(t);
        }

        // if !silent && conf_griddebug() {
        //     println!(
        //         "GRID[{}]: {}, {}, {}, {}, {}",
        //         self.name,
        //         t,
        //         x,
        //         xp,
        //         self.last_pos.unwrap_or(0.0),
        //         self.last_pos_output.unwrap_or(0.0)
        //     );
        // }

        xp
    }

    fn final_value(&self, throw_dist_max: Option<f32>) -> (i32, f32) {
        let conf_min_dist = conf_min_dist();

        let throw_dist_max = throw_dist_max.unwrap_or(1.0 - conf_min_dist());

        if self.last_x_output.is_none() {
            return (self.at(self.bound_i as f32, false).round() as i32, 0.0);
        }

        let (left, right) = self.get_bounds(&self.last_x_output.unwrap());

        // Find final x
        let x_base = self.last_x_output.unwrap();
        let mut p = 0.0;
        if let Some(last_pos) = self.last_pos {
            p = last_pos;
        }

        let mut x_finals = vec![x_base.round()];

        if p > 0.0 {
            if x_finals[0] > x_base {
                x_finals.push(x_base.round());
            }

            let mut x = x_finals[0] + 1.0;
            while x < x_base + throw_dist_max {
                x_finals.push(x.round());
                x += 1.0;
            }
        } else if p < 0.0 {
            if x_finals[0] < x_base {
                x_finals.push(x_base.round());
            }

            let mut x = x_finals[0] - 1.0;
            while x > x_base - throw_dist_max {
                x_finals.push(x.round());
                x -= 1.0;
            }
        }

        let mut ifinal = 0;
        let conf_throw_ps = conf_throw_ps();
        while ifinal < conf_throw_ps.len() {
            if f32::abs(p) < conf_throw_ps[ifinal] {
                break;
            }
            ifinal += 1;
        }
        let xf = x_finals[ifinal.min(x_finals.len() - 1)].round();

        let xf = right.min(left.max(xf.round() as i32));
        let dx = f32::abs(self.last_x_output.unwrap() - xf as f32);
        let dt = dx * conf_time_scale();

        let compare_t = f32::abs(x_base - xf as f32) / f32::max(f32::abs(p), 0.01);
        let dt = if compare_t < dt { compare_t } else { dt };

        if conf_griddebug() {
            let xb = self.at(self.last_x_output.unwrap(), true);
            let t0 = time::Instant::now();
            for i in 0..2 {
                log::debug!(
                    "GRID[{}]: {}, {}, {}, {}, {}",
                    self.name,
                    t0 + i * dt,
                    0,
                    xb + i * (xf - xb),
                    0,
                    if dt == 0.0 {
                        0.0
                    } else if xf > xb {
                        dx / dt
                    } else {
                        -dx / dt
                    }
                );
            }
        }

        (xf as i32, dt)
    }
}
