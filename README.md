<h1 align=center>
<p><code>( /\ /? /\ \/ [- |_ /\</code></p>
</h1>

# Description
Caravela is a Approximate Nearest Neighbour algorithm built using Rust with Python bindings.
It is an attempt to use the factorial explosion of permutations as a means to index and retrieve large amounts of data accurately and efficiently.
It is currently a work in progress.
Caravela uses a relative position system. It measures distance to 16 marks and encodes each point as 128 bit integer, based on the ordering resulting from its distance to the marks. Queries are mady by comparing their permutation of the marks.
1. Speed: New queries only have to check their distance to 16 marks.
2. Memory Reduction: Each point is encoded into the vector of neighbours which is `128` bits.
3. Accuracy: 16 points partition a space into over 20 Trillion(!) regions.

# How it works
Index creation is done in a way as to assure that the 20T regions in fact exist. The region boundaries are the bisectors of every pairwise set of marks. To ensure that all possible permutation exist then we must create a polygon with the points that ensure that there is intersection between any two of these bisectors. Moreover we must ensure that it contains the data.

For now my soultion has been to use a 16-simplex centered on the mean of the data. This assures not only the intersection of all bisectors but allso that such intersection occurs smack down in the middle of the data.
It is easy to see that an n-simplex will contain all possible permutations since a simplex is given by:

$$ \Delta = \left( \theta_1u_1 + \theta_2u_2 + \theta_3u_3 + \dots + \theta_nu_n \middle| \sum_i \theta_i =1\right)$$

It seems to me that as long as we make sure that the data is contained in the simplex then all regions should be accessible. As in, a point `x` will be in region $ABCD$ if $\theta_A > \theta_B > \theta_C > \theta_D$. 

Currently each vertex of the simplex is positioned along the first 16 direction of the PCA, centerd in the mean of the data at the distance of the amplitude of the data. This might change altough it doesnt seem to affect much the results. If the previous sentence is true it would mean that it might be possible to create an index that is inependent of the data given, which is very desireable specially for streaming data.
