use std::collections::HashMap;

pub trait ContinuousScale {
    fn scale(&self, valor: f64) -> Option<f64>;
    fn extent(&self) -> (f64,f64);
}

pub struct Interpolator {
    a: f64,
    b: f64,
}

impl Interpolator {

    pub fn new(a: f64, b: f64) -> Self {
        Interpolator {
            a: a,
            b: b,
        }
    }

    pub fn interpolate(&self, t: f64) -> f64 {
        self.a + (self.b - self.a) * t
    }

    pub fn normalize(&self, v: f64) -> Option<f64> {
        if (self.b - self.a) != 0.0 {
            Some((v - self.a)/(self.b - self.a))
        } else {
            None
        }
    }

    pub fn extent(&self) -> (f64, f64) {
        (self.a,self.b)
    }
}

#[allow(dead_code)]
pub struct LinearScale {
    pub domain: Interpolator,
    pub range: Interpolator,
}

impl LinearScale {
    pub fn new() -> Self {
        LinearScale {
            domain: Interpolator::new(0.0, 1.0),
            range: Interpolator::new(0.0, 1.0),
        }
    }

    pub fn domain(&mut self, a: f64, b: f64) -> &mut Self {
        self.domain = Interpolator::new(a,b);
        self
    }

    pub fn range(&mut self, a: f64, b: f64) -> &mut Self {
        self.range = Interpolator::new(a,b);
        self
    }

    // pub fn scale(&self, ve: f64) -> Option<f64> {
    //     match self.domain.normalize(ve) {
    //         Some(t) => Some(self.range.interpolate(t)),
    //         None => None,
    //     }
    // }
}

impl ContinuousScale for LinearScale {
    fn scale(&self, valor: f64) -> Option<f64> {
        match self.domain.normalize(valor) {
            Some(t) => Some(self.range.interpolate(t)),
            None => None,
        }
    }

    fn extent(&self) -> (f64,f64) {
        self.domain.extent()
    }
}

#[allow(dead_code)]
pub struct OrdinalScale {
    domain: Vec<String>,
    range: Vec<String>,
    mapa: HashMap<String,String>,
    unknown: Option<String>,
}

impl OrdinalScale {
    pub fn new() -> Self {
        OrdinalScale {
            domain: Vec::new(),
            range: Vec::new(),
            mapa: HashMap::new(),
            unknown: None
        }
    }

    pub fn domain(&mut self, dominio: &Vec<String>) -> &mut Self {
        self.domain = dominio.iter().map(|ele|ele.to_string()).collect();

        if !self.range.is_empty() {
            self.consolidate();
        };

        self
    }

    pub fn range(&mut self, rango: &Vec<String>) -> &mut Self {
        self.range = rango.iter().map(|ele|ele.to_string()).collect();

        if !self.domain.is_empty() {
            self.consolidate();
        };

        self
    }

    pub fn unknown(&mut self, valor: &str) -> &mut Self {
        self.unknown = Some(valor.to_string());
        self
    }

    fn consolidate(&mut self) {
        for (inpst, outst) in self.domain.iter().zip(self.range.iter()) {
            self.mapa.insert(inpst.to_string(), outst.to_string());
        }
    }

    pub fn scale(&self, ve: &str) -> Option<String> {
        match self.mapa.get(ve) {
            Some(salida) => Some(salida.to_string()),
            None => match &self.unknown {
                Some(salida) => Some(salida.to_string()),
                None => None,
            }
        }
    }
}

#[allow(dead_code)]
pub struct BandScale {
    domain: Vec<String>,
    range: (f64, f64),
    mapa: HashMap<String,f64>,
    step: f64,
    bandwidth: f64,
    round: bool,
    padding_inner: f64,
    padding_outer: f64,
    align: f64,
}

impl BandScale {
    pub fn new() -> Self {
        BandScale {
            domain: Vec::new(),
            range: (0.0, 1.0),
            mapa: HashMap::new(),
            step: 0.0,
            bandwidth: 0.0,
            round: false,
            padding_inner: 0.0,
            padding_outer: 0.0,
            align: 0.5,
        }
    }

    pub fn domain(&mut self, dominio: &Vec<String>) -> &mut Self {
        self.domain = dominio.iter().map(|ele|ele.to_string()).collect();

        self.consolidate();

        self
    }

    pub fn range(&mut self, a: f64, b: f64) -> &mut Self {
        self.range = (a, b);

        if !self.domain.is_empty() {
            self.consolidate();
        };

        self
    }

    pub fn range_round(&mut self, a: f64, b: f64) -> &mut Self {
        self.range = (a, b);
        self.round = true;

        if !self.domain.is_empty() {
            self.consolidate();
        };

        self
    }

    pub fn round(&mut self, redo: bool) -> &mut Self {
        self.round = redo;

        if !self.domain.is_empty() {
            self.consolidate();
        };

        self
    }

    pub fn padding_inner(&mut self, padding: f64) -> &mut Self {
        self.padding_inner = padding;

        if !self.domain.is_empty() {
            self.consolidate();
        };

        self
    }

    pub fn padding_outer(&mut self, padding: f64) -> &mut Self {
        self.padding_outer = padding;

        if !self.domain.is_empty() {
            self.consolidate();
        };

        self
    }

    pub fn align(&mut self, ali: f64) -> &mut Self {
        self.align = ali;

        if !self.domain.is_empty() {
            self.consolidate();
        };

        self
    }

    pub fn step(&mut self) -> f64 {
        self.step
    }

    pub fn bandwidth(&mut self) -> f64 {
        self.bandwidth
    }

    fn consolidate(&mut self) {
        let mut start: f64;
        let stop: f64;
        let n: usize = self.domain.len();

        if self.range.0 < self.range.1 {
            start = self.range.0;
            stop = self.range.1;
        } else {
            start = self.range.1;
            stop = self.range.0;
        }

        let vtemp: f64 = n as f64 - self.padding_inner + self.padding_outer * 2.0;
        self.step = (stop - start) / vtemp.max(1.0);
        if self.round {
            self.step = self.step.round();
        };

        start = start + (stop - start - self.step * (n as f64 - self.padding_inner)) * self.align;
        self.bandwidth = self.step * (1.0 - self.padding_inner);
        if self.round {
            start = start.round();
            self.bandwidth = self.bandwidth.round();
        };

        for (i,key) in self.domain.iter().enumerate() {
            self.mapa.insert(key.to_string(), start + (self.step * i as f64));
        };
    }

    pub fn scale(&self, ve: &str) -> Option<f64> {
        match self.mapa.get(ve) {
            Some(salida) => Some(*salida),
            None => None,
        }
    }
}

