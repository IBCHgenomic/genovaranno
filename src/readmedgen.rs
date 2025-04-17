use crate::structfile::MedgenMap;
use rayon::prelude::*;
use std::error::Error;
use std::io::{BufRead, BufReader};
/*
 Authom GauravSablok
 Instytut Chemii Bioorganicznej
 Polskiej Akademii Nauk
 ul. Noskowskiego 12/14 | 61-704, Poznań
 Date: 2025-4-16
*/

pub fn medgenmapmap(pubmedstring: &str) -> Result<Vec<MedgenMap>, Box<dyn Error>> {
    let fileopen = std::fs::File::open(pubmedstring).expect("file not found");
    let mut returnvector: Vec<Vec<_>> = Vec::new();
    let fileread = BufReader::new(fileopen);
    returnvector.push(
        fileread
            .lines()
            .filter_map(|line: Result<String, _>| line.ok())
            .par_bridge()
            .map(|x| {mapiter(x).unwrap();})
            .collect()
    );
    let mut finaljson: Vec<MedgenMap> = Vec::new();
    for i in 0..returnvector.len() {
            finaljson.push(MedgenMap {
                cuiid: returnvector[i][0].to_string().clone(),
                prefname: returnvector[i][1].to_string().clone(),
                sourceid: returnvector[i][2].to_string().clone(),
                source: returnvector[i][3].to_string().clone(),
            });
    }
    Ok(finaljson)
}
pub fn mapiter(lineread: String) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    let mut medgenpubmed: Vec<_> = Vec::new();
    let line = lineread.clone();
    if !line.starts_with("#") {
        let linesplit: Vec<_> = line.split("|").map(String::from).collect::<Vec<_>>();
        medgenpubmed.push(linesplit);
    }
    Ok(medgenpubmed)
}
