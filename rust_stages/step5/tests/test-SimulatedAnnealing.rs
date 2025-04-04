use simulated_annealing::SimulatedAnnealing; // Replace `my_crate` with your actual crate name

#[test]
fn test_simulated_annealing() {
    let sa = SimulatedAnnealing::new( "tests/data/Spellman_Yeast_Cell_Cycle.tsv", 10, 200.0, '\t' ); // Assuming `new()` is implemented
    assert_eq!(sa.data.len(), 256, "we have 256 rows");
    assert_eq!(sa.data[0].len(), 16, "we have 16 cols");
}
