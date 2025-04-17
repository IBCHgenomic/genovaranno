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
}
