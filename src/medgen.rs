use crate::stringhpoomim;
use crate::stringhpoomim::stringpoomim;
use crate::stringmedgenhpo::stringhpo;
use crate::stringmedgenpubmed::stringpubmed;
use crate::stringreadmedgen::stringmedgen;
use crate::structfile::CUIJSON;
use crate::structfile::HPOOMIM;
use crate::structfile::MedgenHPO;
use crate::structfile::MedgenMap;
use crate::structfile::MedgenPubMed;
use rayon::prelude;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};

/*
 Authom GauravSablok
 Instytut Chemii Bioorganicznej
 Polskiej Akademii Nauk
 ul. Noskowskiego 12/14 | 61-704, Poznań
 Date: 2025-4-9
*/

pub fn cuiparallel(
    medgenhpo: &str,
    medgen_omim: &str,
    medgenmapping: &str,
    medgenpubmed: &str,
) -> Result<String, Box<dyn Error>> {
    let medgenfile = File::open(medgenhpo).expect("file not present");
    let medgenomim = File::open(medgen_omim).expect("file not present");
    let medgenmapping = File::open(medgenmapping).expect("file not present");
    let medgenpubmedopen = File::open(medgenpubmedopen).expect("file not present");

    let medgenfileread = BufReader::new(medgenfile);
    let medgenominread = BufReader::new(medgenomim);
    let medgenmappingread = BufReader::new(medgenmapping);
    let medgenpubmedread = BufReader::new(medgenpubmedopen);

    let mut medgenvec_vec: Vec<_> = Vec::new();
    let mut medgenomim_vec: Vec<_> = Vec::new();
    let mut medgenmapping_vec: Vec<_> = Vec::new();
    let mut medgenpubmedopen_vec: Vec<_> = Vec::new();

    for i in medgenfileread.lines() {
        let line = i.expect("line not present");
        medgenvec_vec.push(line.clone());
    }

    for i in medgenominread.lines() {
        let line = i.expect("line not present");
        medgenomim_vec.push(line.clone());
    }

    for i in medgenmappingread.lines() {
        let line = i.expect("line not present");
        medgenmapping_vec.push(line.clone());
    }

    for i in medgenpubmedread.lines() {
        let line = i.expect("line not present");
        medgenpubmedopen_vec.push(line.clone());
    }

    let finalmedgenfile = stringhpo(medgenvec_vec).unwrap();
    let finalmgenomim = stringpoomim(medgenomim_vec).unwrap();
    let finalmedgenmapping = stringmedgen(medgenmapping_vec).unwrap();
    let finalmedgenpubmed = stringpubmed(medgenpubmedopen_vec).unwrap();

    Ok("The serialization for the CUi has been written".to_string())
}
