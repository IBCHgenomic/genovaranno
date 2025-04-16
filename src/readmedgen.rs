use crate::structfile::MedgenMap;
use rayon::prelude::*;
use sonic_rs::JsonPointer;
use std::error::Error;
use std::io::{BufRead, BufReader};
/*
 Authom GauravSablok
 Instytut Chemii Bioorganicznej
 Polskiej Akademii Nauk
 ul. Noskowskiego 12/14 | 61-704, Poznań
 Date: 2025-4-16
*/

pub fn medgenmapmap(pubmedstring: &str) -> Result<Vec<Vec<MedgenMap>>, Box<dyn Error>> {
    let fileopen = std::fs::File::open(pubmedstring).expect("file not found");
    let mut returnvector: Vec<Vec<_>> = Vec::new();
    let fileread = BufReader::new(fileopen);
    returnvector.push(
        fileread
            .lines()
            .filter_map(|line: Result<String, _>| line.ok())
            .par_bridge()
            .flat_map(|x| mapiter(x))
            .collect::<Vec<_>>(),
    );
    let mut finaljson: Vec<MedgenMap> = Vec::new();
    for i in returnvector.iter() {
        for val in 0..i.len() {
            finaljson.push(MedgenMap {
                cuiid: i[0].clone().to_string(),
                prefname: i[1].clone().to_string(),
                sourceid: i[2].clone().to_string(),
                source: i[3].clone().to_string(),
            });
        }
    }
    Ok(finaljson)
}
pub fn mapiter(lineread: String) -> Result<Vec<String>, Box<dyn Error>> {
    let mut medgenpubmed: Vec<_> = Vec::new();
    let line = lineread.clone();
    if !line.starts_with("#") {
        let linesplit: Vec<_> = line.split("|").map(String::from).collect::<Vec<_>>();
        medgenpubmed.push(linesplit);
    }
    Ok(medgenpubmed)
}
