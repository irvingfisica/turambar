use crate::escalas;
use svg::node::element::*;

use escalas::ContinuousScale;

pub fn simple_polygon<S,T>(old_data: Option<path::Data>, puntos: &Vec<(f64,f64)>, xscale: &S, yscale: &T) -> Option<path::Data> 
where S: ContinuousScale, 
      T: ContinuousScale,
{
    let mut data = match old_data.clone() {
        Some(path) => path,
        None => path::Data::new()
    };

    let mut coorditer = puntos.iter();

    let inicio = match coorditer.next() {
        Some(coord) => coord,
        None => return old_data
    };

    let scaled_coord = (xscale.scale(inicio.0),yscale.scale(inicio.1));

    match scaled_coord {
        (Some(xsc),Some(ysc)) => {
            data = data.move_to((xsc,ysc));
        },
        _ => return old_data
    };

    for coord in coorditer {
        let scaled_coord = (xscale.scale(coord.0),yscale.scale(coord.1));

        match scaled_coord {
            (Some(xsc),Some(ysc)) => {
                data = data.line_to((xsc,ysc));
            },
            _ => return old_data
        };
    };

    data = data.close();

    Some(data)
}

pub fn geopolygon<S,T>(old_data: Option<path::Data>, pol: &geo::Polygon<f64>, xscale: &S, yscale: &T) -> Option<path::Data> 
where S: ContinuousScale,
      T: ContinuousScale,
{

    let exterior = points_from_linestring(pol.exterior());
    let interiors: Vec<Vec<(f64,f64)>> = pol.interiors().iter()
                            .map(|line| points_from_linestring(line))
                            .collect();

    let mut path = simple_polygon(old_data.clone(), &exterior, xscale, yscale);

    for interior in interiors {
        path = simple_polygon(path, &interior, xscale, yscale) 
    };

    path
}

pub fn geomultipolygon<S,T>(old_data: Option<path::Data>, mpol: &geo::MultiPolygon<f64>, xscale: &S, yscale: &T) -> Option<path::Data>
where S: ContinuousScale,
      T: ContinuousScale,
{

    let mut path = old_data.clone(); 

    for pol in mpol {
        path = geopolygon(path, pol, xscale, yscale);
    };

    path
}

pub fn points_from_linestring(line: &geo::LineString<f64>) -> Vec<(f64,f64)> {
    
    let mut salida = Vec::new();
    
    for point in line.points_iter() {
        salida.push(point.x_y())
    };

    salida
}