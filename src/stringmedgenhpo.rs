use crate::structfile::MedgenHPO;
use rayon::prelude::*;
use std::Sync::{Arc, Mutex};
use std::error::Error;
/*
 Authom GauravSablok
 Instytut Chemii Bioorganicznej
 Polskiej Akademii Nauk
 ul. Noskowskiego 12/14 | 61-704, Poznań
 Date: 2025-4-10
*/

pub fn stringhpo(hpostring: Vec<String>) -> Result<Arc<Mutex<Vec<MedgenHPO>>>, Box<dyn Error>> {
    let stringmedgenhpo = hpostring.clone();
    let mut medgenhpo: Arc<Mutex<Vec<MedgenHPO>>> = Arc::new(Mutex::new(Vec::new()));
    stringmedgenhpo.into_par_iter().for_each(|x| {
        if !x.starts_with("#") {
            let line = x;
            let linesplit: Vec<_> = line.split("|").collect::<Vec<_>>();
            medgenhpo.push(MedgenHPO {
                cui: linesplit[0].clone().to_string(),
                sdui: linesplit[1].clone().to_string(),
                hpostr: linesplit[2].clone().to_string(),
                medgenstr: linesplit[3].clone().to_string(),
                medgenstrsab: linesplit[4].clone().to_string(),
                sty: linesplit[5].clone().to_string(),
            });
        }
    });
    Ok(medgenhpo)
}
