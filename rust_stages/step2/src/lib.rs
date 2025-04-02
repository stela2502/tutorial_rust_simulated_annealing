use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead};
use rand::Rng;

#[allow(dead_code)]
pub struct SimulatedAnnealing {
    /// the normalized expression data
    pub data: Vec<Vec<f32>>,
    /// the names for each row
    rownames: Vec<String>,
    /// the cluter count
    k: usize,
    /// the cluster ids for each row of data
    clusters: Vec<usize>,
    /// the last cluster energies
    clusters_energy: Vec<f32>,
    /// the actual temerature
    temp:f32,

}

impl SimulatedAnnealing {
    pub fn new( file_path:&str, k:usize, temp:f32, split:char ) -> Self{

        let (rownames, data) = Self::read_table_with_names( file_path, split ).unwrap();

        let clusters: Vec<usize> = rownames.iter().map(|_| rand::rng().random_range(0..k)).collect();
        let _n = rownames.len();
        Self{
            data,
            rownames,
            k,
            clusters,
            clusters_energy: vec![0.0; k ],
            temp,
        }
    }
    pub fn read_table_with_names(file_path: &str, split: char ) -> Result<( Vec<String>, Vec<Vec<f32>>), String> {
        let path = Path::new(file_path);
        let file = match File::open(path){
          Ok(f) => f,
          Err(e) => {
            return Err( format!("Failed to open file: {}", e) )
          }
        };
        let reader = BufReader::new(file);

        let mut data = Vec::new();
        let mut rownames= Vec::new();

        for (line_num, line) in reader.lines().enumerate() {
            let line = line.map_err(|e| format!("Error reading line {}: {}", line_num + 1, e))?;
            let mut parts = line.split( split );

            let row_name = parts.next().ok_or_else(|| format!("Missing row name at line {}", line_num + 1))?;
            if row_name == "" {
                // ignore column names
                continue;
            }
            rownames.push( row_name.to_string() );

            let values: Result<Vec<f32>, String> = parts
                .map(|num| num.parse::<f32>().map_err(|_| format!("Invalid number '{}' at line {}", num, line_num + 1)))
                .collect();

            let values = values.unwrap(); // will die on error
            data.push( values );
        }
        Ok((rownames, data ) )
    }


}



#[cfg(test)]
mod tests {
    use super::*; // Import everything from the parent module

    #[test]
    fn test_read_data() {
        match SimulatedAnnealing::read_table_with_names( "tests/data/Spellman_Yeast_Cell_Cycle.tsv", '\t' ){
            Ok((rownames, data)) => {
                assert_eq!(data.len(), 256, "we have 256 rows");
                assert_eq!(data[0].len(), 16, "we have 16 cols");
                assert_eq!(rownames.len(), data.len(), "rownames and data have the same dimension (isch)")
            }Err(e) =>{
                panic!("Could not read the tsv file! : {e}");
            }
        }
    }
}

