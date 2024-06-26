<!-- PROJECT SHIELDS -->
[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]

<div align="center">
  <a href="https://github.com/torcoste/relearn-rs">
    <img src="https://raw.githubusercontent.com/torcoste/relearn-rs/main/images/logo.png" alt="rlrn logo" width="80" height="80">
  </a>
  <h3 align="center">rlrn</h3>
  <p align="center">
    An awesome CLI tool for effectively learning Rust and more
  </p>
</div>

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
- `rlrn` CLI currently only supports bash and zsh. Please make sure you use one of these
- For building server crate you might need install pkg-config (`sudo apt install pkg-config`)

### Installation

1. Install cargo crate
   ```sh
   cargo install rlrn
   ```
2. Initialize app
   ```sh
   rlrn init
   ```
3. Set your daily goal and reminder interval
   <img src="https://raw.githubusercontent.com/torcoste/relearn-rs/main/images/settings.png" alt="Settings">

<!-- USAGE EXAMPLES -->
## Usage

According to the set daily goals and reminder interval, you will see the question of the quiz the next time you open a new tab of the terminal.

<img src="https://raw.githubusercontent.com/torcoste/relearn-rs/main/images/quiz-question-example.png" alt="Quiz question example">

If you want to participate in the quiz without waiting for the next reminder, you can call the quiz question at any time with a simple command:
   ```sh
   rlrn
   ```

If you want to reset your progress and settings you can do so with the command:
   ```sh
   rlrn reset
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
