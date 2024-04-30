import numpy as np
import matplotlib.pyplot as plt

# Simple example lists
a = [1, 2, 3, 4, 5]  # First sequence
b = [2, 3, 4]        # Second sequence, a subset of the first

# Compute cross-correlation
correlation = np.correlate(a, b, mode='full')

# Print the correlation result
print("Cross-correlation:", correlation)

# Plotting for visualization
plt.stem(range(len(correlation)), correlation)
plt.title('Cross-correlation between sequences a and b')
plt.xlabel('Lag')
plt.ylabel('Correlation')
plt.show()