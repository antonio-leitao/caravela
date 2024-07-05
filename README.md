<p align="center">
  <img src='assets/logo.svg' width='200px' align="center"></img>
</p>

<div align="center">
<h3 max-width='200px' align="center">Caravela</h3>
  <p><i>Permutation Based Approximate Nearest Neighbors<br/>
  No index, infinitely incrementable<br/>
  Built with Rust</i><br/></p>
  <p>
<img alt="Pepy Total Downlods" src="https://img.shields.io/pepy/dt/caravela?style=for-the-badge&logo=python&labelColor=white&color=blue">
  </p>
</div>

#

> [!CAUTION]
> Caravela is still in concept stage

# Why Caravela?
1. **No fitting required.** Start your index with as few as a single point.
3. **Infinitely incrementable**. No need to re-run, ever.
3. **Blazingly Fast**. Built in Rust. 

## Installation
Pre-built packages for MacOS, Windos and most Linux distributions in [PyPI](https://pypi.org/project/tda/) and can be installed with:

```sh
pip install caravela
```
On uncommon architectures, you may need to first
[install Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) before running `pip install caravela`.


## Description
Caravela is a Approximate Nearest Neighbour algorithm built using Rust with Python bindings.
It is an attempt to use the factorial explosion of permutations as a means to index and retrieve large amounts of data accurately and efficiently.
It is currently a work in progress.
Caravela uses a relative position system based on permutations.

