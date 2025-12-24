# Features

| Feature | Status | Notes |
| :--- | :--- | :--- |
| **Core** | | |
| AT-SPI Connection | Completed | Connected using `atspi-connection` |
| Event Loop | Completed | Async loop in `atspi_handler` |
| **Output** | | |
| TTS Engine Integration | Completed | Using `tts` crate |
| Speech Queueing | Not Started | Interrupt vs Queue |
| **Input** | | |
| Global Keybindings | Completed | Basic `rdev` loop, Insert+Q to quit |
| Focus Tracking | Completed | Reports name and role of focused items |
| **Navigation** | | |
| Linear Navigation | Not Started | Next/Prev item |
| Structural Navigation | Not Started | Headings, Links, etc. |
