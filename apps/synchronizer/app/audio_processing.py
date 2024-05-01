import threading
import torch
import time
import numpy as np
from shared_resources import SharedResources
from audio_utils import load_audio_buffer, cross_correlation_fft_torch
from osc_sender import send_osc_message

def process_recent_audio():
    shared_resources = SharedResources()
    original = load_audio_buffer('./003-b-rec.wav')
    device = torch.device("mps")

    while True:
        start_compute_time = time.time()  # Capture the start time of the computation
        with shared_resources.buffer_lock:
            buffer_copy = shared_resources.recent_audio_buffer.copy()
        correlation = cross_correlation_fft_torch(original, buffer_copy, device)
        max_corr_index = np.argmax(correlation)

        end_compute_time = time.time()  # Capture the end time of the computation
        compute_time = end_compute_time - start_compute_time  # Calculate the compute time

        # Calculate the time offset in seconds
        time_offset_seconds = max_corr_index / shared_resources.sample_rate  # Assuming the index corresponds directly to the sample offset

        # Adjust for the buffer time and add the computation time
        adjusted_time_offset_seconds = time_offset_seconds + shared_resources.buffer_time + compute_time
        adjusted_time_offset_ms = adjusted_time_offset_seconds * 1000

        # Convert adjusted time offset to mm:ss format
        minutes = int(adjusted_time_offset_seconds // 60)
        seconds = int(adjusted_time_offset_seconds % 60)
        formatted_time = f"{minutes:02d}:{seconds:02d}"

        # Output or use the correlation result as needed
        print(f"Adjusted alignment starts at: {formatted_time}, Compute time: {compute_time:.2f}s")
        send_osc_message("/playback/position", adjusted_time_offset_ms)

        threading.Event().wait(1.0)


