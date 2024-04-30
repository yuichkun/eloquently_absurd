import numpy as np

import matplotlib.pyplot as plt
import librosa

# Function to load an audio file and extract a buffer
def load_audio_buffer(file_path, duration=5, sr=48000):
    audio, _ = librosa.load(file_path, sr=sr,mono=True)
    return audio

# Load buffers from two separate audio files
original = './reference.wav'
cropped = './cropped.wav'
buffer_a = load_audio_buffer(original)
buffer_b = load_audio_buffer(cropped)

# Compute cross-correlation
correlation = np.correlate(buffer_a, buffer_b, mode='valid')
print('len', len(correlation))

# Print the correlation result
max_corr_index = np.argmax(correlation)

# Print the index and value of the highest correlation
print(f"Index of highest correlation: {max_corr_index}")
print(f"Value of highest correlation: {correlation[max_corr_index]}")

# Plotting for visualization
lags = np.arange(-len(buffer_a) + 1, len(buffer_b))
plt.stem(lags, correlation)
plt.title('Cross-correlation between two audio buffers')
plt.xlabel('Lag')
plt.ylabel('Correlation')
plt.show()