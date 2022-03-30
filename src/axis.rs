use svg::node::element::*;
use crate::escalas::ContinuousScale;

pub fn nicenum(x: f64, round: bool) -> f64 {
    let exp = x.log10().floor();
    let f = x/10.0_f64.powf(exp);

    let nf;
    if round {
        if f < 1.5 {
            nf = 1.0;
        } else if f < 3.0 {
            nf = 2.0;
        } else if f < 7.0 {
            nf = 5.0;
        } else {
            nf = 10.0;
        };
    } else {
        if f <= 1.0 {
            nf = 1.0;
        } else if f <= 2.0 {
            nf = 2.0;
        } else if f <= 5.0 {
            nf = 5.0;
        } else {
            nf = 10.0;
        };
    };

    nf * 10.0_f64.powf(exp)
}

pub fn loose_label(ntick:usize, min:f64, max:f64) -> Vec<Tick> {
    let range = nicenum(max - min, false);
    let d = nicenum(range/(ntick as f64 - 1.0), true);
    let graphmin = (min/d).floor() * d;
    let graphmax = (max/d).ceil() * d;
    let nfrac = (-1.0 * d.log10().floor()).max(0.0) as usize;

    let mut x = graphmin;
    let mut salida = Vec::new();
    while x < (graphmax + 0.5*d) {
        if (x >= min) && (x <= max) {
            let etiqueta = format!("{:.*}",nfrac,x);
            let tick = Tick {
                valor: x,
                label: etiqueta,
            };
        
            salida.push(tick);
        }
        x = x + d;
    };

    salida
}

#[derive(Debug)]
pub struct Tick {
    pub valor: f64,
    pub label: String,
}

pub enum Direccion {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Debug)]
pub struct Axis {
    pub eje: Line,
    pub ticks: Vec<Line>,
    pub tilab: Vec<Text>,
}

impl Axis {
    pub fn new<T: ContinuousScale>(direccion: Direccion, escala: &T, nticks: usize, fsize: f64) -> Self {

        let mut axis = Line::new();
        let mut ltic = Vec::new();
        let mut tlab = Vec::new();

        let extent = escala.extent();

        match (escala.scale(extent.0),escala.scale(extent.1)) {
            (Some(e0),Some(e1)) => {
                
                let ticks = loose_label(nticks, extent.0, extent.1);
                let mut fsist = fsize.to_string();
                fsist.push_str("px");

                match direccion {
                    Direccion::Top => {
                        axis = axis.set("x1",e0)
                                    .set("y1",0.0)
                                    .set("x2",e1)
                                    .set("y2",0.0);
                    },
                    Direccion::Bottom => {
                        axis = axis.set("x1",e0)
                                    .set("y1",0.0)
                                    .set("x2",e1)
                                    .set("y2",0.0);
                    },
                    Direccion::Left => {
                        axis = axis.set("x1",0.0)
                                    .set("y1",e0)
                                    .set("x2",0.0)
                                    .set("y2",e1);
                    },
                    Direccion::Right => {
                        axis = axis.set("x1",0.0)
                                    .set("y1",e0)
                                    .set("x2",0.0)
                                    .set("y2",e1);
                    }
                };

                axis = axis.set("stroke","black")
                            .set("stroke-width",0.5);

                for tick in ticks.iter() {
                    if tick.valor < extent.1 {

                        match direccion {
                            Direccion::Top => {
                                let ticko = Line::new()
                                            .set("x1", escala.scale(tick.valor).unwrap())
                                            .set("y1", -2.0)
                                            .set("x2", escala.scale(tick.valor).unwrap())
                                            .set("y2", 2.0)
                                            .set("stroke","black")
                                            .set("stroke-width", 0.5);

                                ltic.push(ticko);

                                let label = Text::new()
                                            .set("x", escala.scale(tick.valor).unwrap())
                                            .set("y", -4.5)
                                            .set("text-anchor","middle")
                                            .set("font-size",fsist.to_string())
                                            .add(svg::node::Text::new(tick.label.to_string()));

                                tlab.push(label);
                            },
                            Direccion::Bottom => {
                                let ticko = Line::new()
                                            .set("x1", escala.scale(tick.valor).unwrap())
                                            .set("y1", -2.0)
                                            .set("x2", escala.scale(tick.valor).unwrap())
                                            .set("y2", 2.0)
                                            .set("stroke","black")
                                            .set("stroke-width", 0.5);

                                ltic.push(ticko);

                                let label = Text::new()
                                            .set("x", escala.scale(tick.valor).unwrap())
                                            .set("y", 4.5)
                                            .set("text-anchor","middle")
                                            .set("dominant-baseline","hanging")
                                            .set("font-size",fsist.to_string())
                                            .add(svg::node::Text::new(tick.label.to_string()));

                                tlab.push(label);
                            },
                            Direccion::Left => {
                                let ticko = Line::new()
                                            .set("y1", escala.scale(tick.valor).unwrap())
                                            .set("x1", -2.0)
                                            .set("y2", escala.scale(tick.valor).unwrap())
                                            .set("x2", 2.0)
                                            .set("stroke","black")
                                            .set("stroke-width", 0.5);

                                ltic.push(ticko);

                                let label = Text::new()
                                            .set("y", escala.scale(tick.valor).unwrap())
                                            .set("x", -4.5)
                                            .set("text-anchor","end")
                                            .set("dominant-baseline","middle")
                                            .set("font-size",fsist.to_string())
                                            .add(svg::node::Text::new(tick.label.to_string()));

                                tlab.push(label);
                            },
                            Direccion::Right => {
                                let ticko = Line::new()
                                            .set("y1", escala.scale(tick.valor).unwrap())
                                            .set("x1", -2.0)
                                            .set("y2", escala.scale(tick.valor).unwrap())
                                            .set("x2", 2.0)
                                            .set("stroke","black")
                                            .set("stroke-width", 0.5);

                                ltic.push(ticko);

                                let label = Text::new()
                                            .set("y", escala.scale(tick.valor).unwrap())
                                            .set("x", 4.5)
                                            .set("dominant-baseline","middle")
                                            .set("font-size",fsist.to_string())
                                            .add(svg::node::Text::new(tick.label.to_string()));

                                tlab.push(label);
                            },
                        }
                    };
                };
            },
            _ => {}
        }

        Axis {
            eje: axis,
            ticks: ltic,
            tilab: tlab
        }
    }

    pub fn consolidar(&self) -> Group {
        let mut ga = Group::new()
            .set("class","axis");

            ga = ga.add(self.eje.clone());
            for tick in self.ticks.iter() {
                ga = ga.add(tick.clone());
            };
            for tick in self.tilab.iter() {
                ga = ga.add(tick.clone());
            };

        ga
    }
}

