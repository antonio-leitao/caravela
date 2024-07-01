import caravela
import numpy as np
import functools
import matplotlib.pyplot as plt

def log(func):
    @functools.wraps(func)
    def wrapper(*args, **kwargs):
        print(f"\t {func.__name__}...", end="")
        try:
            result = func(*args, **kwargs)
            print(" OK")
            return result
        except Exception as e:
            print(" FAIL")
            raise e

    return wrapper


def generate_normalized_points():
    # Generate n random points in 3D space
    points = np.random.randn(3, 256)
    # Normalize each point to lie on the unit sphere
    points = points / np.linalg.norm(points, axis=1, keepdims=True)
    return points

def generate_and_compare(ann):
    # Generate three normalized points
    points = generate_normalized_points()
    
    # Encode the points
    encoded_points = np.array([ann.encode(point) for point in points])
    
    # Compute the original distances
    original_ij = np.linalg.norm(points[0] - points[1])
    original_jk = np.linalg.norm(points[1] - points[2])
    
    # Compute the encoded distances
    encoded_ij = np.linalg.norm(encoded_points[0] - encoded_points[1])
    encoded_jk = np.linalg.norm(encoded_points[1] - encoded_points[2])
    
    # Calculate the differences
    original_diff = original_ij - original_jk
    encoded_diff = encoded_ij - encoded_jk
    
    return original_diff, encoded_diff

def main(n_samples=100):
    original_diffs = []
    encoded_diffs = []
    
    ann = caravela.Caravela(16,256)
    for _ in range(n_samples):
        original_diff, encoded_diff = generate_and_compare(ann)
        original_diffs.append(original_diff)
        encoded_diffs.append(encoded_diff)
    
    # Plot the differences
    plt.scatter(original_diffs, encoded_diffs, alpha=0.5)
    plt.xlabel('Original Difference (ij - jk)')
    plt.ylabel('Encoded Difference (ij - jk)')
    plt.title('Original vs Encoded Distance Differences')
    plt.grid(True)
    plt.show()

if __name__ == "__main__":
    main()
