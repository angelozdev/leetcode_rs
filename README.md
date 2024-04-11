# LeetCode Solutions

Welcome to my repository of LeetCode solutions! This repository is a collection of my attempts and solutions to various LeetCode problems, organized by difficulty. It's a work in progress as I continue to solve more problems and update this repository.

## Structure

The repository is structured into two main parts: the `lib.rs` file, which contains modules organized by difficulty and further divided into individual problems, and the `main.rs` file, where I test and run the solutions for the current problem I am working on.

### `lib.rs`

The `lib.rs` file acts as the library root and is organized into two main modules representing the difficulty levels of the problems: `easy` and `medium`.

- **Easy Module**: Contains simpler problems that serve as a good starting point for beginners or as warm-up exercises.
  - `two_sum`: Solution for the "Two Sum" problem.
  - `valid_palindrome_ii`: Solution for the "Valid Palindrome II" problem.

- **Medium Module**: Houses more challenging problems that require a deeper understanding and more complex solutions.
  - `top_k_frequent_elements`: Solution for the "Top K Frequent Elements" problem.

### `main.rs`

The `main.rs` file is where I execute the solution for the current problem I am tackling. It serves as a testing ground to run and debug my code before finalizing the solution.

## Usage

To run a solution, navigate to the `main.rs` file and call the desired function from the module that corresponds to the problem's difficulty and name. For example, to run the `two_sum` solution, you would modify `main.rs` to call `easy::two_sum::function_name(args)`.

## Contributing

While this repository is primarily for my personal use as I progress through LeetCode problems, contributions, suggestions, and discussions are welcome! If you have improvements or alternative solutions to the problems, feel free to open an issue or pull request.

## License

This repository is open-sourced under the MIT License. See the LICENSE file for more details.

---

Happy coding!
