/*---------------------------------------------------------------------------*\
  =========                 |
  \\      /  F ield         | OpenFOAM: The Open Source CFD Toolbox
   \\    /   O peration     |
    \\  /    A nd           | www.openfoam.com
     \\/     M anipulation  |
-------------------------------------------------------------------------------
    Copyright (C) 2013-2016 OpenFOAM Foundation
    Copyright (C) 2019 OpenCFD Ltd.
-------------------------------------------------------------------------------
License
    This file is part of OpenFOAM.

    OpenFOAM is free software: you can redistribute it and/or modify it
    under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    OpenFOAM is distributed in the hope that it will be useful, but WITHOUT
    ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
    FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License
    for more details.

    You should have received a copy of the GNU General Public License
    along with OpenFOAM.  If not, see <http://www.gnu.org/licenses/>.

\*---------------------------------------------------------------------------*/
// #include "RKF45.H"
// #include "addToRunTimeSelectionTable.H"
// these are the constants from the RKF45.C
const c2  : f64 = 1.0/4.0;
const c3  : f64 = 3.0/8.0;
const c4  : f64 = 12.0/13.0;
const c5  : f64 = 1.0;
const c6  : f64 = 1.0/2.0;
const a21 : f64 = 1.0/4.0;
const a31 : f64 = 3.0/32.0;
const a32 : f64 = 9.0/32.0;
const a41 : f64 = 1932.0/2197.0;
const a42 : f64 = -7200.0/2197.0;
const a43 : f64 = 7296.0/2197.0;
const a51 : f64 = 439.0/216.0;
const a52 : f64 = -8.0;
const a53 : f64 = 3680.0/513.0;
const a54 : f64 = -845.0/4104.0;
const a61 : f64 = -8.0/27.0;
const a62 : f64 = 2.0;
const a63 : f64 = -3544.0/2565.0;
const a64 : f64 = 1859.0/4104.0;
const a65 : f64 = -11.0/40.0;
const b1  : f64 = 16.0/135.0;
const b3  : f64 = 6656.0/12825.0;
const b4  : f64 = 28561.0/56430.0;
const b5  : f64 = -9.0/50.0;
const b6  : f64 = 2.0/55.0;
const e1  : f64 = 25.0/216.0 - b1;
const e3  : f64 = 1408.0/2565.0 - b3;
const e4  : f64 = 2197.0/4104.0 - b4;
const e5  : f64 = -1.0/5.0 - b5;
const e6  : f64 = -b6;



#[allow(non_snake_case)]
#[derive(Debug,Clone)]
pub struct RKF45 {
    yTemp_: Vec<f64>,
    k2_: Vec<f64>,
    k3_: Vec<f64>,
    k4_: Vec<f64>,
    k5_: Vec<f64>,
    k6_: Vec<f64>,
    err_: Vec<f64>,

}

#[allow(non_snake_case)]
impl RKF45 {

    #[inline]
    pub fn solve(&mut self, x0: f64, 
        y0: Vec<f64>,
        dydx0: Vec<f64>,
        dx: f64,
        y: Vec<f64>){

        let yTemp_ = self.yTemp_.clone();

        for (i,_yTemp) in yTemp_.iter().enumerate() {
            self.yTemp_[i] = y0[i] + a21*dx*dydx0[i];
        }

        //note: it appears odes_ is a reference to the ODE system 
        //in ODESolver.H

        // odes_.derivatives(x0 + c2*dx, yTemp_, k2_);

        // forAll(yTemp_, i)
        // {
        //     yTemp_[i] = y0[i] + dx*(a31*dydx0[i] + a32*k2_[i]);
        // }
        for (i,_yTemp) in yTemp_.iter().enumerate() {
            self.yTemp_[i] = y0[i] + dx*(a31*dydx0[i] + a32*self.k2_[i]);
        }

        // odes_.derivatives(x0 + c3*dx, yTemp_, k3_);

        // forAll(yTemp_, i)
        // {
        //     yTemp_[i] = y0[i] + dx*(a41*dydx0[i] + a42*k2_[i] + a43*k3_[i]);
        // }
        for (i,_yTemp) in yTemp_.iter().enumerate() {
            self.yTemp_[i] = y0[i] + dx*(a41*dydx0[i] + a42*self.k2_[i] + a43*self.k3_[i]);
        }

        // odes_.derivatives(x0 + c4*dx, yTemp_, k4_);

        // forAll(yTemp_, i)
        // {
        //     yTemp_[i] = y0[i]
        //         + dx*(a51*dydx0[i] + a52*k2_[i] + a53*k3_[i] + a54*k4_[i]);
        // }
        for (i,_yTemp) in yTemp_.iter().enumerate() {
            self.yTemp_[i] = y0[i]
                + dx*(a51*dydx0[i] + a52*self.k2_[i] + a53*self.k3_[i] + a54*self.k4_[i]);
        }

        // odes_.derivatives(x0 + c5*dx, yTemp_, k5_);

        // forAll(yTemp_, i)
        // {
        //     yTemp_[i] = y0[i]
        //         + dx
        //         *(a61*dydx0[i] + a62*k2_[i] + a63*k3_[i] + a64*k4_[i] + a65*k5_[i]);
        // }
        for (i,_yTemp) in yTemp_.iter().enumerate() {
            self.yTemp_[i] = y0[i]
                + dx*(
                    a61*dydx0[i] + a62*self.k2_[i] + a63*self.k3_[i] 
                    + a64*self.k4_[i] + a65*self.k5_[i]
                );
        }
    }

}
