# Lake - A Universal Build System with Lua Scripting 🚀

Welcome to **Lake**, a flexible and powerful build system that simplifies building projects with Lua scripting. Whether you are working on small tasks or complex workflows, Lake allows you to define your builds in Lua and run them with ease. It's extensible, language-agnostic, and fully customizable. 💻

## Features 🌟

- **Lua Integration**: Automate your builds with Lua scripting in the `build.lake` file.
- **Customizable Plugins**: Extend functionality with your own plugins.
- **Cross-Platform**: Runs on various platforms and environments.
- **Easy Task Management**: Quickly run and manage your tasks with simple commands.
- **Clean and Informative Errors**: Get clear feedback when something goes wrong.

## Table of Contents 📑

- [Installation](#installation-⚙️)
- [Usage](#usage-🏗️)
  - [Running Tasks](#running-tasks-🎯)
  - [Example Buildfile](#example-buildfile-📄)
- [Contributing](#contributing-🤝)
- [License](#license-📄)

## Installation ⚙️

Lake is easy to set up and get started. Follow the steps below to install it on your machine.

### Step 1: Install Rust 🦀

Make sure you have **Rust** installed. You can get it from [rust-lang.org](https://www.rust-lang.org/learn/get-started).

### Step 2: Clone the Repository 🔄

Clone the project repository to your local machine:

```bash
git clone https://github.com/sammwyy/lake.git
cd lake
```

### Step 3: Build the Project 🛠️

Use Cargo (Rust's package manager) to build Lake:

```bash
cargo build --release
```

This will generate a release version of Lake in the `target/release/` directory.

### Step 4: Run Lake 🌊

Now you're ready to use Lake! Simply run:

```bash
./target/release/lake
```

## Usage 🏗️

Lake helps you define tasks for your projects and run them with ease. Here's how to use it:

### Running Tasks 🎯

Once you’ve created your `build.lake` file, you can execute tasks like this:

```bash
lake --file path/to/your/build.lake --task task_name --args arg1 arg2
```

- `--file`: (Optional) Specify a custom `build.lake` path.
- `--task`: The task you want to run (e.g., `build`, `test`, `deploy`).
- `--args`: (Optional) Arguments passed to the task.

### Example Buildfile 📜

Here’s a simple `build.lake` file that defines a few tasks:

```lua
task("build", function()
    print("Building the project... 🚧")
    -- Add your build logic here
end)

task("test", function()
    print("Running tests... 🧪")
    -- Add your test logic here
end)

task("deploy", function()
    print("Deploying project... 🚀")
    -- Add your deploy logic here
end)
```

## Contributing 🤝

We welcome your contributions to Lake! Whether it’s fixing bugs, adding features, or improving documentation, we’d love to have you involved.

### Steps to Contribute

1. Fork the repository and clone it to your local machine.
2. Create a new branch for your feature or bugfix.
3. Make your changes and test them.
4. Submit a pull request with a clear description of what you’ve done.

## License 📄

Lake is licensed under the **MIT License**. See the [LICENSE](LICENSE) file for more information.

---

Thanks for using Lake! 🌊✨
