type state = [f64; 3];


fn lorenz_dt( x: [f64; 3], sigma: f64, beta: f64, rho: f64) -> [f64;3] {

   let a = sigma * (x[1] - x[0]);
   let b = rho * x[0] - x[1] * x[2];
   let c = beta * x[2] + x[1] * x[2];

   let x_out = [a,b,c];
   return x_out;

}

fn adv_single(x: [f64; 3], fract: f64) -> [f64;3] {


   let dt = lorenz_dt(x, 10_f64, 2.667_f64, 28_f64);
   let fract = 1.0_f64;
   return dt;
   //for i in 0..3
   //let first_step[i] 


}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lorenz_dt() {

    let z = lorenz_dt([1.0, 3.4, 3.2], 10.0, 2.667, 28.0);

    }

}

