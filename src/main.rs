use std::{error, io::{self, ErrorKind}, num::TryFromIntError, vec};

use rand::{rngs::ThreadRng, Rng};

fn decide_multiverse(rng: &mut ThreadRng, times: u64) -> (bool, u64) {
    if times > u32::MAX.into() {return (true, times)}
    if let Ok(times_float) = f64::try_from(times as u32){
        let decision = rng.gen::<f64>() < 1.0-1.0/times_float;
        return (decision, times);
    }

    return (true, times);
}

fn conduct_experiment(mut rng: &mut ThreadRng, n: u64, multiverse: bool) -> Result<(f64, f64), TryFromIntError> {
    let times = usize::try_from(n)?;

    let mut total_correct: Vec<u64> = vec! (0; times as usize);
    let mut total_confidence: Vec<u128> = vec! (0; times as usize);
    
    for i in 0..(times as usize) {        
        if rng.gen::<bool>() {
            let outcome = decide_multiverse(&mut rng, i as u64);
            let correct = multiverse == outcome.0;
            total_correct[i] += correct as u64;
            total_confidence[i] += outcome.1 as u128;   
            
            break
        }
    }

    return Ok((1.0, 1.0))

}

fn main() {
    let mut rng = rand::thread_rng();

    let mut input = String::new();

    println!("Times to do a thing:");
    io::stdin().read_line(&mut input).unwrap();

    let times: u64 = input.parse().unwrap();
    
    println!("Times to do that thing:");
    io::stdin().read_line(&mut input).unwrap();

    let meta_times: usize = input.parse().unwrap();

    for n in 0..meta_times {
        let multiverse: bool = rng.gen::<bool>();
        
    }



}
