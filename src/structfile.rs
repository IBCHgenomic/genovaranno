#[derive(Debug, Clone, PartialOrd, PartialEq)]

pub struct CUIJSON {
    pub sdui: String,
    pub hpostr: String,
    pub medgenstr: String,
    pub medgenstr_sab: String,
    pub sty: String,
    pub relationship: String,
    pub hpocui: String,
    pub mimnumber: String,
    pub omimnumber: String,
    pub hpoid: String,
    pub medgenname: String,
    pub medgensource: String,
    pub prefname: String,
    pub sourceid: String,
    pub source: String,
    pub uid: String,
    pub name: String,
    pub pmid: String,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct MedgenHPO {
    pub cui: String,
    pub sdui: String,
    pub hpostr: String,
    pub medgenstr: String,
    pub medgenstrsab: String,
    pub sty: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct HPOOMIM {
    pub omimcui: String,
    pub mimnumber: String,
    pub omimname: String,
    pub relationship: String,
    pub hpocui: String,
    pub hpoid: String,
    pub hponame: String,
    pub medgenname: String,
    pub medgensource: String,
    pub sty: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct MedgenMap {
    pub cuiid: String,
    pub prefname: String,
    pub sourceid: String,
    pub source: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct MedgenPubMed {
    pub uid: String,
    pub cui: String,
    pub name: String,
    pub pmid: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct OMIMEvidence {
    pub databaseid: String,
    pub diseasename: String,
    pub qualifier: String,
    pub hpoid: String,
    pub reference: String,
    pub evidence: String,
    pub onset: String,
    pub frequency: String,
    pub sex: String,
    pub modifier: String,
    pub aspect: String,
    pub biocuration: Vec<String>,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct OMIMEvidenceMerged {
    pub databaseid: String,
    pub diseasename: String,
    pub qualifier: String,
    pub hpoid: String,
    pub reference: String,
    pub evidence: String,
    pub onset: String,
    pub frequency: String,
    pub sex: String,
    pub modifier: String,
    pub aspect: String,
    pub biocuration: Vec<String>,
    pub cui: String,
    pub sdui: String,
    pub hpostr: String,
    pub medgenstr: String,
    pub medgenstrsab: String,
    pub omimcui: String,
    pub mimnumber: String,
    pub omimname: String,
    pub relationship: String,
    pub hpocui: String,
    pub hponame: String,
    pub medgenname: String,
    pub medgensource: String,
    pub sty: String,
}
