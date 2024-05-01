import torch
import time
import numpy as np
import librosa
from shared_resources import SharedResources
from audio_utils import load_audio_buffer, cross_correlation_fft_torch

def process_recent_audio():
    shared_resources = SharedResources()
    original = load_audio_buffer('./reference.wav')
    device = torch.device("mps")

    while True:
        start_compute_time = time.time()  # Capture the start time of the computation
        with shared_resources.buffer_lock:
            buffer_copy = shared_resources.recent_audio_buffer.copy()

        buffer_resampled = librosa.resample(buffer_copy, orig_sr=44100, target_sr=48000)
        correlation = cross_correlation_fft_torch(original, buffer_resampled, device)
        max_corr_index = np.argmax(correlation)

        end_compute_time = time.time()  # Capture the end time of the computation
        compute_time = end_compute_time - start_compute_time  # Calculate the compute time

        # Calculate the time offset in seconds
        time_offset_seconds = max_corr_index / 48000  # Assuming the index corresponds directly to the sample offset

        # Adjust for the buffer time and add the computation time
        adjusted_time_offset_seconds = time_offset_seconds + shared_resources.buffer_time + compute_time

        # Convert adjusted time offset to mm:ss format
        minutes = int(adjusted_time_offset_seconds // 60)
        seconds = int(adjusted_time_offset_seconds % 60)
        formatted_time = f"{minutes:02d}:{seconds:02d}"

        # Output or use the correlation result as needed
        print(f"Adjusted alignment starts at: {formatted_time}, Compute time: {compute_time:.2f}s")