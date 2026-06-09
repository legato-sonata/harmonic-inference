# Setup Guide

## Prerequisites

- Rust 1.70 or later
- Python 3.10 or later
- Git

## Installation

### Step 1: Clone Repository

```bash
git clone https://github.com/legato-sonata/harmonic-inference.git
cd harmonic-inference
```

### Step 2: Build Rust Components

```bash
cd rust
cargo build --release
cd ..
```

### Step 3: Setup Python Environment

```bash
python -m venv venv
source venv/bin/activate  # On Windows: venv\Scripts\activate
pip install -r python/requirements.txt
```

### Step 4: Run Tests

```bash
# Rust tests
cd rust && cargo test && cd ..

# Python tests
pytest tests/ -v
```

### Step 5: Start API Server

```bash
cd python
uvicorn api.server:app --host 0.0.0.0 --port 8000 --reload
```

API available at http://localhost:8000/docs
