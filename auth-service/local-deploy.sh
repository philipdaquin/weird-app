

# Build App 
cargo build --release

# Build Docker Image
docker build -t auth-service .

# Add Tags
docker tag whisper-service philipasd/auth-service:0.0.0

# docker push philipasd/auth-service:v0.0.0
