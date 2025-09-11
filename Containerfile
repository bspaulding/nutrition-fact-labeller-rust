# =========================
# 0. Builder stage
# =========================
FROM public.ecr.aws/docker/library/rust:1.89 as builder

# Create a new empty shell project
WORKDIR /app

# Copy manifests first (for caching dependencies)
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs so dependencies can be built and cached
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies only
RUN cargo build --release && rm -rf src

# Copy source and build actual binary
COPY src ./src
RUN cargo build --release

# =========================
# 1. Model Download stage
# =========================
FROM public.ecr.aws/docker/library/debian:bookworm-slim AS downloader

# Install minimal dependencies for Rust binary execution
RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /paddleocr-models

# Copy model weights
RUN curl --output ppocrv4_mobile_det.onnx https://github.com/GreatV/oar-ocr/releases/download/v0.1.0/ppocrv4_mobile_det.onnx
RUN curl --output en_ppocrv4_mobile_rec.onnx https://github.com/GreatV/oar-ocr/releases/download/v0.1.0/en_ppocrv4_mobile_rec.onnx
RUN curl --output en_dict.txt https://github.com/GreatV/oar-ocr/releases/download/v0.1.0/en_dict.txt
RUN curl --output pplcnet_x1_0_doc_ori.onnx https://github.com/GreatV/oar-ocr/releases/download/v0.1.0/pplcnet_x1_0_doc_ori.onnx
RUN curl --output pplcnet_x1_0_textline_ori.onnx https://github.com/GreatV/oar-ocr/releases/download/v0.1.0/pplcnet_x1_0_textline_ori.onnx
RUN curl --output uvdoc.onnx https://github.com/GreatV/oar-ocr/releases/download/v0.1.0/uvdoc.onnx

# =========================
# 2. Runtime stage
# =========================
FROM public.ecr.aws/docker/library/debian:bookworm-slim AS runtime

WORKDIR /app

# Install minimal dependencies for Rust binary execution
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy compiled binary from builder
COPY --from=builder /app/target/release/nutrition-fact-labeller /usr/local/bin/nutrition-fact-labeller
COPY --from=downloader /paddleocr-models /app/paddleocr-models

# Run as non-root user (optional best practice)
RUN useradd -m appuser
USER appuser

# Entrypoint
ENTRYPOINT ["nutrition-fact-labeller"]
