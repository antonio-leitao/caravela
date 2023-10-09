import caravela
import numpy as np


def main():
    X = np.random.uniform(size=(10000, 100))
    Y = np.random.uniform(size=(100, 100))
    ann = caravela.Caravela([32, 16, 8])
    ann.fit(X)
    preds = ann.batch_query(Y, n_neighbors=50)
    assert len(preds) == 100
    assert len(preds[0]) == 100


if __name__ == "__main__":
    main()
