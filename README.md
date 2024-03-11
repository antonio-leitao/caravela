<p align="center">
  <img src='assets/logo.svg' width='200px' align="center"></img>
</p>

<div align="center">
<h3 max-width='200px' align="center">Caravela</h3>
  <p><i>Permutation Based Approximate Nearest Neighbors<br/>
  No index creation necessary<br/>
  Built with Rust</i><br/></p>
  <p>
<img alt="Pepy Total Downlods" src="https://img.shields.io/pepy/dt/tda?style=for-the-badge&logo=python&labelColor=white&color=blue">
  </p>
</div>

#

> [!CAUTION]
> Caravela is still in concept stage

# Description
Caravela is a Approximate Nearest Neighbour algorithm built using Rust with Python bindings.
It is an attempt to use the factorial explosion of permutations as a means to index and retrieve large amounts of data accurately and efficiently.
It is currently a work in progress.
Caravela uses a relative position system. It measures distance to 16 marks and encodes each point as 128 bit integer, based on the ordering resulting from its distance to the marks. Queries are mady by comparing their permutation of the marks.
1. Speed: New queries only have to check their distance to 16 marks.
2. Memory Reduction: Each point is encoded into the vector of neighbours which is `128` bits.
3. Accuracy: 16 points partition a space into over 20 Trillion(!) regions.

