use crate::structfile::HPOOMIM;
use rayon::prelude::*;
use std::error::Error;
use std::io::{BufRead, BufReader};
/*
 Authom GauravSablok
 Instytut Chemii Bioorganicznej
 Polskiej Akademii Nauk
 ul. Noskowskiego 12/14 | 61-704, Poznań
 Date: 2025-4-10
*/

pub fn hpoomimmap(hpo: &str) -> Result<Vec<HPOOMIM>, Box<dyn Error>> {
    let fileopen = std::fs::File::open(hpo).expect("file not found");
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
    let mut finaljson: Vec<HPOOMIM> = Vec::new();
    for i in 0..returnvector.len() {
            finaljson.push(HPOOMIM {
                omimcui: returnvector[i][0].to_string().clone(),
                mimnumber: returnvector[i][1].to_string().clone(),
                omimname: returnvector[i][2].to_string().clone(),
                relationship: returnvector[i][3].to_string().clone(),
                hpocui: returnvector[i][4].to_string().clone(),
                hpoid: returnvector[i][5].to_string().clone(),
                hponame: returnvector[i][6].to_string().clone(),
                medgenname: returnvector[i][7].to_string().clone(),
                medgensource: returnvector[i][8].to_string().clone(),
                sty: returnvector[i][9].to_string().clone(),
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
