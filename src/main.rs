use std::fs::File;
use std::os::unix::fs::FileExt;
use thiserror::Error;

const MAX_RECORD: usize = 16 * 1024 * 1024;

#[derive(Debug, Error)]
pub enum LogError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("corrupt record at byte {pos}")]
    Corrupt { pos: u64 },

    #[error("offset {0} is out of range")]
    OffsetOutOfRange(u64),
}

pub fn read_record(file: &File, pos: u64) -> Result<Vec<u8>, LogError> {
    let mut header = [0u8; 8];
    file.read_exact_at(&mut header, pos)?;

    let len = u32::from_le_bytes([header[0], header[1], header[2], header[3]]) as usize;
    let crc = u32::from_le_bytes([header[4], header[5], header[6], header[7]]);

    if len > MAX_RECORD {
        return Err(LogError::Corrupt { pos });
    }

    let mut payload = vec![0u8; len];
    file.read_exact_at(&mut payload, pos + 8)?;

    if crc32fast::hash(&payload) != crc {
        return Err(LogError::Corrupt { pos });
    }
    Ok(payload)
}

fn main() {
    let pos = 10;
    let content = match read_record(&file, pos) {
        Ok(rec) => pos += 8 + rec.len() as u64,
        Err(LogError::Io(e)) if e.kind() == std::io::ErrorKind::UnexpectedEof => {
            file.set_len(pos)?; // torn tail: truncate and stop
            break;
        }
        Err(LogError::Corrupt { .. }) if is_last_segment => {
            file.set_len(pos)?;
            break;
        }
        Err(e) => return Err(e),
    }
}
