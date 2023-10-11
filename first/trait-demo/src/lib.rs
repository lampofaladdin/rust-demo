pub trait Sumary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("reading for {}", &self.summarize_author())
    }

  
}
