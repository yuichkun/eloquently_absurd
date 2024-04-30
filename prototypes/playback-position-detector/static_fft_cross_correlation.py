import numpy as np
import matplotlib.pyplot as plt
import librosa

def load_audio_buffer(file_path, sr=48000):
    audio, _ = librosa.load(file_path, sr=sr, mono=True)
    return audio

original = './reference.wav'
cropped = './cropped.wav'
buffer_a = load_audio_buffer(original)  # Longer signal
buffer_b = load_audio_buffer(cropped)   # Shorter signal

def cross_correlation_fft(x, y):
    # Pad the shorter signal with zeros to match the length of the longer signal
    y = np.pad(y, (0, len(x) - len(y)), 'constant')
    
    X = np.fft.fft(x)
    Y = np.fft.fft(y)
    correlation = np.fft.ifft(X * np.conj(Y)).real
    return correlation

correlation = cross_correlation_fft(buffer_a, buffer_b)

# Find the index of the highest correlation
max_corr_index = np.argmax(correlation)
max_corr_value = correlation[max_corr_index]

# Convert index to time in seconds
time_in_seconds = max_corr_index / 48000  # Use the sampling rate

# Convert time in seconds to mm:ss format
minutes = time_in_seconds // 60
seconds = time_in_seconds % 60
formatted_time = f"{int(minutes):02d}:{int(seconds):02d}"

print(f"Index of highest correlation: {max_corr_index}")
print(f"Value of highest correlation: {max_corr_value}")
print(f"Alignment starts at: {formatted_time}")