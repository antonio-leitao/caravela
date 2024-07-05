import caravela
import numpy as np
import time
import pandas as pd
import os
import argparse

# Benchmark function
def benchmark(data, queries, k=10):
    # generate random data
    ann = caravela.Caravela(data.shape[1])
    ann.fit(data)

    start_time = time.time()
    neighbors = ann.query(queries, k)
    query_time = time.time() - start_time

    recall = []
    for j, query in enumerate(queries):
        ground_truth = np.argsort(1 - np.dot(data, query))[:k]
        _recall = np.intersect1d(neighbors[j], ground_truth).shape[0] / k
        recall.append(_recall)
    return np.mean(recall), query_time / len(queries)


def append_to_csv(data, csv_file_path="benchmarks/data.csv"):
    """
    Appends a dictionary to the bottom of a CSV file. If the file does not exist, it creates one.

    Parameters:
    data (dict): Dictionary containing the data to append. Keys should match the columns of the CSV.
    csv_file_path (str): Path to the CSV file.
    """
    # Check if the CSV file exists
    if os.path.exists(csv_file_path):
        # Read the existing CSV file
        df = pd.read_csv(csv_file_path)
    else:
        # Create a new DataFrame if the CSV file does not exist
        df = pd.DataFrame(columns=data.keys())
    # Append the new data to the DataFrame
    df = pd.concat([df, pd.DataFrame([data])], ignore_index=True)
    # Save the DataFrame back to the CSV file
    df.to_csv(csv_file_path, index=False)
    print(f"Appended to CSV: {data}")


def main():
    # Set up argument parsing
    parser = argparse.ArgumentParser(description="Append data to CSV.")
    parser.add_argument(
        "-m",
        "--message",
        type=str,
        help="The string to be saved in the CSV.",
    )

    # Parse the arguments
    args = parser.parse_args()
    # Data to append
    np.random.seed(42)
    # normalized search points
    X = np.random.uniform(size=(200_000, 128))
    X = X / np.linalg.norm(X, axis=1, keepdims=True)
    # normalized query points
    Y = np.random.uniform(size=(100, 128))
    Y = Y / np.linalg.norm(Y, axis=1, keepdims=True)
    recall, query_times = benchmark(X, Y)
    # Check if the message argument is provided
    if args.message:
        data = {
            "RECALL": recall,
            "QUERYxSEC": 1.0 / query_times,
            "TIMESTAMP": int(time.time()),
            "COMMENT": args.message,
        }
        append_to_csv(data)
    else:
        print(f"RECALL: {recall} | QUERYxSEC: {1.0 / query_times}")

    # Call the function


if __name__ == "__main__":
    main()
