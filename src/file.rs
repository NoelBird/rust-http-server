use std::{fs, io};
use std::io::Write;
use flate2::Compression;
use flate2::write::GzEncoder;

pub(crate) fn read_file(path: String) -> io::Result<Vec<u8>> {
    fs::read(path)
}

pub(crate) fn write_file(path: String, contents: &[u8]) -> io::Result<()> {
    fs::write(path, &contents)
}

pub(crate) fn gzip_encode(buffer: Vec<u8>) -> io::Result<Vec<u8>> {
    let mut out_buffer = Vec::new();

    // GzEncoder 초기화
    let gz_encoder = GzEncoder::new(&mut out_buffer, Compression::default());

    // BufWriter로 감싸기
    let mut out_buf = io::BufWriter::new(gz_encoder);

    // 데이터를 압축 스트림에 쓰기
    out_buf.write_all(&buffer)?;

    // flush()로 BufWriter에 남아 있는 데이터를 비움
    out_buf.flush()?;

    // GzEncoder가 완전히 종료되도록 finish() 호출
    out_buf.into_inner()?.finish()?;

    Ok(out_buffer)
}
