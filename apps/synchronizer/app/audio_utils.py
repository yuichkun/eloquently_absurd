import librosa
import torch
import numpy as np

from shared_resources import SharedResources

def load_audio_buffer(file_path, sr):
    shared_resources = SharedResources()
    if sr is None:
        sr = shared_resources.sample_rate
    audio, _ = librosa.load(file_path, sr=sr, mono=True)
    return audio.astype(np.float32)

def cross_correlation_fft_torch(x, y, device):
    x = torch.tensor(x, dtype=torch.float32, device=device)
    y = torch.tensor(y, dtype=torch.float32, device=device)

    if y.size(0) > x.size(0):
        x, y = y, x
    y = torch.nn.functional.pad(y, (0, x.size(0) - y.size(0)))

    X = torch.fft.fft(x)
    Y = torch.fft.fft(y)
    correlation = torch.fft.ifft(X * torch.conj(Y)).real

    return correlation.cpu().numpy()