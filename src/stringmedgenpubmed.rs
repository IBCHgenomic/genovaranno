use crate::structfile::MedgenPubMed;
use rayon::prelude::*;

pub fn stringpubmed(pubmedstring: &str) -> Result<Vec<MedgenPubMed>, Box<dyn Error>> {
    let stringpubmed = listread.clone();
    let mut medgenpubmed: Vec<MedgenPubMed> = Vec::new();
    stringpubmed.into_par_iter().for_each(|x| {
        let line = x.expect("line not present");
        let linesplit: Vec<_> = line.split("|").collect::<Vec<_>>();
        medgenpubmed.push(MedgenPubMed {
            uid: linesplit[0].to_clone(),
            cui: linesplit[1].to_clone(),
            name: linesplit[2].to_clone(),
            pmid: linesplit[3].to_clone(),
        })
    })
}
