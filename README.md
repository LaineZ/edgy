![edgy](./logo.png)

# Edgy - embedded discreet graphics, yes

Portable, lightweight and robust `no_std` hybrid-mode (immidiate mode drawing and tree, and retained event handling) GUI library targeted for microcontrollers. Uses [embedded_graphics](https://github.com/embedded-graphics/embedded-graphics) library for rendering therefore supports a great portion of displays out of box; and shares some types like `Rectangle` or `Size`.

Library uses ``alloc`` for widget dynamic dispatch, threfore a allocator is required.

Work in progress, something is work, but is very ROUGH shape and requires polishing and improvements in many terms, including but not limited: code quality, documentation, features.

# System requirements

Mandatory: `alloc` support

## Minimum:

Tested on STM32F103C6T6 MCU. It's run fine with small number of widgets. Therefore minimal system requirements something like this

* Flash: 32 KiB
* RAM: depends on the number of widgets (but generally quite small)

## Recommended:

Tested on ESP32C3 MCU. Runs absolutly fine in 500+ FPS (excluding display transactions overhead which depends on display device)

* Flash: 4 MiB
* RAM: 400+ KiB

# Display support

Any display that supports [embedded_graphics](https://github.com/embedded-graphics/embedded-graphics) library. Keep in mind, current style implementation very limited for `BinaryColor` displays, and there is no BinaryColor-friendly theme by default

# Input device support

There is not bound to specific device, you can build any event system atop of library, or using ``SystemEvent`` enum with `ui_context.push_event()` function. Check out examples for more info!