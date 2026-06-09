import torch
from torch.utils.data import Dataset
import numpy as np

class MusicDataset(Dataset):
    def __init__(self, audio_features, labels, transform=None):
        self.audio_features = audio_features
        self.labels = labels
        self.transform = transform

    def __len__(self):
        return len(self.audio_features)

    def __getitem__(self, idx):
        features = torch.FloatTensor(self.audio_features[idx])
        label = torch.LongTensor([self.labels[idx]])
        
        if self.transform:
            features = self.transform(features)
        
        return features, label

class AudioAugmentation:
    """Data augmentation for audio features."""
    
    def __init__(self, noise_factor=0.01):
        self.noise_factor = noise_factor
    
    def __call__(self, features):
        noise = torch.randn_like(features) * self.noise_factor
        return features + noise
