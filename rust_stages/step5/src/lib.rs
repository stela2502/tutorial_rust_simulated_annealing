use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead};
use rand::Rng;
use plotters::prelude::*;
use std::error::Error;
use std::io::Write;

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
    /// store to store the row row distances
    store:Vec<f32>

}

impl SimulatedAnnealing {
    pub fn new( file_path:&str, k:usize, temp:f32, split:char ) -> Self{

        let (rownames, data) = Self::read_table_with_names( file_path, split ).unwrap();

        let clusters: Vec<usize> = rownames.iter().map(|_| rand::rng().random_range(0..k)).collect();
        let n = rownames.len();
        Self{
            data,
            rownames,
            k,
            clusters,
            clusters_energy: vec![0.0; k ],
            temp,
            store: Vec::with_capacity(n * (n - 1) / 2), 
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
    
     pub fn scale_01( &mut self){
        for row in &mut self.data {
            let mut min = f32::INFINITY;
            let mut range = f32::NEG_INFINITY;

            for i in 0..row.len(){
                min = min.min(row[i]);
                range = range.max(row[i]);
           
            }
            range -= min;
            // Now apply the scaling using the min/max in place
            for entry in row {
                *entry = (*entry - min) / range;  // In-place scaling
            }
        }
        let n = self.data.len();
        for i in 0..n {
            for j in i + 1..n {
                let distance = self.euclidean_distance(i,j);
                self.store.push(distance);
            }
        }
    }
    /// Function to compute the Euclidean distance between rows of data
    fn euclidean_distance(&self, i: usize, j: usize ) -> f32 {
        let v1 = &self.data[i];
        let v2 = &self.data[j];
        let mut sum:f32 = 0.0;
        for i in 0..v1.len(){
            sum += (v1[i] - v2[i]).powi(2);
        }
        sum.sqrt()
    }
    
    /// Calculates the cluster energy
    fn calc_ek(&self, clus: usize) -> f32 {
        let ids = self.cluster_rows( clus );
        //println!("I found these cluster gene ids: {:?}",ids);
        let mut sum = 0.0;
        let n = self.data.len();
        for i in 0..ids.len() {
            for j in i+1..ids.len() {
                //println!("adding {} ([{}][{}]) to the sum",self.store[ ids[i] ][ ids[j] ], ids[i], ids[j] );
                //sum += self.euclidean_distance( ids[i], ids[j] );
                let index = (ids[i] * (n - 1)) - (ids[i] * (ids[i] + 1)) / 2 + ids[j] - ids[i] - 1;
                sum += self.store[index]
            }
        }
        //println!("I found ek {sum}");
        sum
    }
    
    /// which rows are in cluster x?
    fn cluster_rows( &self, clus: usize ) -> Vec<usize>{
        let mut ret = Vec::<usize>::with_capacity( self.clusters.len() );
        for i in 0..self.clusters.len(){
             if self.clusters[i] == clus{
                ret.push(i);
             }
        }
        ret
    }

    pub fn run( &mut self, max_iter:usize, cool:f32 ) -> usize {
    
      let mut it = 0;
      // calculate the inital energies - this will be modified later
      let mut old_energies= Vec::<f32>::with_capacity( self.k );
      for i in 0..self.k {
          old_energies.push( self.calc_ek( i ) );
      }
      
      let mut old_total: f32 = old_energies.iter().sum::<f32>() / self.k as f32;
      
      let mut rand = rand::rng();
      
      for _ in 0..max_iter{
          it += 1;
          // initate all varaibales
          let mut new_energies = old_energies.clone();
          let moving_row = rand.random_range(0..self.data.len());
          let move_from = self.clusters[moving_row];
          let mut move_to = rand.random_range(0..self.k);
          while move_from == move_to{
              move_to = rand.random_range(0..self.k);
          }
          // move the row from to
          self.clusters[moving_row] = move_to;
          // calculate the new energies
          new_energies[move_from] = self.calc_ek( move_from );
          new_energies[move_to] = self.calc_ek( move_to );
          
          let new_total:f32 = new_energies.iter().sum::<f32>() / self.k as f32;
          
          if new_total < old_total || 
            (-((new_total - old_total) / self.temp)).exp() > rand.random_range(0.0..1.0){
              // that is a good one - keep this
              old_energies[move_from] = new_energies[move_from];
              old_energies[move_to] = new_energies[move_to];
              old_total = new_total;
          }else {
              //this move was not good - drop it!
              self.clusters[moving_row] = move_from;
          }
          // cool the system and exit if it reached ~0
          self.temp *= cool;
      }
      it
    }
    pub fn write_clusters(&self, ofile: &str, sep: char) -> Result<(), Box<dyn Error>> {
        // Open the file in write mode
        let mut file = File::create(ofile)?;

        // Write the header (optional)
        writeln!(file, "Rowname{}Cluster", sep)?;

        // Iterate over the rownames and clusters, writing them to the file
        for (rowname, cluster) in self.rownames.iter().zip(self.clusters.iter()) {
            writeln!(file, "{}{}{}", rowname, sep, cluster+1)?;
        }

        Ok(())
    }
    
    pub fn plot(&self, prefix:&str )-> Result<(), Box<dyn std::error::Error>> {
        let output_dir = Path::new(prefix).parent().unwrap_or_else(|| Path::new("."));
        std::fs::create_dir_all(output_dir)?;

        for cluster_id in 0..self.k {
            let filename = format!("{}_cluster_{}.png", prefix, cluster_id +1 );
            let root = BitMapBackend::new(&filename, (800, 600)).into_drawing_area();
            root.fill(&WHITE)?;

            let mut chart = ChartBuilder::on(&root)
                .caption(format!("Cluster {}", cluster_id+1), ("sans-serif", 20))
                .margin(20)
                .x_label_area_size(40)
                .y_label_area_size(40)
                .build_cartesian_2d(0..self.data[0].len(), 0.0f32..1.0)?; // Adjust Y range if needed

            chart.configure_mesh().draw()?;

            // Collect all rows belonging to this cluster
            let cluster_data: Vec<&Vec<f32>> = self.data.iter()
                .zip(&self.clusters)
                .filter(|&(_, &c)| c == cluster_id)
                .map(|(row, _)| row)
                .collect();

            // Draw each row as a line plot
            for row in cluster_data {
                chart.draw_series(LineSeries::new(
                    row.iter().enumerate().map(|(x, &y)| (x, y)),
                    &BLUE,
                ))?;
            }

            root.present()?;
            println!("Saved: {}", filename);
        }

        Ok(())
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

    #[test]
    fn tes_scale01(){
        let mut obj = SimulatedAnnealing::new( "tests/data/Spellman_Yeast_Cell_Cycle.tsv", 8, 1000.0, '\t' );
        obj.scale_01();
        let exp5:Vec<f32> = vec![0.6989,0.0000,0.0968,0.3333,0.4301,1.0000,0.7419,0.7419,0.6022,0.7634,0.1720,0.4301,0.5161,0.7634,0.6989,0.6559];
        let exp7:Vec<f32> = vec![0.0803,0.0000,0.2867,0.5849,0.9679,1.0000,0.7775,0.7156,0.5505,0.5459,0.4518,0.6193,0.8440,0.8532,0.9335,0.7752];
        let mut dist: f32 = 0.0;
        for i in 0..exp5.len() {
            assert!(
                (obj.data[5][i] - exp5[i]).abs() < 1e-4,
                "Mismatch in gene 5 at index {}: got {}, expected {}",
                i, obj.data[5][i], exp5[i]
            );
            assert!(
                (obj.data[7][i] - exp7[i]).abs() < 1e-4,
                "Mismatch in gene 7 at index {}: got {}, expected {}",
                i, obj.data[7][i], exp7[i]
            );
            dist += (obj.data[7][i]-obj.data[5][i]).powi(2);
        }
        dist = dist.sqrt();
        //assert_eq!( obj.store[5][7], dist, "the distance between gene 5 and 7" );
        obj.clusters[5] = 8;
        obj.clusters[7] = 8;
        obj.k =9;
        assert_eq!( obj.calc_ek(8), dist, "the distance in cluster 8 (genes 5 and 7)" );
    }
}

