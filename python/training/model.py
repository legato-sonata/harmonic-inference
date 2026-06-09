import torch
import torch.nn as nn
import torch.nn.functional as F

class GenreClassifier(nn.Module):
    """Neural network for music genre classification."""
    
    def __init__(self, input_size=64, num_genres=10):
        super().__init__()
        self.fc1 = nn.Linear(input_size, 256)
        self.fc2 = nn.Linear(256, 128)
        self.fc3 = nn.Linear(128, 64)
        self.fc4 = nn.Linear(64, num_genres)
        
        self.dropout = nn.Dropout(0.3)
        self.batch_norm1 = nn.BatchNorm1d(256)
        self.batch_norm2 = nn.BatchNorm1d(128)
        self.batch_norm3 = nn.BatchNorm1d(64)
    
    def forward(self, x):
        x = F.relu(self.batch_norm1(self.fc1(x)))
        x = self.dropout(x)
        
        x = F.relu(self.batch_norm2(self.fc2(x)))
        x = self.dropout(x)
        
        x = F.relu(self.batch_norm3(self.fc3(x)))
        x = self.dropout(x)
        
        x = self.fc4(x)
        return x

class HarmonicPredictor(nn.Module):
    """Neural network for harmonic progression prediction."""
    
    def __init__(self, input_size=64, sequence_length=4):
        super().__init__()
        self.lstm = nn.LSTM(input_size, 128, num_layers=2, batch_first=True, dropout=0.3)
        self.fc1 = nn.Linear(128, 64)
        self.fc2 = nn.Linear(64, 12)  # 12 notes in chromatic scale
        
    def forward(self, x):
        x = x.unsqueeze(1)  # Add sequence dimension
        lstm_out, _ = self.lstm(x)
        x = F.relu(self.fc1(lstm_out[:, -1, :]))
        x = self.fc2(x)
        return x
