pub struct Ensemble {
    state_size: f64,
    ensemble_size: f64,
    ens: Vec<f64>  // should this be a vector of type state?

}


pub fn new(state_size: f64, ensemble_size: f64) -> Ensemble {
  Ensemble {
      state_size,
      ensemble_size,
      ens: vec![0.0, state_size*ensemble_size]

  }
}


impl Ensemble {

    pub fn info(self) {
   
       println!("state size = {} ensemble size = {}", self.state_size, self.ensemble_size) 

    }

}



