import numpy as np
import matplotlib.pyplot as plt

# Simple example lists
a = [1, 2, 3, 4, 5]  # First sequence
b = [2, 3, 4]        # Second sequence, a subset of the first

# Compute cross-correlation
correlation = np.correlate(a, b, mode='full')

# Print the correlation result
print("Cross-correlation:", correlation)
# Find the index of the highest correlation
max_corr_index = np.argmax(correlation)

# Print the index and value of the highest correlation
print(f"Index of highest correlation: {max_corr_index}")
print(f"Value of highest correlation: {correlation[max_corr_index]}")

# Plotting for visualization
plt.stem(range(len(correlation)), correlation)
plt.title('Cross-correlation between sequences a and b')
plt.xlabel('Lag')
plt.ylabel('Correlation')
plt.show()