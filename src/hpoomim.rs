use crate::structfile::MedgenPubMed;
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

pub fn stringpoomim(
    pubmedstring: Vec<String>,
) -> Result<Arc<Mutex<Vec<MedgenPubMed>>>, Box<dyn Error>> {
    let stringpubmed = pubmedstring.clone();
    let mut medgenpubmed: Arc<Mutex<Vec<MedgenPubMed>>> = Arc::new(Mutex::new(Vec::new()));
    stringpubmed.into_par_iter().for_each(|x| {
        if !x.starts_with("#") {
            let line = x;
            let linesplit: Vec<_> = line.split("|").collect::<Vec<_>>();
            medgenpubmed.push(MedgenPubMed {
                uid: linesplit[0].clone().to_string(),
                cui: linesplit[1].clone().to_string(),
                name: linesplit[2].clone().to_string(),
                pmid: linesplit[3].clone().to_string(),
            });
        }
    });
    Ok(medgenpubmed)
}
