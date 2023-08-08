extern crate rand;

use rand::Rng;

/// 蒙特卡洛随机数生成算法。
/// 
/// 随机生成0-1之间的数，值越大生成的概率越大
pub fn montecarlo() -> f32 {
    let mut rng = rand::thread_rng();
    loop {
        let r1: f32 = rng.gen();
        let probability = r1;
        let r2: f32 = rng.gen();
        if r2 < probability {
            return r1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_montecarlo() {
        let mut cap = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        for _ in 0..100000 {
            let index = (montecarlo() * 10.0) as usize;
            cap[index] += 1;
         }
        println!("{:?}", cap);
        for i in 0..(cap.len() - 1) {
            assert!(cap[i + 1] > cap[i]);
        }
    }
}
