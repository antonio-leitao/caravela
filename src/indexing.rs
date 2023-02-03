// indexing
//works very well for up to 5M points
// should work perfectly up untill 20_922_789_888_000 (20 TRILLION) points without quantization
//

//requries: 
//      X: n dimensional vectors
//      n: number of marks (16)
//      quant?: quantization (4) (not to be implemented now)

// calculate mean point of data (to be optimized)
// calculate n principal components
// create n-simplex
// for each point calculate distance to simplex vertices
// create for each point (64 bits) -> drop point
// create {tree} (study this part)
// return tree
