# NeuralSynthesizer: Bayesian Meta-Learning for Compositional Generalization

NeuralSynthesizer is a Rust-based research project focused on developing a novel Bayesian meta-learning framework leveraging differentiable neural architecture search (NAS) to achieve robust compositional generalization in abstract reasoning tasks. This repository provides the implementation of our proposed approach, along with the tools and scripts necessary to reproduce our experimental results and extend the framework to new domains.

Our work tackles the critical challenge of enabling neural networks to extrapolate beyond their training data by explicitly learning compositional rules and structures. NeuralSynthesizer achieves this through a two-stage process. First, a differentiable NAS engine explores a vast space of neural architectures, guided by a Bayesian meta-learner that prioritizes architectures demonstrating strong generalization performance across a diverse set of abstract reasoning tasks. This meta-learner effectively learns a prior distribution over architectures that are well-suited for compositional generalization. Second, the selected architecture is fine-tuned on a specific target task, enabling it to quickly adapt and solve complex reasoning problems even with limited training data. The differentiable approach allows for efficient gradient-based optimization of the architecture, overcoming the limitations of traditional discrete NAS methods. This results in a system that is not only capable of solving the reasoning tasks presented during training but also exhibits a remarkable ability to generalize to novel combinations of rules and concepts.

The primary goal of this project is to provide researchers and practitioners with a powerful and flexible tool for exploring the intersection of meta-learning, neural architecture search, and abstract reasoning. We believe that this work contributes significantly to the development of more robust and generalizable AI systems capable of tackling complex cognitive tasks. By providing a well-documented and easy-to-use implementation of our framework, we hope to encourage further research in this exciting area. The repository includes detailed instructions for setting up the environment, running experiments, and contributing to the project.

## Key Features

*   **Differentiable Neural Architecture Search (NAS):** Employs a continuous relaxation of the architecture search space, enabling gradient-based optimization of the neural network architecture. This significantly improves search efficiency compared to discrete NAS methods. Specifically, we implement a Gumbel-Softmax relaxation over architectural parameters, allowing for end-to-end training of the architecture and weights.
*   **Bayesian Meta-Learning:** Integrates a Bayesian meta-learner to guide the NAS process, favoring architectures that exhibit strong generalization across a distribution of abstract reasoning tasks. This prior knowledge facilitates rapid adaptation to new tasks. The Bayesian framework utilizes a variational inference approach to approximate the posterior distribution over architectures given the observed performance on meta-training tasks.
*   **Compositional Generalization:** Designed specifically to address the challenge of compositional generalization in abstract reasoning. The architecture search process is driven by the objective of discovering architectures that are capable of learning and applying compositional rules.
*   **Rust Implementation:** Leveraging the performance and safety guarantees of Rust, the implementation is highly efficient and reliable. This is crucial for computationally intensive tasks such as NAS and meta-learning.
*   **Modular Design:** The codebase is structured in a modular fashion, allowing for easy extension and customization. Individual components, such as the NAS engine, meta-learner, and abstract reasoning task generators, can be modified or replaced as needed.
*   **Reproducible Experiments:** Includes scripts and configurations for reproducing the experimental results presented in our research paper. This ensures the transparency and verifiability of our findings.
*   **Extensive Documentation:** Provides comprehensive documentation, including API references and usage examples, to facilitate the understanding and application of the framework.

## Technology Stack

*   **Rust:** The primary programming language, chosen for its performance, safety, and concurrency features.
*   **Tensors:** We use a custom tensor library built on top of the `ndarray` crate to provide efficient tensor operations.
*   **Backpropagation:** Implemented via forward and backward passes to facilitate differentiation.
*   **Meta-Learning Framework:** A custom meta-learning framework is built on top of the tensor library to support various meta-learning algorithms.
*   **Abstract Reasoning Task Generator:** A custom generator is created to provide abstract reasoning tasks, based on the Raven's Progressive Matrices paradigm.

## Installation

1.  **Install Rust:** If you don't have Rust installed, download and install it from [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).
2.  **Clone the repository:**
    
3.  **Build the project:**
    
    This will build the project in release mode, which is recommended for performance.
4.  **Install dependencies:** Some dependencies may require system-level packages. Refer to the documentation of the crates used within the project for specific installation instructions.

## Configuration

The project uses environment variables for configuration. Create a `.env` file in the root directory of the project and set the following variables:

*   `TASK_GENERATOR_SEED`: Seed for the random number generator used to generate abstract reasoning tasks. Defaults to 0.
*   `META_LEARNING_RATE`: Learning rate for the meta-learner. Defaults to 0.001.
*   `ARCHITECTURE_LEARNING_RATE`: Learning rate for the architecture parameters. Defaults to 0.001.
*   `NUM_META_TRAINING_TASKS`: Number of tasks used for meta-training. Defaults to 100.
*   `NUM_META_TESTING_TASKS`: Number of tasks used for meta-testing. Defaults to 20.
*   `ARCHITECTURE_SEARCH_ITERATIONS`: Number of iterations for architecture search. Defaults to 1000.

Example `.env` file:

The project also supports command-line arguments, which can be used to override the environment variables.

## Usage

The main entry point of the project is the `src/main.rs` file. To run the project, use the following command:

Where `[options]` can be any of the following:

*   `--task_generator_seed <seed>`: Sets the task generator seed.
*   `--meta_learning_rate <learning_rate>`: Sets the meta-learning rate.
*   `--architecture_learning_rate <learning_rate>`: Sets the architecture learning rate.
*   `--num_meta_training_tasks <num_tasks>`: Sets the number of meta-training tasks.
*   `--num_meta_testing_tasks <num_tasks>`: Sets the number of meta-testing tasks.
*   `--architecture_search_iterations <num_iterations>`: Sets the number of iterations for architecture search.

Example usage:


Detailed API documentation for the core components of the framework is available in the `docs/` directory.

## Contributing

We welcome contributions to the NeuralSynthesizer project. To contribute, please follow these guidelines:

1.  Fork the repository.
2.  Create a new branch for your feature or bug fix.
3.  Implement your changes, ensuring that the code is well-documented and adheres to the project's coding style.
4.  Write unit tests to verify the correctness of your changes.
5.  Submit a pull request with a clear description of your changes.

## License

This project is licensed under the MIT License. See the [LICENSE](https://github.com/jjfhwang/NeuralSynthesizer/blob/main/LICENSE) file for details.

## Acknowledgements

This project was supported by [Insert Funding Source/Organization Here, if applicable]. We would also like to thank [Insert any individuals who contributed to the project outside of direct code contributions here] for their valuable contributions to this research.