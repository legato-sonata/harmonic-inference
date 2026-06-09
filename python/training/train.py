import torch
import torch.nn as nn
import torch.optim as optim
from torch.utils.data import DataLoader
import logging

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

class Trainer:
    def __init__(self, model, device='cpu'):
        self.model = model.to(device)
        self.device = device
        self.criterion = nn.CrossEntropyLoss()
        self.optimizer = optim.Adam(model.parameters(), lr=0.001)
    
    def train_epoch(self, train_loader):
        self.model.train()
        total_loss = 0.0
        
        for batch_idx, (features, labels) in enumerate(train_loader):
            features = features.to(self.device)
            labels = labels.squeeze().to(self.device)
            
            self.optimizer.zero_grad()
            outputs = self.model(features)
            loss = self.criterion(outputs, labels)
            loss.backward()
            self.optimizer.step()
            
            total_loss += loss.item()
            
            if batch_idx % 10 == 0:
                logger.info(f"Batch {batch_idx}, Loss: {loss.item():.4f}")
        
        return total_loss / len(train_loader)
    
    def validate(self, val_loader):
        self.model.eval()
        total_loss = 0.0
        correct = 0
        total = 0
        
        with torch.no_grad():
            for features, labels in val_loader:
                features = features.to(self.device)
                labels = labels.squeeze().to(self.device)
                
                outputs = self.model(features)
                loss = self.criterion(outputs, labels)
                total_loss += loss.item()
                
                _, predicted = torch.max(outputs, 1)
                correct += (predicted == labels).sum().item()
                total += labels.size(0)
        
        accuracy = 100 * correct / total
        logger.info(f"Validation Loss: {total_loss / len(val_loader):.4f}, Accuracy: {accuracy:.2f}%")
        return total_loss / len(val_loader), accuracy
    
    def train(self, train_loader, val_loader, epochs=10):
        for epoch in range(epochs):
            logger.info(f"Epoch {epoch + 1}/{epochs}")
            train_loss = self.train_epoch(train_loader)
            val_loss, val_acc = self.validate(val_loader)
            logger.info(f"Train Loss: {train_loss:.4f}, Val Loss: {val_loss:.4f}, Val Acc: {val_acc:.2f}%")
