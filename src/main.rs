use turambar::svgplot;
use turambar::escalas;

use svgplot::Plot;
use escalas::LinearScale;
use escalas::ContinuousScale;
use svg::node::element::*;


fn main() {
    let mut plot = Plot::new();
        plot.set_margin((0.0,0.0,0.0,0.0));
        let mut mg = plot.get_tgroup();
        let mut docu = plot.get_docu();

        let mut scy = LinearScale::new();
         scy.domain(0.0, 100.0).range(plot.ef_h(), 0.0);

        println!("{:?}",scy.scale(50.0));

        let mut gc = Group::new()
            .set("class","circulos");

        let circulo = Circle::new()
                .set("cx",400)
                .set("cy",300)
                .set("r",200)
                .set("fill","#ECB365")
                .set("stroke-width",15)
                .set("stroke","#064663");

        gc = gc.add(circulo);

        mg = mg.add(gc);

        docu = docu.add(mg);

        svg::save("./pruebas/salida.svg",&docu).unwrap();
}
