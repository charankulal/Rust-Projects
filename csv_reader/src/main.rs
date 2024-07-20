use csv;
use std::error::Error;
use std::env::args;

fn read_from_file(path: &str)->Result<(), Box<dyn Error>>{
    let mut reader= csv::Reader::from_path(path)?;
    
    for result in reader.records(){
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn main() {
    if args().len() != 2 {
        eprintln!("Usage: `source filename`\nExample: cargo run filename.csv");
        return;
    }
    let args=args().nth(1).unwrap();

    if args.len()<2 {
        println!("Usage: {} <filename>", args);
        return;
    }

    let fname= args.as_str();
    if let Err(e) = read_from_file(fname){
        eprintln!("{}",e);
    }
}
