# Fast Image Stacker 🖼

[![Rust](https://img.shields.io/badge/Built%20with-Rust-brown)](https://www.rust-lang.org)
[![Version](https://img.shields.io/badge/Latest%20Release-0.1.0-brightgreen)](https://github.com/GarthTB/fast_image_stacker/releases)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue)](https://www.apache.org/licenses/LICENSE-2.0)

A lightweight command-line tool for stacking 8/16-bit RGB images.

## Features

- **Stacking Modes**: Choose among `mean`, `max`, or `min` pixel stacking
- **High Precision**: 32-bit floating-point arithmetic for accurate results
- **Input Support**: Processes 8-bit or 16-bit RGB images ([complete list](https://docs.rs/image/0.25.6/image/enum.ImageFormat.html))
- **Output**: Saves result as 16-bit TIFF
- **Lightweight**: Minimal dependencies and fast execution

## Usage

1. Place all the images to be processed in a subfolder named `images` within the program directory, or manually select the folder.
2. Choose a mode.

## Technical Notes

### Image Requirements

- All input images must have identical dimensions (width × height)
- RGB color format (3 channels)
- Supported bit depths: 8 or 16 bits per channel

### Processing Workflow

- Calculation accuracy: 32-bit floating-point
- Stack using selected mode:
  - Mean: Average of all pixel values
  - Max: Highest pixel value across stack
  - Min: Lowest pixel value across stack

## License

Distributed under the Apache-2.0 License. See LICENSE for details.

Author: GarthTB <g-art-h@outlook.com>

## Release Notes

### v0.1.0 - 20250411

- The first release!
