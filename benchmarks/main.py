import caravela
import numpy as np
import time
import matplotlib.pyplot as plt

# Benchmark function
def benchmark(data, queries, anchors, k=50, batch_size=20):
    # generate random data
    ann = caravela.Caravela(anchors)
    ann.fit(data)
    recalls = []
    query_times = []

    # time entire query time
    num_batches = (len(queries) + batch_size - 1) // batch_size

    for i in range(num_batches):
        batch_queries = queries[i * batch_size : (i + 1) * batch_size]

        start_time = time.time()
        batch_neighbors = ann.query(batch_queries, k)
        query_time = time.time() - start_time

        recall = []
        for j, query in enumerate(batch_queries):
            # Calculate recall (using dummy ground truth for demonstration)
            ground_truth = np.argsort(np.linalg.norm(data - query, axis=1))[:k]
            recall.append(len(set(batch_neighbors[j]) & set(ground_truth)) / k)

        recalls.append(np.mean(recall))
        query_times.append(query_time / len(batch_queries))  # Average time per query

    return recalls, query_times


# Generate data and queries
def main():

    X = np.random.uniform(size=(10000, 128))
    Y = np.random.uniform(size=(100, 128))

    param_list = [
        {"label": "balanced", "anchors": [32, 16, 8]},
        {"label": "all32", "anchors": [32, 32, 32]},
        {"label": "all16", "anchors": [16, 16, 16]},
    ]

    plt.figure(figsize=(10, 7))
    for params in param_list:
        recalls, query_times = benchmark(X, Y, params["anchors"])
        queries_per_second = [1.0 / qt for qt in query_times]
        plt.plot(recalls, queries_per_second, marker="o", label=params["label"])

    plt.xlabel("Recall")
    plt.ylabel("Queries per second (1/s)")
    plt.title(
        "Recall-Queries per second (1/s) tradeoff - up and to the right is better"
    )
    plt.legend()
    plt.grid(True)
    plt.show()


if __name__ == "__main__":
    main()
