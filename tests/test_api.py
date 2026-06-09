import pytest
from fastapi.testclient import TestClient

try:
    from python.api.server import app
    client = TestClient(app)
    
    def test_health_check():
        response = client.get("/health")
        assert response.status_code == 200
        assert response.json()["status"] == "healthy"
    
    def test_list_models():
        response = client.get("/api/models")
        assert response.status_code == 200
        assert "models" in response.json()
except ImportError:
    pytest.skip("FastAPI not available")
