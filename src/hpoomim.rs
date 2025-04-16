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

pub fn hpoomimmap(hpo: &str) -> Result<Vec<Vec<HPOOMIM>>, Box<dyn Error>> {
    let fileopen = std::fs::File::open(hpo).expect("file not found");
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
    let mut finaljson: Vec<HPOOMIM> = Vec::new();
    for i in returnvector.iter() {
        for val in 0..i.len() {
            finaljson.push(HPOOMIM {
                omimcui: i[0].clone().to_string(),
                mimnumber: i[1].clone().to_string(),
                omimname: i[2].clone().to_string(),
                relationship: i[3].clone().to_string(),
                hpocui: i[4].clone().to_string(),
                hpoid: i[5].clone().to_string(),
                hponame: i[6].clone().to_string(),
                medgenname: i[7].clone().to_string(),
                medgensource: i[8].clone().to_string(),
                sty: i[9].clone().to_string(),
            });
        }

        Ok(finaljson)
    }
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
