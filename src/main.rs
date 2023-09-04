use clap::Parser;
use mozdevice::{Device, Host, UnixPath};
use x509_parser::pem::Pem;
use x509_parser::prelude::X509Certificate;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // The path to the PEM file to install
    pem_file: String,
    // The path to the CA certificate directory of the system
    #[clap(long, default_value = "/system/etc/security/cacerts/")]
    cert_path: String,
    // The device serial number to use
    #[clap(long)]
    device_serial: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let pem_file = args.pem_file;
    println!("Installing {}", pem_file);

    println!("Finding ADB device");
    let host = Host::default();
    let device = host.device_or_default(
        args.device_serial.as_ref(),
        mozdevice::AndroidStorageInput::Auto,
    )?;
    println!("Found device {}", &device.serial);

    let pem_file = std::fs::read(pem_file)?;
    for pem in Pem::iter_from_buffer(&pem_file) {
        for cert in pem.iter().flat_map(|p| p.parse_x509()) {
            install_cert(&cert, &args.cert_path, &pem_file, &device)?;
        }
    }
    Ok(())
}

fn old_hash_encode(object: &[u8]) -> u32 {
    let md5_hash = md5::compute(object);
    let mut hash = [0u8; 4];
    hash.copy_from_slice(&md5_hash[..4]);
    u32::from_le_bytes(hash)
}

fn collision_aware_copy(
    base_name: &str,
    iter: u8,
    cert_file: &[u8],
    device: &Device,
) -> anyhow::Result<bool> {
    let cert_filename = format!("{}.{}", base_name, iter);
    let unix_path = UnixPath::new(&cert_filename);

    let mut cert_file = std::io::Cursor::new(cert_file);
    if !device.path_exists(unix_path, false)? {
        println!("Copying to {}", &cert_filename);
        device.push(&mut cert_file, unix_path, 0)?;
        return Ok(true);
    }

    let mut existing_cert = Vec::new();
    device.pull(unix_path, &mut existing_cert)?;

    if existing_cert == cert_file.into_inner() {
        anyhow::bail!("Certificate already installed");
    }

    println!("Collision detected, trying next iteration");

    Ok(false)
}

fn install_cert(
    cert: &X509Certificate,
    cert_path: &str,
    cert_bytes: &[u8],
    device: &Device,
) -> anyhow::Result<()> {
    println!("Installing certificate for subject {}", cert.subject());

    let md5_hash = old_hash_encode(cert.subject().as_raw());
    println!("Calculated MD5 hash for subject {:x}", &md5_hash);
    let cert_filename = format!("{}{:x}", cert_path, &md5_hash);
    let mut iteration = 0;
    while !collision_aware_copy(&cert_filename, iteration, cert_bytes, device)? {
        iteration += 1;
    }

    Ok(())
}
