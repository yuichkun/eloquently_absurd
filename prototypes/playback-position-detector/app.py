import numpy as np
import librosa
import sounddevice as sd
import matplotlib.pyplot as plt
from scipy.signal import correlate


def load_audio(file_path):
    audio, sr = librosa.load(file_path, sr=None)
    return audio, sr

def format_time(seconds):
    minutes = int(seconds // 60)
    seconds = seconds % 60
    return f"{minutes:02d}:{seconds:06.3f}"


def audio_callback(indata, frames, time, status):
    # This function is called for each audio block.
    global buffer, static_audio, sample_rate
    indata = indata.flatten()  # Flatten to 1D array if stereo

    # print(indata)

    # Append new data to the buffer
    buffer = np.concatenate((buffer[len(indata):], indata))

    # Compute cross-correlation
    correlation = correlate(buffer, static_audio, mode='valid')
    lag = np.argmax(correlation)
    print(f"Correlation: {correlation.max()} at lag: {lag}")
    current_playback_position = lag / sample_rate

    # Optionally, you can do something with the current playback position here
    print(f"Current playback position: {format_time(current_playback_position)}")


def start_stream(sr, device=None, channels=1):
    with sd.InputStream(callback=audio_callback, samplerate=sr, channels=channels, device=device):
        print("Streaming started...")
        sd.sleep(duration_seconds * 1000)  # Keep streaming for 'duration_seconds' long


# Load the static sound file
file_path = 'reference.wav'
static_audio, sample_rate = load_audio(file_path)

# Define buffer size and initialize buffer
buffer_size = len(static_audio)  # Buffer size to hold enough data for correlation
buffer = np.zeros(buffer_size)  # Initialize buffer

# Start the audio stream
duration_seconds = 60  # Stream duration in seconds
start_stream(sample_rate)

# time_axis = np.linspace(0, len(static_audio) / sample_rate, num=len(static_audio))

# # Plotting
# plt.figure(figsize=(10, 4))  # Set the figure size (width, height) in inches
# plt.plot(time_axis, static_audio)
# plt.title('Static Audio Waveform')
# plt.ylabel('Amplitude')
# plt.xlabel('Time (seconds)')
# plt.grid(True)
# plt.show()