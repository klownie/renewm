use std::cmp::{max, min};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Grid {
    name: String,
    x0: i32,
    x1: i32,
    xi: i32,
    allow_out_of_bounds: bool,
    d_ovr: f32,
    m_snap: f32,
    last_t: Option<f64>,
    last_x: Option<f32>,
    last_p: Option<f32>,
    last_x_output: Option<f32>,
    last_p_output: Option<f32>,
}

impl Grid {
    pub fn new(name: &str, x0: i32, x1: i32, xi: i32, d_ovr: f32, m_snap: f32) -> Self {
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
            x0,
            x1,
            xi,
            allow_out_of_bounds: true,
            d_ovr,
            m_snap,
            last_t: None,
            last_x: None,
            last_p: None,
            last_x_output: None,
            last_p_output: None,
        }
    }

    pub fn get_bounds(&mut self, x: f32) -> (i32, i32) {
        if self.x0 <= x && x <= self.x1 {
            self.allow_out_of_bounds = false;
        }

        let mut x0 = self.x0;
        let mut x1 = self.x1;

        if self.allow_out_of_bounds && x < x0 {
            x0 = x.floor() as i32;
        }

        if self.allow_out_of_bounds && x > x1 {
            x1 = x.ceil() as i32;
        }

        (x0, x1)
    }
}
