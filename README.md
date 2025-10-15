# Muzeyt (Rust tui music player)
[![Rust](https://img.shields.io/badge/Rust-stable-orange.svg)](https://www.rust-lang.org) 
[![Tokio](https://img.shields.io/badge/tokio-async-blue.svg)](https://tokio.rs)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)

A terminal-based asynchronous music queue manager written in **Rust**, built to demonstrate clean async design, safe concurrency, and idiomatic code structure.


## About 

**Muzeyt** is a terminal application showcasing my Rust  programming skills, with a focus on:

- Asynchronous architecture using [`tokio`](https://tokio.rs)
- Message passing with `tokio::sync::mpsc`
- Safe queue and navigation management without unsafe code
- Non-blocking UI updates
- Correct handling of mutable borrows across `.await` points

The project is still in progress but already demonstrates modular async design and clear state handling. 
Originally the idea was to query songs from youtube music (thus the "yt" in the name), but since there is no
youtube music api to play songs, this part of the project has been postpone for later till i can find a workaround. 
While currently the app focues on its main purpose which is showcasing my skills rather than being an actual project



## Features

- Modular architecture (App / Queue / Navigator / UI)
- Asynchronous message passing for UI updates
- Safe, non-blocking state transitions
- TUI-based interaction



## Tech Stack

| Category | Tool / Concept |
|-----------|----------------|
| Language | Rust |
| Async Runtime | [Tokio](https://tokio.rs) |
| UI | [ratatui](https://github.com/ratatui-org/ratatui)  |
| Concurrency | `tokio::sync::mpsc`, `tokio::sync::watch`, async/await |
| Logging | Custom debug logger |


## Project Structure
```
├─ audio_stream/       # Audio streaming
├─ file/               # File I/O
├─ ui/                 # UI components
├─ utility/            # Logging, helpers, and reusable utilities
├─ app.rs              # Core application
└─ main.rs             # Program entry point
```




## Screen Shots





## Current Issues/area of improvement


## To Do
- [] Testing
- [] Unloading app.rs 
- [] Making previous song be visible in queue
- [] Implement full file exploration
- [] Implement starting with file as a flag (passing the file to the player)
- [] Different file types with Symphonia


## Author

**Arison7**  
Rust Developer and Systems Programming Enthusiast  

[![GitHub](https://img.shields.io/badge/GitHub-181717?logo=github&logoColor=white)](https://github.com/Arison7)
[![LinkedIn](https://img.shields.io/badge/LinkedIn-0A66C2?logo=linkedin&logoColor=white)]([https://linkedin.com/in/arison7](https://www.linkedin.com/in/%C5%82ukasz-krysmalski-13a9a721a/))
[![Email](https://img.shields.io/badge/Email-lucaskrysmalski@gmail.com-D14836?logo=gmail&logoColor=white)](mailto:lucaskrysmalski@gmail.com)

---

## License

This project is licensed under the **MIT License** — see the [LICENSE]([LICENSE](https://mit-license.org/)) file for details.

 
