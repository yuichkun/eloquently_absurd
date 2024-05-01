import threading
from audio_stream import start_audio_stream
from audio_processing import process_recent_audio

if __name__ == "__main__":
    audio_thread = threading.Thread(target=start_audio_stream, daemon=True)
    audio_thread.start()

    processing_thread = threading.Thread(target=process_recent_audio, daemon=True)
    processing_thread.start()

    threading.Event().wait()