import threading
import sounddevice as sd
import numpy as np
from shared_resources import SharedResources

def audio_callback(indata, frames, time, status):
    if status:
        print(status)
    shared_resources = SharedResources()
    with shared_resources.buffer_lock:
        mono_data = np.mean(indata, axis=1)
        shared_resources.recent_audio_buffer = np.roll(shared_resources.recent_audio_buffer, -len(mono_data))
        shared_resources.recent_audio_buffer[-len(mono_data):] = mono_data

def start_audio_stream():
    print("Starting audio stream...")
    # TODO: sample rate should be configurable
    stream = sd.InputStream(channels=1, samplerate=44100, callback=audio_callback)
    with stream:
        threading.Event().wait()