import numpy as np
import sounddevice as sd
import librosa
import threading
import time
import torch


# Load the original audio file
def load_audio_buffer(file_path, sr=48000):
    audio, _ = librosa.load(file_path, sr=sr, mono=True)
    return audio.astype(np.float32)

original = load_audio_buffer('./reference.wav')  # Original signal, sample rate 48000
print(f"Original audio length: {len(original) / 48000:.2f} seconds")

buffer_time = 5  # seconds
sample_rate = 44100 #hz
# Initialize a FIFO buffer with a static length of 5 seconds at 44100 Hz
buffer_length = buffer_time * sample_rate
recent_audio_buffer = np.zeros(buffer_length)

# Lock for synchronizing access to the buffer
buffer_lock = threading.Lock()

# Audio callback function
def audio_callback(indata, frames, time, status):
    if status:
        print(status)
    global recent_audio_buffer
    with buffer_lock:
        # Convert the input array to a 1D numpy array suitable for processing
        mono_data = np.mean(indata, axis=1)
        # Update the buffer with new data, maintaining its static length
        recent_audio_buffer = np.roll(recent_audio_buffer, -len(mono_data))
        recent_audio_buffer[-len(mono_data):] = mono_data

# Start streaming from the microphone
def start_audio_stream():
    stream = sd.InputStream(channels=1, samplerate=44100, callback=audio_callback)
    with stream:
        threading.Event().wait()  # Wait indefinitely
# Ensure tensors are created on the M1 GPU (if available)
device = torch.device("mps")  # Apple's Metal Performance Shaders (MPS) backend

def cross_correlation_fft_torch(x, y):
    # Convert x and y to float32 before creating tensors for MPS compatibility
    x = torch.tensor(x, dtype=torch.float32, device=device)
    y = torch.tensor(y, dtype=torch.float32, device=device)

    # Ensure x is the longer signal
    if y.size(0) > x.size(0):
        x, y = y, x
    # Pad the shorter signal with zeros
    y = torch.nn.functional.pad(y, (0, x.size(0) - y.size(0)))

    X = torch.fft.fft(x)
    Y = torch.fft.fft(y)
    correlation = torch.fft.ifft(X * torch.conj(Y)).real

    # The result is already in float32 and can be used directly or converted back to numpy
    return correlation.cpu().numpy()

# Function to continuously process the recent audio buffer
def process_recent_audio():
    while True:
        print("Processing recent audio...")
        start_compute_time = time.time()  # Capture the start time of the computation
        with buffer_lock:
            # Make a copy of the buffer for processing to avoid mutation
            buffer_copy = recent_audio_buffer.copy()

        # Resample the copied buffer to match the original's sample rate
        buffer_resampled = librosa.resample(buffer_copy, orig_sr=44100, target_sr=48000)

        # Perform FFT-based cross-correlation
        correlation = cross_correlation_fft_torch(original, buffer_resampled)
        max_corr_index = np.argmax(correlation)

        end_compute_time = time.time()  # Capture the end time of the computation
        compute_time = end_compute_time - start_compute_time  # Calculate the compute time

        # Calculate the time offset in seconds
        time_offset_seconds = max_corr_index / 48000  # Assuming the index corresponds directly to the sample offset

        # Adjust for the buffer time and add the computation time
        adjusted_time_offset_seconds = time_offset_seconds + buffer_time + compute_time

        # Convert adjusted time offset to mm:ss format
        minutes = int(adjusted_time_offset_seconds // 60)
        seconds = int(adjusted_time_offset_seconds % 60)
        formatted_time = f"{minutes:02d}:{seconds:02d}"

        # Output or use the correlation result as needed
        print(f"Adjusted alignment starts at: {formatted_time}, Compute time: {compute_time:.2f}s")

# Start the audio stream in a separate thread
audio_thread = threading.Thread(target=start_audio_stream, daemon=True)
audio_thread.start()

# Start processing the recent audio in a separate thread
processing_thread = threading.Thread(target=process_recent_audio, daemon=True)
processing_thread.start()

# Keep the main thread alive
threading.Event().wait()