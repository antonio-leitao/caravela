
<h1 align=center>
<img src="assets/caravela_logo_placeholder.jpeg">
<p><code>( /\ /? /\ \/ [- |_ /\</code></p>
<h1/>

<p>Logo is quanta Magazine but I like the vibe, is here for inspo.</p>


# Description
Caravela uses a relative position system. It measures your position to 16 marks and encodes each vector as a sequence of nearest neighbors (ABCD). This provides
1. Speed: New queries only have to check their distance to 16 marks.
2. Memory Reduction: Each point is encoded into the vector of neighbours which is `64` bits.
3. Accuracy: 16 points partition a space into over 20 Trillion(!) regions.

## Index Creation
Index creation has to be done in a carefull way as to assure that the 20Trl regions in fact exist. The region boundaries are the bisectors of every pairwise set of marks. To ensure that all possible permutation exist then we must create a polygon with the points that ensure that there is intersection between any two of these bisectors. MOreover we must ensure that it contains the data.

For now my soultion has been to use a 16-simplex centered on the mean of the data. This assures not only the intersection of all bisectors but allso that such intersection occurs smack down in the middle of the data.
It is easy to see that an n-simplex will contain all possible permutations since a simplex is given by:

$$ \Delta = \left{ \theta_1u_1 + \theta_2u_2 + \theta_3u_3 + \dots + \theta_nu_n \middle| \sum_iˆk \theta_i =1\right}$$

It seems to me that as long as we make sure that the data is contained in the simplex then all regions should be accessible. As in, a point `x` will be in region $A,B,C,D$ if $\theta_a > \theta_b > \theta_c > \theta_d$. But i might be wrong, altough early tests point otherwise.

Currently each vertex of the simplex is positioned along the first 16 direction of the PCA, centerd in the mean of the data at the distance of the amplitude of the data. This might change altough it doesnt seem to affect much the results.

## Query
Necessary first step is to find the distance to each of the 16 vertexes and order the result. Finding an exact match is as simple as accessing some array with that vector as index. The problem is searching.

This is the tricky part. We dont want to search. Obvious choice is to just access some index. But i really dont see how. Best candidate has been the prefix tree let me show how:

Each vector is encoded as the sequecne of their neighbours. Meaning if we are querying a vector ABCD we want to search first for an exact match ABCD (a point in the same region), if there isnt we go from the bottom bup, we search ABDC.

If there isnt where do we go? Do we go AC-- or AD--? This one is obvious we go AC-- since it is closer to C than to B. ACBD -> ACDB -> BACD -> BADC -> CABD -> CADB. Therre is something interesting as the initial order of first neighbours also gives us the order in which to look for. This makes the search orderable which really is just indexing then. But. I cant. make. this. work.

So far I've gor only a prefix tree. First level is the first neighbour {A,B,C,D} second level is {B, C, D} if the path goes thorugh A. Good thisng about this is that we cant stop the serach early on if we know that there are no points that start with AD-- in the tree. Im very worried about speed and memory here thgouh.

# Stats
Assume `n` points in a `d` dimensional vector space with `k=16` marks. 
1. **Speed**:
    - Index Creation -> PCA + distance to n points should be something like `O(n logn + d )`
    - Query -> Distance to `k=16` points + tree transversal that's `O(k)`
2. **Memory**:
    - Permutations can be encoded into `O(k logk) = 64` bits (half the size of a pointer) which is great if we manage to add no memory overhead. Means we can't use pointers in query.
    - Realistically I think we will be looking at `128` bits per sample which is a shame but query speed is more important. Besides still beats the `256`bits of `faiss` and `scann`.
3. **Accuracy**:
    - Not expecting anything less than 100% recall rate untill 20_000_000_000_000 (20 trillion) samples. Probably won't be true but there's no reason why we shouldn't be aiming towards that ballpark.
    - The more accurate the algorithm is the faster it is since the main bottleneck is tree search.

# Todo
### Theory
    - [ ] Find more about hyperplane arranjements and simplexes.
    - [ ] Find data structure. Compressed prefix tree seems apropriate but maybe there's better.
    - [ ] Figure out query algorithm. Prefix tree search could work on O(k) -> O(1) but memory overhead seems inapropriate. There has to be a smarter way.
    - [ ] Need a good PCA. Not problematic since its at index creation.
### Testing
    - [ ] Test something other than the mean for simplex center.   
    - [ ] Test limit of 16 point partition in n-dimensional space
    - [ ] Test simplex structure. Compare with:
        - [x] Random arranjement
        - [x] K-Means
        - [ ] Unaligned Simplex (no PCA)
        - [ ] PCA with eight vectors
        - [ ] Others
    - [ ] Figure out arranjement for dimensions less than 15. Maybe revisit PCA with 2 points per pc (original idea)
    - [ ] Test effect of amplitude and component weights (even though equidistance seems a must).



