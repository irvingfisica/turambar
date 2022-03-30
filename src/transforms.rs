use num_format::{Locale, ToFormattedString};
use voca_rs::*;

pub fn get_translation(x: f64, y: f64) -> String {
    let mut trans = "translate(".to_string();
            trans.push_str(&(x.to_string()));
            trans.push_str(",");
            trans.push_str(&(y.to_string()));
            trans.push_str(")");

    trans
}

pub fn get_rotation(angle: f64) -> String {
    let mut trans = "rotate(".to_string();
            trans.push_str(&(angle.to_string()));
            trans.push_str(")");

    trans
}

pub fn get_anchored_rotation(angle: f64, x: f64, y: f64) -> String {
    let mut trans = "rotate(".to_string();
            trans.push_str(&(angle.to_string()));
            trans.push_str(" ");
            trans.push_str(&(x.to_string()));
            trans.push_str(" ");
            trans.push_str(&(y.to_string()));
            trans.push_str(")");

    trans
}

pub fn to_pxs(size: f64) -> String {
    let mut cadena = size.to_string();
    cadena.push_str("px");

    cadena
}

pub fn to_locale_en<T: ToFormattedString>(num: T) -> String {
    num.to_formatted_string(&Locale::en)
}

pub fn to_prcnt(num: f64) -> String {
    let mut pcstr = format!("{:.*}",2,num).to_string();
    pcstr.push_str("%");
    pcstr
}

pub fn to_title(titulo: &str) -> String {
    let mut titcap = case::title_case(titulo);

    titcap = titcap.replace("De ","de ");
    titcap = titcap.replace("Del ","del ");

    titcap
} 

pub fn compose_title(first: Option<String>, second: Option<String>) -> String {
    
    let mut cadena = "".to_string();
    
    match (first, second) {
        (Some(prim),Some(secu)) => {
            cadena.push_str(&to_title(&prim));
            cadena.push_str(", ");
            cadena.push_str(&to_title(&secu));
        },

        (Some(prim),None) => {
            cadena.push_str(&to_title(&prim));
        }, 

        (None,Some(secu)) => {
            cadena.push_str(&to_title(&secu));
        },

        _ => {}
    }

    cadena
}