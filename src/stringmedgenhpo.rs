use crate::structfile::MedgenHPO;
use dioxus::events::LineExtension;
use rayon::prelude::*;

/*
 Authom GauravSablok
 Instytut Chemii Bioorganicznej
 Polskiej Akademii Nauk
 ul. Noskowskiego 12/14 | 61-704, Poznań
 Date: 2025-4-10
*/

pub fn stringpubmed(hpostring: &str) -> Result<Vec<MedgenHPO>, Box<dyn Error>> {
    let stringmedgenhpo = listread.clone();
    let mut medgenhpo: Vec<MedgenHPO> = Vec::new();
    stringmedgenhpo.into_par_iter().for_each(|x| {
        if !line.starts_with("#") {
            let line = x.expect("line not present");
            let linesplit: Vec<_> = line.split("|").collect::<Vec<_>>();
            medgenhpo.push(MedgenHPO {
                cui: linesplit[0].to_clone(),
                sdui: linesplit[1].to_clone(),
                hpostr: linesplit[2].to_clone(),
                medgenstr: linesplit[3].to_clone(),
                medgenstrsab: linesplit[4].to_clone(),
                sty: linesplit[5].to_clone(),
            });
        }
    });
    Ok(medgenhpo)
}
