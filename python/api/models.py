from pydantic import BaseModel
from typing import List, Optional

class AudioAnalysisRequest(BaseModel):
    audio_data: bytes
    sample_rate: int = 44100

class GenreClassification(BaseModel):
    genre: str
    confidence: float

class HarmonicProgression(BaseModel):
    chord: str
    confidence: float
    timestamp_ms: float

class AnalysisResponse(BaseModel):
    filename: str
    duration_ms: float
    genres: List[GenreClassification]
    harmonics: List[HarmonicProgression]
    processing_time_ms: float
