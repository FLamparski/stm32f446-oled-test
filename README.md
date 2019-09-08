# SSD1306 OLED examples in Rust on the STM32F446RE

More stuff and proper writeup coming soon.

![Rustacean on the OLED](https://pbs.twimg.com/media/ED9u7lYX4AA3Mnl?format=jpg&name=4096x4096)

## Converting images to SSD1306 bitmaps

Use ImageMagick's `convert` tool:

```sh
convert rustacean-1bit.png -depth 1 rustacean.data
```

## launch.json and tasks.json for VS Code

Together these files will add a debug target in VS Code which builds the crate, flashes it to
the MCU, and resets into a debug breakpoint.

### tasks.json
```json
{
    "version": "2.0.0",
    "tasks": [
        {
            "label": "cargo build",
            "type": "cargo",
            "subcommand": "build",
            "problemMatcher": [
                "$rustc"
            ],
            "group": "build"
        }
    ]
}
```

### launch.json

```json
{
    "version": "0.2.0",
    "configurations": [
        {
            "type": "gdb",
            "request": "launch",
            "name": "Debug Microcontroller",
            "preLaunchTask": "cargo build",
            "target": "${workspaceFolder}\\target\\thumbv7em-none-eabi\\debug\\stm32f446-oled-test",
            "cwd": "${workspaceRoot}",
            "gdbpath": "path\\to\\arm-none-eabi-gdb.exe",
            "autorun": [
                "target remote :3333",
                "monitor arm semihosting enable",
                "load",
                "step"
            ]
        }
    ]
}
```

## Acknowledgements

* Ferris the crab logo is from https://rustacean.net/.