use std::process::ExitCode;

use std::io;

use rs_asn1_files2sostr::stdin2names2ovalues2der2stdout_default;

fn sub() -> Result<(), io::Error> {
    stdin2names2ovalues2der2stdout_default()
}

fn main() -> ExitCode {
    sub().map(|_| ExitCode::SUCCESS).unwrap_or_else(|e| {
        eprintln!("{e}");
        ExitCode::FAILURE
    })
}
