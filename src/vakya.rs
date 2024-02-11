use crate::VarnList;
use crate::Shabd;
use std::fs::File;
use std::io::{self, BufRead};

pub struct Vakya {
    pub shabds: Vec<Shabd>,
}

impl Vakya {
    pub fn from_line(line: &str) -> Result<Self, io::Error> {
        let mut shabds = Vec::new();

        for word in line.split_whitespace() {
            let varnlist = match VarnList::parse_word(word) {
                Ok(varnlist) => varnlist,
                Err(e) => return Err(io::Error::new(io::ErrorKind::Other, format!("Failed to parse word: {}", e))),
            };

            let mut s = Shabd::new_from_list(varnlist);
            s.make_akshars();
            s.get_matra();

            shabds.push(s);
        }

        Ok(Vakya { shabds })
    }

    pub fn from_file(file_path: &str) -> Result<Vec<Vakya>, io::Error> {
        let file = File::open(file_path)?;
        let reader = io::BufReader::new(file);

        let mut vakyas = Vec::new();

        for line in reader.lines() {
            let line = line?;
            let vakya = Vakya::from_line(&line)?;
            vakyas.push(vakya);
        }

        Ok(vakyas)
    }

    pub fn print(&mut self) {
        for s in self.shabds.iter() {
            s.print();
        }
    }
}
