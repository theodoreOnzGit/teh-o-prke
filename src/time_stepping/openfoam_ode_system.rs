/// rust translation of the OpenFOAM ODE system
///
/// note that this is nested inside ODESolver struct
pub struct ODESystem {
    /// takes in two vectors, x and y 
    /// and then outputs dydx
    user_specified_ode_system: fn(f64, Vec<f64>) -> Vec<f64>
}



impl ODESystem {

    /// constructor for ODE system 
    /// probably want some easier ways to construct the ode system
    pub fn new(ode_system: fn(f64, Vec<f64>) -> Vec<f64>) -> Self
    {
        Self {
            user_specified_ode_system: ode_system
        }
    }


    /// this evaluates a vector dydx based on a vector y and 
    /// vector x
    pub fn derivatives(&self,
        x: f64,
        y: Vec<f64>,
        dydx: &mut Vec<f64>) {

        let dydx_local = (self.user_specified_ode_system)(x,y);

        *dydx = dydx_local;


    }
}
