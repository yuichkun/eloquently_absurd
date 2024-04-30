import torch
import numpy as np
import matplotlib.pyplot as plt

# Simple example lists converted to PyTorch tensors
a = torch.tensor([1, 2, 3, 4, 5], dtype=torch.float32)  # First sequence
b = torch.tensor([2, 3, 4], dtype=torch.float32)       # Second sequence, a subset of the first

# Ensure tensors are on the GPU
device = torch.device("cuda" if torch.cuda.is_available() else "mps")  # Use CUDA if available, else MPS for Apple Silicon
a = a.to(device)
b = b.to(device)

# Perform cross-correlation using PyTorch
# PyTorch does not have a direct correlate function, so we use conv1d which is equivalent for this purpose
# We need to add batch and channel dimensions ([batch, channel, length]) to the tensors
a_unsqueezed = a.unsqueeze(0).unsqueeze(0)  # Shape becomes [1, 1, 5]
b_unsqueezed = b.flip(dims=[0]).unsqueeze(0).unsqueeze(0)  # Shape becomes [1, 1, 3] and flip to act as correlation

# Compute cross-correlation using conv1d
correlation = torch.nn.functional.conv1d(a_unsqueezed, b_unsqueezed, padding='valid')

# Move the result back to CPU for analysis and visualization
correlation_cpu = correlation.squeeze().to("cpu").numpy()

# Print the correlation result
print("Cross-correlation:", correlation_cpu)

# Find the index of the highest correlation
max_corr_index = np.argmax(correlation_cpu)

# Print the index and value of the highest correlation
print(f"Index of highest correlation: {max_corr_index}")
print(f"Value of highest correlation: {correlation_cpu[max_corr_index]}")

# Plotting for visualization
plt.stem(range(len(correlation_cpu)), correlation_cpu)
plt.title('Cross-correlation between sequences a and b')
plt.xlabel('Lag')
plt.ylabel('Correlation')
plt.show()