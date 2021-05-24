pub mod helpers {
    use std::fs::File;
    use std::io::{self, prelude::*, BufReader};

    pub fn read_file(filename: String) -> io::Result<Vec<String>> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);

        let mut lines: Vec<String> = Vec::new();

        for line in reader.lines() {
            lines.push(line?);
        }

        Ok(lines)
    }

    pub fn run(search: String, lines: io::Result<Vec<String>>) -> Result<(), io::Error> {
        for line in lines? {
            if line.contains(&search) {
                println!("{}", line);
            }
        }
        Ok(())
    }
}
