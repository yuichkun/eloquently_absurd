import threading
import numpy as np

class SingletonMeta(type):
    _instances = {}
    def __call__(cls, *args, **kwargs):
        if cls not in cls._instances:
            cls._instances[cls] = super(SingletonMeta, cls).__call__(*args, **kwargs)
        return cls._instances[cls]

class SharedResources(metaclass=SingletonMeta):
    def __init__(self):
        self.sample_rate = 44100
        self.buffer_time = 5
        self.buffer_length = self.sample_rate * self.buffer_time
        self.recent_audio_buffer = np.zeros(self.buffer_length, dtype=np.float32)
        self.buffer_lock = threading.Lock()