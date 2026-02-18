# `container`

`container` is a minimalistic runtime for running Linux containers and managing container images. It is written in Rust, daemonless, and optimized for performance and security.

The tool consumes and produces OCI-compatible container images, so you can pull and run images from any standard container registry. You can push images you build to those registries as well, and run the images in any other OCI-compatible runtime.

Container provides full process isolation using Linux namespaces, cgroups, and a lightweight root filesystem. It is designed to be minimal and modular, making it easy to integrate into custom orchestration tools or use standalone for development and production workloads.

# Get Started
**Requirements**

You need a Linux system to run Container. It supports modern kernels with cgroups v2 and user namespaces enabled.

To build and run the project, see the BUILDING guide.

**Install**

Clone the repository and build with Cargo:

```
git clone https://github.com/horizonproductions/container.git
cd container
cargo build --release
```

The compiled binary will be in target/release/container.

**Uninstall**

Remove the binary from your system manually if installed:

```
rm /usr/local/bin/container
```

**Next Steps**

- Try running a container with a simple Linux root filesystem.
- Explore container.toml
to configure images and commands.

- Read the technical overview
 for architecture details.

- Browse the command reference
.

- Build and test the runtime with Cargo on your own development system.

# Contributing

Contributions to Container are welcome and encouraged. Please see the CONTRIBUTING.md
 guide for more information.

# Project Status

Container is currently under active development. It is stable for basic container creation and execution, but features and APIs may change until version 1.0.0 is released.
