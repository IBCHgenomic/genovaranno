use crate::structfile::GenePhenotype;
use crate::structfile::Phenotype;
use crate::structfile::PhenotypeHPOA;
use crate::structfile::PhenotypeMerged;
use rayon::prelude::*;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

/*
 Author Gaurav Sablok
 Instytut Chemii Bioorganicznej
 Polskiej Akademii Nauk
 ul. Noskowskiego 12/14 | 61-704, Poznań
 Date: 2025-4-22
*/

pub fn phenotype(
    genesdisease: &str,
    genesphenotype: &str,
    phenotypehpoa: &str,
    phenotypesgenes: &str,
) -> Result<Vec<PhenotypeMerged>, Box<dyn Error>> {
    let fileopen = std::fs::File::open(pathncbi).expect("file not found");
    let fileread = BufReader::new(fileopen);
    let returnvector: Vec<Vec<_>> = fileread
        .lines()
        .filter_map(|line: Result<String, _>| line.ok())
        .par_bridge()
        .map(|x| mapiter(x).unwrap())
        .collect::<Vec<_>>();
    let mut finaloutlay: Vec<Ontology> = Vec::new();
}
