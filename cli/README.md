<!-- PROJECT SHIELDS -->
[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]

<p align="center">
  <h3 align="center">rlrn</h3>
  <p align="center">
    An awesome CLI tool for effectively learning Rust and more
  </p>
</p>

<!-- TABLE OF CONTENTS -->
<details open="open">
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
      <ul>
        <li><a href="#built-with">Built With</a></li>
      </ul>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#roadmap">Roadmap</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
  </ol>
</details>


<!-- ABOUT THE PROJECT -->
## About The Project

ReLeaRN or `rlrn` is a CLI tool, allowing Rustaceans to learn, improve and maintain their Rust skills!
`rlrn` uses simple quizzes combined with a **Spaced Repitition** engine to help you find and overcome with Rust weakspots and improve your software engineering skills.

`rlrn` gamifies learning Rust and brings your friends and colleagues together in your daily learning, with badges (as NFTs), progress bars, and leaderboards (both public and company/clan) among other features.

### Built With

ReLeaRN is built with Rust end-to-end, and uses Open Source Rust libraries to create an interface to the engine.

<!-- GETTING STARTED -->
## Getting Started

### Prerequisites

- You should have a Unix-like OS (Linux, macOS, Windows **with WSL only**)
- You should have [rust and cargo](https://www.rust-lang.org/tools/install) installed on your computer
- `rlrn` currently only supports bash and zsh. Please make sure you use one of these

### Installation

1. Install cargo crate
   ```sh
   cargo install rlrn
   ```
2. Instialize app to see quiz when you open new terminal
   ```sh
   rlrn init
   ```

<!-- USAGE EXAMPLES -->
## Usage

```sh
rlrn
```

<!-- ROADMAP -->
## Roadmap

`rlrn` can be used as engine with any kind of learning, including non-programming topics such medical exams, driving exams and tests etc.

See the [open issues](https://github.com/torcoste/relearn-rs/issues) for a list of proposed features (and known issues).

<!-- CONTRIBUTING -->
## Contributing

Contributions are what make the open source community such an amazing place to be learn, inspire, and create. Any contributions you make are **greatly appreciated**.

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

<!-- LICENSE -->
## License

Distributed under the MIT License. See `LICENSE` for more information.

<!-- REFERENCES -->
[contributors-shield]: https://img.shields.io/github/contributors/torcoste/relearn-rs.svg?style=for-the-badge
[contributors-url]: https://github.com/torcoste/relearn-rs/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/torcoste/relearn-rs.svg?style=for-the-badge
[forks-url]: https://github.com/torcoste/relearn-rs/network/members
[stars-shield]: https://img.shields.io/github/stars/torcoste/relearn-rs.svg?style=for-the-badge
[stars-url]: https://github.com/torcoste/relearn-rs/stargazers
[issues-shield]: https://img.shields.io/github/issues/torcoste/relearn-rs.svg?style=for-the-badge
[issues-url]: https://github.com/torcoste/relearn-rs/issues
[license-shield]: https://img.shields.io/github/license/torcoste/relearn-rs.svg?style=for-the-badge
[license-url]: https://github.com/torcoste/relearn-rs/blob/master/LICENSE.txt
