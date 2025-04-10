use crate::structfile::MedgenPubMed;
use rayon::prelude::*;

/*
 Authom GauravSablok
 Instytut Chemii Bioorganicznej
 Polskiej Akademii Nauk
 ul. Noskowskiego 12/14 | 61-704, Poznań
 Date: 2025-4-10
*/

pub fn stringpubmed(pubmedstring: &str) -> Result<Vec<MedgenPubMed>, Box<dyn Error>> {
    let stringpubmed = listread.clone();
    let mut medgenpubmed: Vec<MedgenPubMed> = Vec::new();
    stringpubmed.into_par_iter().for_each(|x| {
        if !line.starts_with("#") {
            let line = x.expect("line not present");
            let linesplit: Vec<_> = line.split("|").collect::<Vec<_>>();
            medgenpubmed.push(MedgenPubMed {
                uid: linesplit[0].to_clone(),
                cui: linesplit[1].to_clone(),
                name: linesplit[2].to_clone(),
                pmid: linesplit[3].to_clone(),
            });
        }
    });
    Ok(medgenpubmed)
}
