use clap::Parser;
use x509_parser::pem::Pem;
use x509_parser::prelude::X509Certificate;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // The parth to the PEM file to install
    #[arg(short, long)]
    pem_file: String
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let pem_file = args.pem_file;
    println!("Installing {}", pem_file);

    let mut pem_file = std::fs::read(pem_file)?;
    for pem in Pem::iter_from_buffer(&mut pem_file) {
        for cert_req in pem.iter().map(|p| p.parse_x509()) {
            if let Ok(cert) = cert_req {
                install_cert(&cert)?;
            }
        }
    }

    Ok(())
}

fn install_cert(cert: &X509Certificate) -> anyhow::Result<()> {
    println!();
    Ok(())
}
