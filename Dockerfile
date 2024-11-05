# Start from ROS 2 Humble
FROM ros:humble

# Install build essentials, curl, and libclang (needed for Rust bindgen)
RUN apt-get update && apt-get install -y \
    build-essential \
    curl \
    libclang1 \
    libclang-dev \
    clang \
    # Graphics dependencies
    pkg-config \
    libasound2-dev \
    libudev-dev \
    # X11 dependencies
    libx11-dev \
    libxext-dev \
    libxrender-dev \
    libxinerama-dev \
    libxi-dev \
    libxrandr-dev \
    libxcursor-dev \
    libxkbcommon-dev \
    libxkbcommon-x11-0 \
    # Wayland dependencies
    libwayland-dev \
    libwayland-cursor0 \
    # Vulkan dependencies
    libvulkan1 \
    vulkan-tools \
    libvulkan-dev \
    # OpenGL dependencies
    libgl1-mesa-dev \
    libgles2-mesa-dev \
    mesa-utils \
    # Additional useful tools
    x11-apps \
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

ENV DISPLAY=127.0.0.1:0
ENV LIBGL_ALWAYS_SOFTWARE=1

ENV DISPLAY=host.docker.internal:0
ENV XAUTHORITY=/root/.Xauthority

CMD ["bash"]