use turambar::svgplot;
use turambar::escalas;
use turambar::paths;
use turambar::transforms;

use svgplot::Plot;
use escalas::UnitScale;
use paths::simple_polygon;
use paths::geopolygon;
use paths::geomultipolygon;
use transforms::get_translation;
use svg::node::element::*;
use geo_types::{LineString, Polygon, MultiPolygon};

fn main() {
    let mut plot = Plot::new();
        plot.set_margin((0.0,0.0,0.0,0.0));
        let mut mg = plot.get_tgroup();
        let mut docu = plot.get_docu();

        let sc = UnitScale::new();

        let translate = get_translation(0.0, 100.0);

        let mut gc = Group::new()
                        .set("transform",translate);

        let star: Vec<(f64,f64)> = Vec::from([(50.0,0.0), (21.0,90.0), (98.0,35.0), (2.0,35.0), (79.0,90.0)]);

        let data = simple_polygon(None, &star, &sc, &sc).unwrap();

        let simplepol = Path::new()
                .set("fill","#ECB365")
                .set("stroke-width",3)
                .set("stroke","#064663")
                .set("fill-rule","evenodd")
                .set("d",data);

        gc = gc.add(simplepol);

        let polygon = Polygon::new(
            LineString::from(vec![(100.0, 0.0), (100.0, 100.0), (200.0, 100.0), (200.0, 0.0)]),
            vec![LineString::from(vec![
                (110.0, 10.0),
                (190.0, 90.0),
                (190.0, 10.0),
            ]),
            LineString::from(vec![
                (110.0, 20.0),
                (110.0, 90.0),
                (180.0, 90.0),
            ]),
            ],
        );

        let pdata = geopolygon(None, &polygon, &sc, &sc).unwrap();

        let simplepol = Path::new()
                .set("fill","#ECB365")
                .set("stroke-width",3)
                .set("stroke","#064663")
                .set("fill-rule","evenodd")
                .set("d",pdata);

        gc = gc.add(simplepol);

        let pol1 = Polygon::new(
            LineString::from(vec![
                (210.0, 10.0),
                (290.0, 90.0),
                (290.0, 10.0),
            ]),vec![]
        );

        let pol2 = Polygon::new(
            LineString::from(vec![
                (210.0, 20.0),
                (210.0, 90.0),
                (280.0, 90.0),
            ]),vec![]
        );

        let mpol = MultiPolygon::from(vec![pol1,pol2]);

        let mpdata = geomultipolygon(None, &mpol, &sc, &sc).unwrap();

        let multipol = Path::new()
                .set("fill","#ECB365")
                .set("stroke-width",3)
                .set("stroke","#064663")
                .set("fill-rule","evenodd")
                .set("d",mpdata);

        gc = gc.add(multipol);

        mg = mg.add(gc);

        docu = docu.add(mg);

        svg::save("./pruebas/salida.svg",&docu).unwrap();
}
