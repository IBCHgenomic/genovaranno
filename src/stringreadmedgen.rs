use crate::structfile::MedgenMap;
use rayon::prelude::*;

/*
 Authom GauravSablok
 Instytut Chemii Bioorganicznej
 Polskiej Akademii Nauk
 ul. Noskowskiego 12/14 | 61-704, Poznań
 Date: 2025-4-10
*/

pub fn stringmedgen(listread: &str) -> Result<Vec<MedgenMap>, Box<dyn Error>> {
    let stringhold = listread.clone();
    let mut medgenmap: Vec<MedgenMap> = Vec::new();
    stringhold.into_par_iter().for_each(|x| {
        if !line.starts_with("#") {
            let line = x.expect("line not present");
            let linesplit: Vec<_> = line.split("|").collect::<Vec<_>>();
            medgenmap.push(MedgenMap {
                cuiid: linesplit[0].to_clone(),
                prefname: linesplit[1].to_clone(),
                sourceid: linesplit[2].to_clone(),
                source: linesplit[3].to_clone(),
            });
        }
    });
    Ok(medgenmap)
}
