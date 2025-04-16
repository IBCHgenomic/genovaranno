use crate::structfile::MedgenHPO;
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

pub fn medgenhpomap(pubmedstring: &str) -> Result<Vec<Vec<MedgenHPO>>, Box<dyn Error>> {
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

    let mut finaljson: Vec<MedgenHPO> = Vec::new();
    for i in returnvector.iter() {
        for val in 0..i.len() {
            finaljson.push(MedgenHPO {
                cui: i[0].clone().to_string(),
                sdui: i[1].clone().to_string(),
                hpostr: i[2].clone().to_string(),
                medgenstr: i[3].clone().to_string(),
                medgenstrsab: i[4].clone().to_string(),
                sty: i[5].clone().to_string(),
            });
        }
    }
    Ok(finaljson)
}

pub fn mapiter(lineread: String) -> Result<Vec<String>, Box<dyn Error>> {
    let mut medgenhpo: Vec<_> = Vec::new();
    let line = lineread.clone();
    if !line.starts_with("#") {
        let linesplit: Vec<_> = line.split("|").map(String::from).collect::<Vec<_>>();
        medgenhpo.push(linesplit);
    }
    Ok(medgenhpo)
}
