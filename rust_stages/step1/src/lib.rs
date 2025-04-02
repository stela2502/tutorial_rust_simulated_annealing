
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

        let clusters: Vec<usize> = rownames.iter().map(|_| rand::thread_rng().gen_range(0..k)).collect();
        let n = rownames.len();
        Self{
            data,
            rownames,
            k,
            clusters,
            clusters_energy: vec![0.0; k ],
            temp,
        }
    }
}
