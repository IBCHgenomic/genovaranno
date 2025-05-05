# genovaranno

![](https://github.com/IBCHgenomic/eVaiutilities/blob/main/logo.png)

- Parallel threaded variant annotation. For graph uses dioxus.
- Each command can be used separately to prepapre the JSON.
- A single merged command for the complete annotation allowing the user to do complete annotation of the vcf files.
- Database download and upgradation via dotenv.
- **Note:** RUST Binaries will be tagged for the final release version else development version and should not be used

```
 cargo build
```

```
Variant annotator for human genomics.
       ************************************************
      Gaurav Sablok, IBCH, PAN, Poznan, Poland,
      https://portal.ichb.pl/laboratory-of-genomics/.
      Email: gsablok@ibch.poznan.pl
      Prof. Luiza Handschuh
      Email: luizahan@ibch.poznan.pl.
      ************************************************

Usage: genovaranno <COMMAND>

Commands:
  cuigenerate          prepapre the CUI for the medgen
  omim                 OMIM and Evidence Annotator
  clinvaromimevidence  clinicvar OMIM and Evidence annotator
  ncbiannotate         NCBI gene annotate
  annotator            Multistage annotation linker
  vcfclinvarannotate   annotate vcf to clinvar and medgen
  phenotypelinker      Phenotype associations
  databases            Download databases
  help                 Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

- Acknowledgements: MOSAIC platform, developed as part of the ECBiG-MOSAIC project (POIR.04.02.00-00-D017/20), co-financed by the European Regional Development Fund (ERDF) under the Smart Growth Operational Programme 2014-2020, Measure 4.2 for the development of modern research infrastructure in the science sector.
- Project PI and Informal queries: Prof. Luiza Handschuh: luizahan@ibch.poznan.pl.
- Code related queries: Dr. Gaurav Sablok: gsablok@ibch.poznan.pl.

Gaurav Sablok Instytut Chemii Bioorganicznej Polskiej Akademii Nauk ul. Noskowskiego 12/14 | 61-704, Poznań Poland
