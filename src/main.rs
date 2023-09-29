use rand::distributions::{Distribution, Standard};
use rand::thread_rng;
use statrs::distribution::{
    Beta, Cauchy, Chi, ChiSquared, Dirac, Dirichlet, Erlang, Exp, FisherSnedecor, Gamma, Geometric,
    Hypergeometric, InverseGamma, Laplace, LogNormal, Multinomial, MultivariateNormal,
    NegativeBinomial, Normal, Pareto, Poisson, StudentsT, Uniform, Weibull,Triangular,
};


pub fn gamma(alpha:f64, beta:f64)->f64{
    let mut rng = thread_rng();
    let v: &Vec<f64> =
    &Gamma::new(alpha, beta)
        .unwrap()
        .sample_iter(&mut rng)
        .take(1)
        .collect();
    v[0]
}

fn main(){
    for _ 0..1000{
        println!("{}",gamma(4.0,1.0/(2.0)))
    }
}