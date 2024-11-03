# Start from ROS 2 Humble
FROM ros:humble

# Install build essentials, curl, and libclang (needed for Rust bindgen)
RUN apt-get update && apt-get install -y \
    build-essential \
    curl \
    libclang1 \
    libclang-dev \
    clang \
    && rm -rf /var/lib/apt/lists/*

# Install Rust
ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Create a workspace directory
WORKDIR /workspace

# Source ROS environment by default
RUN echo "source /opt/ros/humble/setup.bash" >> ~/.bashrc

# Export LIBCLANG_PATH for bindgen
ENV LIBCLANG_PATH=/usr/lib/llvm-14/lib

# Set default command
CMD ["bash"]