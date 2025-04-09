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
    Ok("The serialization for the CUi has been written".to_string())
}
