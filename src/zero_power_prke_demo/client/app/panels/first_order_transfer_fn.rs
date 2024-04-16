use uom::si::{f64::*, time::second};

#[derive(Debug,PartialEq, PartialOrd, Clone)]
pub struct FirstOrderStableTransferFn {
    process_gain: f64,
    process_time: Time,
    previous_timestep_input: f64,
    /// previous timestep output
    offset: f64,
    /// delay
    delay: Time,

    /// vector of first order responses 
    response_vec: Vec<FirstOrderResponse>,
}

impl Default for FirstOrderStableTransferFn {
    /// default is: 
    ///
    /// 1 / (s + 1)
    ///
    /// with initial user input of 0.0 
    /// and initial user value of 0.0
    fn default() -> Self {
        FirstOrderStableTransferFn { 
            process_gain: 1.0, 
            process_time: Time::new::<second>(1.0), 
            previous_timestep_input: 0.0, 
            offset: 0.0, 
            delay: Time::new::<second>(0.0), 
            response_vec: vec![],
        }
    }
}

impl FirstOrderStableTransferFn {

    /// constructors 
    pub fn new(process_gain: f64,
        process_time: Time,
        initial_input: f64,
        initial_value: f64,
        delay: Time,) -> Self {
        FirstOrderStableTransferFn { 
            process_gain, 
            process_time, 
            previous_timestep_input: initial_input, 
            offset: initial_value, 
            delay, 
            response_vec: vec![],
        }
    }

    /// first order filter 
    pub fn new_filter(process_time: Time,
        initial_input: f64,
        initial_value: f64) -> Self {
        FirstOrderStableTransferFn { 
            process_gain: 1.0, 
            process_time, 
            previous_timestep_input: initial_input, 
            offset: initial_value, 
            delay: Time::new::<second>(0.0), 
            response_vec: vec![],
        }
    }

    /// sets the user input to some value
    pub fn set_user_input_and_calc_output(&mut self, 
        current_time: Time,
        current_input: f64) 
    -> f64 {
        // check if input is equal to current input 

        // case where input is not the same to 9 decimal places

        let input_changed: bool = 
            (current_input * 1e9).round() 
            - (self.previous_timestep_input.clone()*1e9).round() != 0.0 ;

        if input_changed {
            // need to add a response to the vector

            let process_gain = self.process_gain;
            let process_time = self.process_time;
            let user_input = current_input - self.previous_timestep_input;
            // the time where the first order response kicks in
            let start_time = current_time + self.delay;

            // make a new response
            let new_response = FirstOrderResponse::new(
                process_gain,
                process_time,
                start_time,
                user_input,
                current_time
            );

            // add response to the vector
            self.response_vec.push(new_response);

            // then we need to change the previous_timestep_input 
            // to the current input value 
            self.previous_timestep_input = current_input;

            // then we are done!
            
        }

        // clean up the vector first
        self.clear_first_order_response_vector();

        // need to calculate using the list of 
        // first order response vectors as per normal
        //
        // So we are summing this up
        // O(t) = summing:: u2(t - t2) * b [1-exp(-a * [t-t2])] 
        // + offset
        // first we add the offset

        let summation_of_responses: f64 = self.response_vec.
            iter_mut().map(
                |first_order_response|{
                    first_order_response.calculate_response(current_time)}
            ).sum();

        let output = self.offset + summation_of_responses;

        return output;

    }

    /// clears the item if they have reached steady state
    fn clear_first_order_response_vector(&mut self){

        let index_of_steady_state_result = self.response_vec.iter().position(
            |first_order_response| {
                first_order_response.is_steady_state()
            }
        );

        match index_of_steady_state_result {

            // if I found something at the index, remove it, 
            // repeatedly test it until nothing is left
            Some(index) => {
                // first get the steady state value and add it to the 
                // offset
                let first_order_response = self.response_vec[index].clone();
                let steady_state_value_of_response = 
                    first_order_response.steady_state_value();
                self.offset += steady_state_value_of_response;

                // then i remove the first order response from the 
                // index
                self.response_vec.remove(index);
            },

            // if no vectors reach steady state, exit
            // with no issue
            None => return,
        }

        // now, we have cleared the vector once, if there are other 
        // times we need to clear the vector, then we enter a loop

        let index_of_steady_state_result = self.response_vec.iter().position(
            |first_order_response| {
                first_order_response.is_steady_state()
            }
        );
        // check if steady state responses are present
        let mut steady_state_responses_present = 
            match index_of_steady_state_result {
                Some(_) => true,
                None => false,
            };
        
        if !steady_state_responses_present {
            return;
        } 

        // repeatedly clear the vector until no steady state responses 
        // are left
        while steady_state_responses_present {

            // check for index
            let index_of_steady_state_result = self.response_vec.iter().position(
                |first_order_response| {
                    first_order_response.is_steady_state()
                }
            );

            steady_state_responses_present = match index_of_steady_state_result {
                Some(index) => {
                    // first get the steady state value and add it to the 
                    // offset
                    let first_order_response = self.response_vec[index].clone();
                    let steady_state_value_of_response = 
                        first_order_response.steady_state_value();
                    self.offset += steady_state_value_of_response;

                    // then i remove the first order response from the 
                    // index
                    self.response_vec.remove(index);

                    // return true value to while loop
                    true
                },
                // return false value to while loop
                None => false,
            };

        }
        return;

    }
    
}




/// first order response struct, 
/// will help to caluclate
/// u1(t - t1) * Kp * [1-exp(- [t-t1] / tau])
#[derive(Debug,PartialEq, PartialOrd, Clone, Copy)]
pub struct FirstOrderResponse {
    process_gain: f64,
    process_time: Time,
    start_time: Time,
    user_input: f64,
    current_time: Time,
}

impl Default for FirstOrderResponse {
    fn default() -> Self {
        FirstOrderResponse { 
            process_gain: 1.0, 
            process_time: Time::new::<second>(1.0), 
            start_time: Time::new::<second>(0.0), 
            user_input: 1.0, 
            current_time: Time::new::<second>(0.0),
        }
    }
}


impl FirstOrderResponse {

    /// constructor 
    pub fn new(
        process_gain: f64,
        process_time: Time,
        start_time: Time,
        user_input: f64,
        current_time: Time,) -> Self {
        FirstOrderResponse { 
            process_gain, 
            process_time, 
            start_time, 
            user_input, 
            current_time,
        }
    }

    /// checks if the transfer function has more or less reached 
    /// steady state,
    ///
    /// I consider this where the time elapsed is 20 times 
    /// the process_time
    ///
    /// this is because exp(-20) is about 2e-9, it is tiny...
    pub fn is_steady_state(&self) -> bool {
        let time_elapsed = self.current_time - self.start_time;

        let time_ratio: f64 = time_elapsed.value/self.process_time.value;

        if time_ratio > 20.0 {
            return true;
        }

        return false;
    }


    /// calculates the response of the first order system
    /// at a given time
    /// u1(t - t1) * Kp * [1-exp(- [t-t1] / tau])
    pub fn calculate_response(&mut self, simulation_time: Time) -> f64 {

        // get the current time (t - t0)
        self.current_time = simulation_time;
        let time_elapsed = self.current_time - self.start_time;

        // first let's deal with the heaviside function

        let heaviside_on: bool = self.current_time > self.start_time;

        // if the current time is before start time, no response 
        // from this transfer function
        if !heaviside_on {
            return 0.0;
        }

        let time_ratio: Ratio = time_elapsed /  self.process_time;
        let exponent_ratio: f64 = -time_ratio.value;

        // otherwise, calculate as per normal

        // u1(t - t1) * Kp * [1-exp(- [t-t1] / tau])
        let response: f64 = self.steady_state_value()
            * (1.0 - exponent_ratio.exp());

        return response;
    }

    /// steady state value 
    /// u1(t - t1) * Kp 
    pub fn steady_state_value(&self) -> f64 {
        let response: f64 = self.user_input * self.process_gain;
        response
    }
}

