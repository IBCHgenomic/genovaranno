use crate::structfile::GenePhenotype;
use crate::structfile::NCBIgene;
use crate::structfile::Phenotype;
use crate::structfile::PhenotypeHPOA;
use crate::structfile::PhenotypeMerged;
use rayon::prelude::*;
use std::error::Error;
use std::io::{BufRead, BufReader};

/*
 Author Gaurav Sablok
 Instytut Chemii Bioorganicznej
 Polskiej Akademii Nauk
 ul. Noskowskiego 12/14 | 61-704, Poznań
 Date: 2025-4-22
*/

pub fn phenotypeall(
    genesdisease: &str,
    genesphenotype: &str,
    phenotypehpoa: &str,
    phenotypesgenes: &str,
) -> Result<Vec<PhenotypeMerged>, Box<dyn Error>> {
    let genediseaseopen = std::fs::File::open(genesdisease).expect("file not found");
    let genediseaseread = BufReader::new(genediseaseopen);
    let gendiseasevector: Vec<Vec<_>> = genediseaseread
        .lines()
        .filter_map(|line: Result<String, _>| line.ok())
        .par_bridge()
        .map(|x| genesdiseasemapper(x).unwrap())
        .collect::<Vec<_>>();

    let genesphenoopen = std::fs::File::open(genesphenotype).expect("file not found");
    let genesphenoread = BufReader::new(genesphenoopen);
    let genesphenovector: Vec<Vec<_>> = genesphenoread
        .lines()
        .filter_map(|line: Result<String, _>| line.ok())
        .par_bridge()
        .map(|x| genesphenotypemapper(x).unwrap())
        .collect::<Vec<_>>();

    let phenohpoaopen = std::fs::File::open(phenotypehpoa).expect("file not found");
    let phenohpoaread = BufReader::new(phenohpoaopen);
    let phenohpoavector: Vec<Vec<_>> = phenohpoaread
        .lines()
        .filter_map(|line: Result<String, _>| line.ok())
        .par_bridge()
        .map(|x| phenotypehpoamapper(x).unwrap())
        .collect::<Vec<_>>();

    let phenotypeopen = std::fs::File::open(phenotypesgenes).expect("file not found");
    let phenotyperead = BufReader::new(phenotypeopen);
    let phenotypevector: Vec<Vec<_>> = phenotyperead
        .lines()
        .filter_map(|line: Result<String, _>| line.ok())
        .par_bridge()
        .map(|x| phenotypeallmapper(x).unwrap())
        .collect::<Vec<_>>();

    let mut gendivector: Vec<NCBIgene> = Vec::new();
    let mut genphenvector: Vec<GenePhenotype> = Vec::new();
    let mut phenhpoavector: Vec<PhenotypeHPOA> = Vec::new();
    let mut phenotypvector: Vec<Phenotype> = Vec::new();

    for i in gendiseasevector.iter(){
        for val in i.iter(){
            gendivector.push(NCBIgene {
                ncbigeneid: val.ncbigeneid.clone(),
                genesymbol: val.genesymbol.clone(),
                association: val.genesymbol.clone(),
                disease: val.disease.clone(),
            });
        }
    }



    Ok(gendivector)

}

pub fn genesdiseasemapper(lineread: String) -> std::io::Result<Vec<NCBIgene>> {
    let mut ncbigene: Vec<NCBIgene> = Vec::new();
    let line = lineread.clone();
    if !line.starts_with("#") {
        let linesplit: Vec<_> = line.split("\t").map(String::from).collect::<Vec<_>>();
        ncbigene.push(NCBIgene {
            ncbigeneid: linesplit[0].to_string(),
            genesymbol: linesplit[1].to_string(),
            association: linesplit[2].to_string(),
            disease: linesplit[3].to_string().split(":").collect::<Vec<_>>()[1]
                .parse::<usize>()
                .unwrap(),
        });
    }
    Ok(ncbigene)
}

pub fn genesphenotypemapper(lineread: String) -> std::io::Result<Vec<GenePhenotype>> {
    let mut genepheno: Vec<GenePhenotype> = Vec::new();
    let line = lineread.clone();
    if !line.starts_with("#") {
        let linesplit: Vec<_> = line.split("\t").map(String::from).collect::<Vec<_>>();
        genepheno.push(GenePhenotype {
            ncbigeneid: linesplit[0].to_string(),
            genesymbol: linesplit[1].to_string(),
            hpoid: linesplit[2].to_string().split(":").collect::<Vec<_>>()[1]
                .parse::<usize>()
                .unwrap(),
            hponame: linesplit[3].to_string(),
            frequency: linesplit[4].to_string(),
            diseaseid: linesplit[5].to_string().split(":").collect::<Vec<_>>()[1]
                .parse::<usize>()
                .unwrap(),
        });
    }
    Ok(genepheno)
}

pub fn phenotypehpoamapper(lineread: String) -> std::io::Result<Vec<PhenotypeHPOA>> {
    let mut phenohpoa: Vec<PhenotypeHPOA> = Vec::new();
    let line = lineread.clone();
    if !line.starts_with("#") {
        let linesplit: Vec<_> = line.split("\t").map(String::from).collect::<Vec<_>>();
        phenohpoa.push(PhenotypeHPOA {
            databaseid: linesplit[0].to_string(),
            diseasename: linesplit[1].to_string(),
            qualifier: linesplit[2].to_string(),
            hpoid: linesplit[3].to_string().split(":").collect::<Vec<_>>()[1]
                .parse::<usize>()
                .unwrap(),
            reference: linesplit[3].to_string(),
            evidence: linesplit[4].to_string(),
            onset: linesplit[5].to_string(),
            frequency: linesplit[6].to_string(),
            sex: linesplit[7].to_string(),
            modifier: linesplit[8].to_string(),
            aspect: linesplit[9].to_string(),
            biocuration: linesplit[10].to_string(),
        });
    }
    Ok(phenohpoa)
}

pub fn phenotypeallmapper(lineread: String) -> std::io::Result<Vec<Phenotype>> {
    let mut phenpo: Vec<Phenotype> = Vec::new();
    let line = lineread.clone();
    if !line.starts_with("#") {
        let linesplit: Vec<_> = line.split("\t").map(String::from).collect::<Vec<_>>();
        phenpo.push(Phenotype {
            hponameid: linesplit[0].to_string().split(":").collect::<Vec<_>>()[1]
                .parse::<usize>()
                .unwrap(),
            hponame: linesplit[1].to_string(),
            ncbi_geneid: linesplit[2].to_string(),
            genesymbol: linesplit[3].to_string(),
            diseaseidpheno: linesplit[5].to_string().split(":").collect::<Vec<_>>()[1]
                .parse::<usize>()
                .unwrap(),
        });
    }
    Ok(phenpo)
}
