use serde::{Deserialize, Serialize};
use serde_json::{self};
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};

// structs
#[derive(Serialize, Deserialize, Debug)]
struct Message {
    id: u64,
    content: String,
}

struct MessageQueue {
    file_path: String,
    offset_file_path: String,
}

impl MessageQueue {
    fn new(file_path: &str, offset_file_path: &str) -> Self {
        // constructor
        Self {
            file_path: file_path.to_string(),
            offset_file_path: offset_file_path.to_string(),
        }
    }

    fn read_message(&self, offset: u64) -> std::io::Result<Message> {
        let mut file = OpenOptions::new().read(true).open(&self.file_path)?;
        file.seek(SeekFrom::Start(offset))?; // this reads from offset onwards.
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        let message = serde_json::from_slice(&buffer)?;
        Ok(message)
    }
}

// functions
fn write_message(file_path: &str, message: &Message) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(file_path)?;
    // serialise message
    let serialised = serde_json::to_vec(message)?;

    file.write_all(&serialised)?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    let file_path = "messages.json";
    let offset_file_path = "offsets.txt";
    let message_q = MessageQueue::new(file_path, offset_file_path);

    // write message
    let message = Message {
        id: 1,
        content: "Hello World!".to_string(),
    };
    write_message(&file_path, &message)?;

    let offset = 0;
    let read_message = message_q.read_message(offset)?;
    println!("message output: {:?}", read_message);
    Ok(())
}
