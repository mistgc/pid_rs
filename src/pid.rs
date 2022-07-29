#[derive(Debug)]
pub struct PidCalibaration {
    pub kp: f64,    // Proportional gain
    pub ki: f64,    // Integral gain
    pub kd: f64,    // Derivative gain
}

#[derive(Debug)]
pub struct PidState {
    pub actual: f64,
    pub target: f64,
    pub time_delta: f64,
    pub previous_error: f64,
    pub integral: f64,
    pub output: f64,
}

impl PidCalibaration {
    pub fn new(kp: f64, ki: f64, kd: f64) -> Self {
        Self { kp, ki, kd }
    }

    pub fn set_gain(&mut self, kp: Option<f64>, ki: Option<f64>, kd: Option<f64>) {
        if let Some(kp) = kp {
            self.kp = kp;
        }
        if let Some(ki) = ki {
            self.ki = ki;
        }
        if let Some(kd) = kd {
            self.kd = kd;
        }
    }

    pub fn get_gain(&self) -> (f64, f64, f64) {
        (self.kp, self.ki, self.kd)
    }
}

impl PidState {
    pub fn new(actual: f64, target: f64, dt: f64) -> Self {
        Self {
            actual,
            target,
            time_delta: dt,
            previous_error: 0.,
            integral: 0.,
            output: 0.,
        }
    }
}

pub fn pid_iterate<'a>(calibaration: &PidCalibaration, state: &'a mut PidState) -> &'a mut PidState {
    let error = state.target - state.actual;
    state.integral += error * state.time_delta;
    let derivative = (error - state.previous_error) / state.time_delta;
    state.output = calibaration.kp * error + calibaration.ki * state.integral + calibaration.kd * derivative;
    state.previous_error = error;

    state
}
