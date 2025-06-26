# calc-riff

James' Q2 2025 explorations of the pico calc

## Block Diagram

![Block Diagram of the PicoCalc mainboard](./docs/picocalc-block-diagram.png)

## Pico Header Pin Mapping

| Pico Pin #    | RP2040 Pad Name   | PicoCalc Net  | Group         | Expansion?    | Note                      |
| :---          | :---              | :---          | :---          | :---          | :---                      |
| 01            | GPIO00            | UART0_TX      | USB DBG UART  | J702-3        | TOP LEFT, USB SIDE        |
| 02            | GPIO01            | UART0_RX      | USB DBG UART  | J702-2        |                           |
| 03            | GND               | GND           | -             | -             |                           |
| 04            | GPIO02            | RAM_TX        | PSRAM         | J703-2        |                           |
| 05            | GPIO03            | RAM_RX        | PSRAM         | J703-3        |                           |
| 06            | GPIO04            | RAM_I02       | PSRAM         | J703-4        |                           |
| 07            | GPIO05            | RAM_IO3       | PSRAM         | J703-5        |                           |
| 08            | GND               | GND           | -             | -             |                           |
| 09            | GPIO06            | I2C1_SDA      | SOUTHBRIDGE   | -             |                           |
| 10            | GPIO07            | I2C1_SCL      | SOUTHBRIDGE   | -             |                           |
| 11            | GPIO08            | UART1_TX      | SOUTHBRIDGE   | -             |                           |
| 12            | GPIO09            | UART1_RX      | SOUTHBRIDGE   | -             |                           |
| 13            | GND               | GND           | -             | -             |                           |
| 14            | GPIO10            | SPI1_SCK      | LCD           | -             |                           |
| 15            | GPIO11            | SPI1_TX       | LCD           | -             |                           |
| 16            | GPIO12            | SPI1_RX       | LCD           | -             |                           |
| 17            | GPIO13            | SPI1_CS       | LCD           | -             |                           |
| 18            | GND               | GND           | -             | -             |                           |
| 19            | GPIO14            | LCD_DC        | LCD           | -             |                           |
| 20            | GPIO15            | LCD_RST       | LCD           | -             | BOTTOM LEFT, SWD SIDE     |
| 21            | GPIO16            | SPI0_RX       | SDCARD        | -             | BOTTOM RIGHT, SWD SIDE    |
| 22            | GPIO17            | SPI0_CS       | SDCARD        | -             |                           |
| 23            | GND               | GND           | -             | -             |                           |
| 24            | GPIO18            | SPI0_SCK      | SDCARD        | -             |                           |
| 25            | GPIO19            | SPI0_TX       | SDCARD        | -             |                           |
| 26            | GPIO20            | RAM_CS        | PSRAM         | -             |                           |
| 27            | GPIO21            | RAM_SCK       | PSRAM         | J703-6        |                           |
| 28            | GND               | GND           | -             | -             |                           |
| 29            | GPIO22            | SD_DET        | SDCARD        | -             |                           |
| 30            | RUN               | RUN           | -             | -             |                           |
| 31            | GPIO26/A0         | PWM_L         | SOUND         | -             |                           |
| 32            | GPIO27/A1         | PWM_R         | SOUND         | -             |                           |
| 33            | ADC GND           | AGND          | -             | -             |                           |
| 34            | GPIO28/A2         | GP28          | NONE          | J703-7        |                           |
| 35            | ADC VRef          | ADC_VREF      | -             | -             |                           |
| 36            | 3v3 Out           | 3V3_OUT       | -             | -             |                           |
| 37            | 3v3 En            | 3V3_EN        | -             | -             |                           |
| 38            | GND               | GND           | -             | -             |                           |
| 39            | VSYS 5v0          | PICO_VSYS     | -             | -             |                           |
| 40            | VBUS 5v0          | VBUS          | -             | -             | TOP RIGHT, USB SIDE       |
