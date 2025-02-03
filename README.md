# Edgy - embedded discreet graphics, yes

Portable, lightweight and robust `no_std` immediate-mode GUI library targeted for microcontrollers. Uses [embedded_graphics](https://github.com/embedded-graphics/embedded-graphics) library for rendering therefore supports a great portion of displays out of box; and shares some types like `Rectangle` or `Size`.

Library uses ``alloc`` for widget dynamic dispatch, threfore a allocator is required.


Work in progress, nothing works well atm...