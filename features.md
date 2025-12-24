# Features

| Feature | Status | Notes |
| :--- | :--- | :--- |
| **Core** | | |
| AT-SPI Connection | Completed | Connected using `atspi-connection` |
| Event Loop | Completed | Async loop in `atspi_handler` |
| **Output** | | |
| TTS Engine Integration | Completed | Using `tts` crate |
| Speech Queueing | Not Started | Interrupt vs Queue |
| **Verbosity** | | |
| Window Titles | Not Started | Read window title on switch |
| Control States | Not Started | Checked, Expanded, Selected, etc. |
| Role Descriptions | Not Started | Button, Checkbox, Link, etc. |
| Typing Echo | Not Started | Characters and Words |
| **Input** | | |
| Global Keybindings | Completed | AT-SPI `DeviceEventListener` (Wayland compatible), Insert+Q to quit |
| Focus Tracking | Completed | Reports name and role of focused items |
| **Navigation** | | |
| Linear Navigation | Not Started | Next/Prev item |
| Structural Navigation | Not Started | Headings, Links, etc. |
