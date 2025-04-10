use crate::structfile::MedgenMap;
use rayon::prelude::*;
use std::error::Error;
use std::sync::{Arc, Mutex};
/*
 Authom GauravSablok
 Instytut Chemii Bioorganicznej
 Polskiej Akademii Nauk
 ul. Noskowskiego 12/14 | 61-704, Poznań
 Date: 2025-4-10
*/

pub fn stringmedgen(listread: Vec<String>) -> Result<Arc<Mutex<Vec<MedgenMap>>>, Box<dyn Error>> {
    let stringhold = listread.clone();
    let mut medgenmap: Arc<Mutex<Vec<MedgenMap>>> = Arc::new(Mutex::new(Vec::new()));
    stringhold.into_par_iter().for_each(|x| {
        if !x.starts_with("#") {
            let line = x;
            let linesplit: Vec<_> = line.split("|").collect::<Vec<_>>();
            medgenmap.push(MedgenMap {
                cuiid: linesplit[0].clone().to_string(),
                prefname: linesplit[1].clone().to_string(),
                sourceid: linesplit[2].clone().to_string(),
                source: linesplit[3].clone().to_string(),
            });
        }
    });
    Ok(medgenmap)
}
