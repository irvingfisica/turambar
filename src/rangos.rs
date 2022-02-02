use std::fmt;

pub struct Rangos {
    pub xmin: f64,
    pub xmax: f64,
    pub ymin: f64,
    pub ymax: f64,
}

impl Rangos {
    pub fn new(xrange: (f64,f64), yrange: (f64, f64)) -> Self {
        Rangos {
            xmin: xrange.0,
            xmax: xrange.1,
            ymin: yrange.0,
            ymax: yrange.1,
        }
    }

    pub fn xrange(&self) -> (f64,f64) {
        (self.xmin,self.xmax)
    }

    pub fn yrange(&self) -> (f64,f64) {
        (self.ymin,self.ymax)
    }

    pub fn xlength(&self) -> f64 {
        self.xmax - self.xmin
    }

    pub fn ylength(&self) -> f64 {
        self.ymax - self.ymin
    }

    pub fn ratio(&self) -> f64 {
        self.xlength()/self.ylength()
    }

    pub fn orientacion(&self) -> Orientacion {
        if self.xlength() > self.ylength() {
            Orientacion::Horizontal
        } else {
            Orientacion::Vertical
        }
    }

    pub fn fit(&self, alpha: f64, vpratio: f64) -> Rangos {

        let mut ori = self.orientacion();

        if self.ratio() < vpratio {
            ori = ori.invert();
        }

        let (og, os, rvp) = match ori {
            Orientacion::Vertical => {
                let og = self.ylength();
                let os = self.xlength();
                let rvp = vpratio; 
                (og, os, rvp)
            },
            Orientacion::Horizontal => {
                let og = self.xlength();
                let os = self.ylength();
                let rvp = 1.0/vpratio; 
                (og, os, rvp)
            },
        };

        let ogp = og / alpha;
        let osp = rvp * ogp;
        let beta = os / osp;

        let eg = og * (1.0 - alpha) / alpha;
        let es = os * (1.0 - beta) / beta;

        println!("eg: {}, es: {}, beta: {}", eg, es, beta);

        let (xminp,xmaxp,yminp,ymaxp) = match ori {
            Orientacion::Vertical => {
                let xminp = self.xmin - (es/2.0);
                let xmaxp = self.xmax + (es/2.0);
                let yminp = self.ymin - (eg/2.0);
                let ymaxp = self.ymax + (eg/2.0);
                (xminp,xmaxp,yminp,ymaxp)
            },
            Orientacion::Horizontal => {
                let xminp = self.xmin - (eg/2.0);
                let xmaxp = self.xmax + (eg/2.0);
                let yminp = self.ymin - (es/2.0);
                let ymaxp = self.ymax + (es/2.0);
                (xminp,xmaxp,yminp,ymaxp)
            }
        };

        Rangos {
            xmin: xminp,
            xmax: xmaxp,
            ymin: yminp,
            ymax: ymaxp,
        }
    }
}

pub enum Orientacion {
    Vertical,
    Horizontal,
}

impl Orientacion {
    pub fn invert(&self) -> Orientacion {
        match self {
            Orientacion::Horizontal => Orientacion::Vertical,
            Orientacion::Vertical => Orientacion::Horizontal,
        }
    }
}

impl fmt::Display for Orientacion {
     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Orientacion::Vertical => write!(f, "vertical"),
            Orientacion::Horizontal => write!(f, "horizontal")
        }
     }
}

