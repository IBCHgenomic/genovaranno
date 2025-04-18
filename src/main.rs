mod args;
mod hpoomim;
mod medgen;
mod medgenhpo;
mod medgenpubmed;
mod omim;
mod readmedgen;
mod structfile;
use crate::args::CommandParse;
use crate::args::Commands;
use crate::medgen::cuiparallel;
use clap::Parser;

/*
 Authom GauravSablok
 Instytut Chemii Bioorganicznej
 Polskiej Akademii Nauk
 ul. Noskowskiego 12/14 | 61-704, Poznań
 Date: 2025-4-8
*/

fn main() {
    let argsparse = CommandParse::parse();
    match &argsparse.command {
        Commands::CUIgenerate {
            medgenhpo,
            medgen_omim,
            medgenmapping,
            medgenpubmed,
        } => {
            let command = cuiparallel(medgenhpo, medgen_omim, medgenmapping, medgenpubmed).unwrap();
            println!("The command has been completed");
        }
        Commands::OMIM { omimfile, evidencenumber, hpomapping, hpomedgen } => {
            let command = omimevidence(omimfile).unwrap();
            println!("The links for the given evidence are:{}", command);
        }
    }
}
