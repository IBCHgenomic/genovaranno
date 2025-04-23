use clap::{Parser, Subcommand};
#[derive(Debug, Parser)]
#[command(
    name = "VarAntAnno",
    version = "1.0",
    about = "Variant annotator for human genomics.
       ************************************************
      Gaurav Sablok, IBCH, PAN, Poznan, Poland,
      https://portal.ichb.pl/laboratory-of-genomics/.
      Email: gsablok@ibch.poznan.pl
      Prof. Luiza Handschuh
      Email: luizahan@ibch.poznan.pl.
      ************************************************"
)]
pub struct CommandParse {
    /// subcommands for the specific actions
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// prepapre the CUI for the medgen
    CUIgenerate {
        /// medgen HPO file
        medgenhpo: String,
        /// medgen OMIM file
        medgen_omim: String,
        /// medgen mapping
        medgenmapping: String,
        /// medgen pubmed
        medgenpubmed: String,
    },
    /// OMIM and Evidence Annotator
    OMIM {
        /// generate the link to OMIM and NCBI
        omimfile: String,
        /// evidence number
        evidencenumber: String,
        /// HPO mapping
        hpomapping: String,
        /// HPO megdgen file
        hpomedgen: String,
    },
    /// clinicvar OMIM and Evidence annotator
    ClinVarOMIMEvidence {
        /// provide the clinicavar file
        clinvar: String,
        /// provide the medgen file
        medgen: String,
        /// provide the medgenhpo file
        medgenhpo: String,
        /// provide the OMIM number
        omim: String,
    },
    /// NCBI gene annotate
    NCBIAnnotate {
        /// provide the ncbigene id file
        ncbigeneid: String,
        /// provide the clinvar file
        clinvar: String,
        /// provide the medgenomim file
        medgenomim: String,
        /// provide the medgenhpo file
        medgenhpo: String,
        /// provide the OMIM number
        omimsearch: String,
    },
    /// Multistage annotation linker
    Annotator {
        ///maxo annotations file
        pathncbimaxo: String,
        /// provide the medgenomim file
        medgenomim: String,
        /// provide the medgenhpo file
        medgenhpo: String,
        /// provide the evidence number
        evidence: String,
    },
}
