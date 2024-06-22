uniffi::setup_scaffolding!("trig");

use std::sync::Arc;

#[derive(uniffi::Record, Debug)]
pub struct Complex {
    pub real: f64,
    pub imag: f64,
}

#[uniffi::export]
fn to_string(a: Complex) -> String {
    format!("{a:?}")
}

#[uniffi::export]
fn one() -> Complex {
    Complex {
        real: 1.0,
        imag: 0.0,
    }
}

#[uniffi::export]
fn add(a: Complex, b: Complex) -> Complex {
    Complex {
        real: a.real + b.real,
        imag: a.imag + b.imag,
    }
}

#[uniffi::export]
fn mul(a: Complex, b: Complex) -> Complex {
    Complex {
        real: a.real * b.real - a.imag * b.imag,
        imag: a.real * b.imag + a.imag * b.real,
    }
}

#[derive(uniffi::Object, Debug, Copy, Clone)]
pub struct Angle {
    rad: f64,
}

#[uniffi::export]
impl Angle {
    #[uniffi::constructor]
    fn from_degrees(deg: f64) -> Arc<Self> {
        Arc::new(Self {
            rad: deg.to_radians(),
        })
    }

    #[uniffi::constructor]
    fn from_rad(rad: f64) -> Arc<Self> {
        Arc::new(Self { rad })
    }

    fn degrees(&self) -> f64 {
        self.rad.to_degrees()
    }

    fn radians(&self) -> f64 {
        self.rad
    }

    #[uniffi::constructor]
    fn tau() -> Arc<Self> {
        Arc::new(Self {
            rad: std::f64::consts::TAU,
        })
    }

    fn print(&self) {
        println!("{self:?}");
    }

    /// Debug print while also returning self for continued operations
    fn dbg(&self) -> Arc<Self> {
        self.hidden();
        println!("{self:?}");
        Arc::new(*self)
    }

    fn mul(&self, x: f64) -> Arc<Self> {
        Arc::new(Self { rad: self.rad * x })
    }

    // Return e^(i*angle)
    fn exp(&self) -> Complex {
        Complex {
            real: self.rad.cos(),
            imag: self.rad.sin(),
        }
    }
}

impl Angle {
    fn hidden(&self) {
        println!("ran fn hidden from ffi");
    }
}
