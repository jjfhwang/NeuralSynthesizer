# NeuralSynthesizer: Real-time Audio Synthesis with Neural Networks

NeuralSynthesizer is a Rust-based project designed for real-time audio synthesis using neural networks. It provides a flexible and performant framework for exploring and deploying various neural synthesis techniques, offering developers a powerful tool for creating innovative audio experiences.

This project aims to bridge the gap between cutting-edge machine learning and practical audio applications. Unlike traditional synthesis methods that rely on handcrafted algorithms, NeuralSynthesizer leverages the expressive power of neural networks to generate complex and dynamic sounds. This opens up possibilities for creating novel instruments, sound effects, and interactive audio environments. The core of the system is built with efficiency in mind, allowing for real-time performance on modern hardware. This is achieved through careful optimization of the neural network inference process and efficient memory management.

NeuralSynthesizer is not just a theoretical exercise; its built for real-world use. We envision it being used in a variety of applications, including interactive music performance, game audio, virtual reality sound design, and research into novel audio synthesis techniques. The modular design of the project allows developers to easily experiment with different neural network architectures and training datasets. This makes it a valuable tool for both researchers and practitioners interested in exploring the potential of neural audio synthesis.

This library provides a set of core components and utilities to facilitate the creation of custom audio synthesizers. It allows developers to easily integrate pre-trained neural network models and control them through a simple and intuitive API. The project also includes examples and tutorials to help users get started quickly and explore the various possibilities of neural audio synthesis. The goal is to provide a robust and well-documented platform for the development of innovative audio applications.

## Key Features

*   **Real-time Audio Synthesis:** Optimized for low-latency audio generation, enabling interactive performance and responsive sound design.
    Technically, this is achieved through a lock-free ring buffer design for audio data transfer and optimized neural network inference routines leveraging SIMD instructions where available.
*   **Modular Neural Network Integration:** Supports a wide range of neural network architectures, allowing developers to easily integrate their own custom models.
    The system uses the `tract` crate for efficient neural network inference and provides a flexible interface for defining custom network input and output mappings.
*   **Flexible Control Interface:** Provides a simple and intuitive API for controlling the neural network parameters and generating dynamic sounds.
    This interface utilizes Rust's powerful type system to ensure type safety and prevent common errors. It offers functions to set network inputs (e.g., MIDI parameters, control signals) and retrieve audio samples in real-time.
*   **Cross-Platform Compatibility:** Designed to run on a variety of platforms, including Windows, macOS, and Linux.
    The codebase is written in pure Rust and utilizes cross-platform libraries to ensure compatibility across different operating systems. Platform-specific optimizations are implemented where necessary to maximize performance.
*   **Extensible Audio Backend:** Supports multiple audio backends (e.g., PortAudio, JACK) for seamless integration with existing audio environments.
    This allows developers to choose the audio backend that best suits their needs and integrate NeuralSynthesizer into their existing audio workflows.
*   **Example Neural Synthesizer Implementations:** Includes example implementations of different neural synthesizers, showcasing the capabilities of the framework.
    These examples serve as a starting point for developers looking to create their own custom neural synthesizers. They demonstrate how to integrate pre-trained neural networks, control their parameters, and generate audio in real-time.

## Technology Stack

*   **Rust:** The primary programming language, chosen for its performance, safety, and concurrency features.
*   **Tract:** A high-performance neural network inference engine for Rust, used for efficient execution of neural networks.
*   **PortAudio/JACK:** Cross-platform audio libraries used for audio input and output. PortAudio provides a simple and portable API, while JACK offers advanced features for professional audio applications.
*   **ndarray:** Provides efficient multi-dimensional array manipulation, essential for processing audio data and neural network inputs/outputs.
*   **serde:** Used for serialization and deserialization of configuration data and potentially for saving/loading neural network models.

## Installation

1.  **Install Rust:** If you don't have Rust installed, download and install it from [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install). Make sure you have Cargo, Rust's package manager and build system, installed as well.

2.  **Clone the Repository:** Clone the NeuralSynthesizer repository from GitHub:
    `git clone https://github.com/jjfhwang/NeuralSynthesizer.git`

3.  **Navigate to the Project Directory:**
    `cd NeuralSynthesizer`

4.  **Build the Project:**
    `cargo build --release` This command builds the project in release mode, optimizing for performance.

5.  **Install Dependencies:** The project relies on PortAudio for audio output. The installation process varies depending on your operating system.
    *   **Linux:** You may need to install the PortAudio development libraries using your system's package manager. For example, on Debian/Ubuntu:
        `sudo apt-get install libportaudio2 libportaudiocpp0 portaudio19-dev`
    *   **macOS:** PortAudio can be installed using Homebrew:
        `brew install portaudio`
    *   **Windows:** PortAudio dependencies are typically handled automatically during the build process, but you may need to install the Visual Studio Build Tools if you encounter any issues.

## Configuration

NeuralSynthesizer uses a configuration file (e.g., `config.toml`) to define various settings, such as the audio backend, buffer size, and neural network parameters. You can customize this file to suit your specific needs.

Example `config.toml`:

[audio]
backend = "portaudio" # or "jack"
buffer_size = 1024
sample_rate = 44100

[neural_network]
model_path = "path/to/your/model.onnx"
input_size = 128 # Size of the input vector for the neural network

These settings can also be overridden using environment variables. For example, to set the audio backend to JACK, you can set the `NEURALSYNTHESIZER_AUDIO_BACKEND` environment variable:

`export NEURALSYNTHESIZER_AUDIO_BACKEND="jack"` (Linux/macOS)
`set NEURALSYNTHESIZER_AUDIO_BACKEND=jack` (Windows)

## Usage

The core API provides functions for initializing the synthesizer, loading neural network models, setting input parameters, and generating audio.



For detailed API documentation, please refer to the generated documentation by running `cargo doc --open` after building the project. This will open a web browser with the API documentation for all the crates in the project.

## Contributing

We welcome contributions to NeuralSynthesizer! Please follow these guidelines:

1.  Fork the repository.
2.  Create a new branch for your feature or bug fix.
3.  Write clear and concise code with appropriate comments.
4.  Include unit tests for your changes.
5.  Submit a pull request with a detailed description of your changes.

We encourage you to discuss your proposed changes with the project maintainers before starting work to ensure that your contributions align with the project's goals.

## License

This project is licensed under the MIT License. See the [LICENSE](https://github.com/jjfhwang/NeuralSynthesizer/blob/main/LICENSE) file for details.

## Acknowledgements

We would like to thank the developers of Rust, Tract, PortAudio, ndarray, and serde for creating the excellent libraries that make this project possible. We also acknowledge the contributions of the open-source community to the field of neural audio synthesis.