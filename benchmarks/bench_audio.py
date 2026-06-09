import time
import numpy as np

def benchmark_feature_extraction():
    sample_audio = np.random.randn(44100).astype(np.float32)
    start = time.time()
    
    # Placeholder computation
    _ = np.fft.rfft(sample_audio)
    
    duration = time.time() - start
    print(f"Feature extraction: {duration * 1000:.2f}ms")

def benchmark_inference():
    input_features = np.random.randn(64).astype(np.float32)
    start = time.time()
    
    # Placeholder computation
    _ = np.dot(input_features, np.random.randn(64, 10))
    
    duration = time.time() - start
    print(f"Inference: {duration * 1000:.2f}ms")

if __name__ == "__main__":
    benchmark_feature_extraction()
    benchmark_inference()
