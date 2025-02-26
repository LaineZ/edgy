# Edgy - embedded discreet graphics, yes

Portable, lightweight and robust `no_std` immediate-mode GUI library targeted for microcontrollers. Uses [embedded_graphics](https://github.com/embedded-graphics/embedded-graphics) library for rendering therefore supports a great portion of displays out of box; and shares some types like `Rectangle` or `Size`.

Library uses ``alloc`` for widget dynamic dispatch, threfore a allocator is required.

Work in progress, something is work, but is very ROUGH shape and requires polishing and improvements in many terms, including but not limited: code quality, documentation, features

# System requirements

Any device with ``alloc`` support and at least 32 KiB flash minimum. For example, STM32F103C6T6 - 32K Flash, can run basic UI's, with small amount of widgets. More complex setups requires something like ESP32-C3, there have a 4 MiB of flash, more RAM, and basically faster processor for a reasonable price.

# Display support

Any display that supports [embedded_graphics](https://github.com/embedded-graphics/embedded-graphics) library.

# Input device support

Since, this library designed mainly as immediate mode, there is not bound to specific device, you can build any event system atop of library, or using ``SystemEvent`` enum with `ui_context.push_event()` function. Check out examples for more info!