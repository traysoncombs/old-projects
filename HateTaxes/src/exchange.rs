use super::account::*;
use super::coin::*;
use super::errors::*;
use super::transaction::*;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

pub trait Exchange {
    fn import_csv(&self, file: String) -> SpecialResult<bool> {
      let mut lines = self.open_file(file)?;
      let header = lines.next().unwrap().map_err(|err| {
          SpecialError::new(SpecialErrorKind::ErrorOpeningFile, err.to_string().as_str())
      })?;
      self.process_header(header)?;
      for line in lines.next() {
          match line {
              Ok(l) => self.process_line(l)?,
              Err(e) => break, // TODO: Error handling instead of just exiting lol
          }
      }
      Ok(true)
    }
    fn open_file(path: String) -> SpecialResult<Lines<BufReader<File>>> {
        let file = File::open(path).map_err(|err| {
            SpecialError::new(SpecialErrorKind::ErrorOpeningFile, "Error opening file")
        })?;
        Ok(BufReader::new(file).lines())
    }
    fn text_to_coin(coin_text: String) -> Coins;
    fn process_line(&self, line: String) -> SpecialResult<()>;
    fn process_header(&self, line: String) -> SpecialResult<bool>;
}
