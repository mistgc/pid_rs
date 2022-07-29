use pid_rs::*;
use std::{ thread, time };
fn main() {
    let calibaration = PidCalibaration::new(0.07, 0.005, 0.005);
    let mut state = PidState::new(0., 100., 1.);
    let (kp, ki, kd) = calibaration.get_gain();
    let second = time::Duration::from_secs_f64(0.1);

    println!("calibaration:\n\tkp: {} \tki: {} \tkd: {}", kp, ki, kd);
    println!("initial state:\n\tactual: {} \ttarget: {} \tdt: {}", state.actual, state.target, state.time_delta);
    
    loop {
        pid_iterate(&calibaration, &mut state);
        state.actual += state.output * 2.;
        println!("real-time state:\n\tactual: {} \ttarget: {} \tdt: {} \tprevious error: {} \tintegral: {} \toutput: {}", state.actual, state.target, state.time_delta, state.previous_error, state.integral, state.output);
        thread::sleep(second);
    }
}
