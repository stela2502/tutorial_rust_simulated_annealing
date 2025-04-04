use clap::Parser;
use std::time::SystemTime;
// this is specific for my package which I have called simulated_annealing_new as I had an other version, too.
use simulated_annealing::SimulatedAnnealing;

#[derive(Parser)]
#[clap(version = "1.0.0", author = "Stefan L. <stefan.lang@med.lu.se>")]
struct Opts {
    /// the input text table
    #[clap(short, long)]
    file: String,
    /// the column separator for the file
    #[clap(default_value= "\\t",short, long)]
    sep: String,
    /// the number of clusters
    #[clap(short, long)]
    clusters: usize,
    /// the starting temperature
    #[clap(default_value_t= 20.0,short, long)]
    temp: f32,
    /// the cooling factor
    #[clap(default_value_t= 0.9995,short, long)]
    cool: f32,
    ///max number of iterations
    #[clap(default_value_t= 1000*1000,short, long)]
    max_it: usize,
    /// the grouping outfile
    #[clap(short, long)]
    outfile: String,
}

fn main() {
    let now = SystemTime::now();
    
    let opts: Opts = Opts::parse();

    let mut sep = '\t';
    if &opts.sep != "\\t"{
        //println!("I set sep to {}", opts.sep );
        sep = opts.sep.chars().next().unwrap(); 
    }

    let mut sim = SimulatedAnnealing::new( &opts.file, opts.clusters, opts.temp, sep );
    sim.scale_01();    

    //println!("Initial state: {sim}");

    let iterations = sim.run( opts.max_it, opts.cool );

    let _= sim.plot( &opts.outfile );

    //println!("Final state {sim}");

    match sim.write_clusters( &opts.outfile, sep ){
        Ok(_) => println!("Clusters written to {}", &opts.outfile ),
        Err(e) => eprintln!("Failed to write the data to {}: {:?}", &opts.outfile, e),
    }

    match now.elapsed() {
        Ok(elapsed) => {
            let mut milli = elapsed.as_millis();

            let mil = milli % 1000;
            milli= (milli - mil) /1000;

            let sec = milli % 60;
            milli= (milli -sec) /60;

            let min = milli % 60;
            milli= (milli -min) /60;

            eprintln!("finished in {milli} h {min} min {sec} sec {mil} milli sec");
        },
        Err(e) => {println!("Error: {e:?}");}
    }
}


