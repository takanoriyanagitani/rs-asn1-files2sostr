use std::io;

use io::BufRead;
use io::Read;

use io::BufWriter;
use io::Write;

use std::fs::File;

use der::asn1::OctetString;

use der::Encode;

pub fn bytes2ostr(b: Vec<u8>) -> Result<OctetString, io::Error> {
    OctetString::new(b).map_err(io::Error::other)
}

pub fn osvalues2der2wtr<W>(ovalues: &Vec<OctetString>, mut wtr: W) -> Result<(), io::Error>
where
    W: Write,
{
    ovalues.encode(&mut wtr).map_err(io::Error::other)
}

pub fn filenames2ovalues2der2writer<I, W>(
    filenames: I,
    mut wtr: W,
    limit: u64,
) -> Result<(), io::Error>
where
    I: Iterator<Item = String>,
    W: Write,
{
    let mut ovalues: Vec<OctetString> = vec![];
    for filename in filenames {
        let f: File = File::open(filename)?;
        let mut limited = f.take(limit);
        let mut v: Vec<u8> = vec![];
        limited.read_to_end(&mut v)?;
        let o: OctetString = bytes2ostr(v)?;
        ovalues.push(o);
    }
    osvalues2der2wtr(&ovalues, &mut wtr)?;
    wtr.flush()
}

pub fn stdin2filenames() -> impl Iterator<Item = String> {
    let i = io::stdin();
    let l = i.lock();
    let lines = l.lines();
    lines.map_while(Result::ok)
}

pub const FILESIZE_LIMIT_DEFAULT: u64 = 1048576;

pub fn stdin2names2ovalues2der2stdout(limit: u64) -> Result<(), io::Error> {
    let names = stdin2filenames();
    let o = io::stdout();
    let mut l = o.lock();

    let bw = BufWriter::new(&mut l);
    filenames2ovalues2der2writer(names, bw, limit)?;

    l.flush()
}

pub fn stdin2names2ovalues2der2stdout_default() -> Result<(), io::Error> {
    stdin2names2ovalues2der2stdout(FILESIZE_LIMIT_DEFAULT)
}
