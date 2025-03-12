# Presence Detection System

A complete IoT solution for accurate presence detection using mmWave radar technology, Bluetooth 5.0 LE, plus a cross-platform desktop application.

## Overview

This project consists of two main components:
1. **IoT Device**: ESP32-C6 based hardware with DFRobot SEN0395 mmWave radar sensor
2. **Desktop Application**: Cross-platform application built with Tauri, Svelte, TypeScript, and Tailwind CSS

## IoT Device

### Hardware Components

- **Microcontroller**: ESP32-C6 (featuring Wi-Fi 6, Bluetooth 5.0 LE, Zigbee 3.0, and Thread 1.3)
- **Sensor**: DFRobot SEN0395 24GHz mmWave radar sensor
- **Power**: 3.6-5V DC input (USB-C connector)
- **Connectivity**: Multiple protocols (Wi-Fi, BLE, Zigbee, Thread)
- **Optional**: RGB LED for status indication, additional GPIO connections for external triggers

### Features

- Presence detection using mmWave radar technology (detects both stationary and moving humans)
- Multi-target tracking with precise distance, angle, and movement information
- Accurate detection even for sleeping persons or minimal movement
- Up to 9m detection range with 100°×40° beam angle
- Configurable detection zones and parameters via serial interface
- Confidence level reporting for detection reliability assessment
- Movement direction determination (approaching, receding, lateral movement)
- Low power consumption (90mA typical operating current)
- Strong anti-interference ability against environmental factors (temperature, humidity, dust, light, etc.)
- LED status indicators for easy troubleshooting

### Setup Instructions

1. **Hardware Assembly**
   - Connect the SEN0395 mmWave radar sensor to the ESP32-C6 board:
     - VCC → 3.3V or 5V (3.3V recommended)
     - GND → GND
     - UART Tx → GPIO17 (RX pin on ESP32-C6)
     - UART Rx → GPIO16 (TX pin on ESP32-C6)
     - GPIO2 → GPIO6 (for direct presence detection output)
   - Position the sensor following recommended mounting guidelines (top, underneath, or horizontal installation)
   - Power the device using a USB-C cable

2. **Firmware Installation**
   - Clone this repository
   - Open the Arduino IDE
   - Install ESP32-C6 board support:
     - Go to File → Preferences
     - Add this URL to the "Additional Boards Manager URLs": https://raw.githubusercontent.com/espressif/arduino-esp32/gh-pages/package_esp32_index.json
     - Go to Tools → Board → Boards Manager
     - Search for "esp32" and install the latest version
   - Select the appropriate board: Tools → Board → ESP32 Arduino → ESP32-C6 Dev Module
   - Open the firmware sketch from the `/firmware/presence_detection` directory
   - Install required libraries via Library Manager (Tools → Manage Libraries)
   - Connect your ESP32-C6 board via USB
   - Select the correct port under Tools → Port
   - Click Upload button to flash the firmware

3. **Sensor Configuration**
   - The default configuration supports basic presence detection
   - For advanced configuration:
     - Use serial commands to adjust detection range (0-9m)
     - Configure output delay parameters
     - Set up multiple detection zones if needed
   - Use the desktop application to manage device settings or send direct UART commands at 115200 baud

### Technical Specifications

#### ESP32-C6 Microcontroller
- **Wi-Fi**: 2.4 GHz Wi-Fi 6 (802.11ax)
- **Bluetooth**: Bluetooth 5.0 LE
- **Other Protocols**: Zigbee 3.0, Thread 1.3 (IEEE 802.15.4)
- **Flash Memory**: 8MB SPI flash
- **Operating Voltage**: 3.3V (with onboard 5V to 3.3V LDO)
- **USB Interface**: USB 2.0 full-speed (12 Mbps)

#### mmWave Radar Sensor (SEN0395)
- **Technology**: FMCW (Frequency Modulated Continuous Wave) radar
- **Frequency**: 24GHz millimeter-wave
- **Detection Range**: Up to 9 meters
- **Detection Angle**: 100° horizontal, 40° vertical
- **Power Consumption**: 90mA operating, can be optimized with configurable parameters
- **Operating Voltage**: 3.6-5V
- **Communication**: Serial UART (115200 baud) and GPIO outputs
- **Firmware Updates**: Serial port configuration and updates supported
- **Operating Temperature**: -40~85℃

## Desktop Application

### Features

- Real-time presence detection visualization with radar display
- Multi-target tracking with distance, angle, and movement direction
- Confidence level monitoring and display
- Historical data logging and analysis
- Multiple device management
- Customizable alerts and notifications
- Device configuration and firmware updates

### Visualization Interface

![Radar Visualization Interface](./docs/images/radar_visualization.png)

The application features an advanced radar-style visualization that displays:

- **Multiple Target Tracking**: Detect and track up to several individuals simultaneously
- **Spatial Mapping**: View targets on a polar coordinate system with distance rings
- **Movement Analysis**: See direction of movement (approaching, receding, left, right)
- **Detailed Metrics**:
  - Precise distance measurements (accuracy to 0.01m)
  - Confidence levels (0-255 scale)
  - Movement status (moving or stationary)
  - Target IDs for consistent tracking
- **Real-time Updates**: Continuous data refresh from the mmWave sensor

### Installation

#### Pre-built Binaries
Download the latest release for your platform from the [Releases](https://github.com/yourusername/presence-detection/releases) page.

#### Build from Source
1. Clone this repository
2. Navigate to the `/app` directory
3. Install Rust if not already installed (https://rustup.rs/)
4. For development:
   ```
   cargo tauri dev
   ```
5. For production build:
   ```
   cargo tauri build
   ```

### Key Application Features

#### 1. Dashboard
- At-a-glance system status
- Real-time occupancy information
- Quick access to all devices and settings

#### 2. Radar View
- Interactive radar visualization as shown above
- Adjustable view parameters (range, scale, refresh rate)
- Historical trail visualization for movement patterns
- Filtering options (by distance, confidence, movement)

#### 3. Analytics
- Occupancy trends and patterns
- Heatmap generation for space utilization
- Movement flow analysis
- Data export for external processing

#### 4. Device Management
- Add, configure, and update multiple devices
- Sensor parameter adjustment
- Firmware updates
- Power management settings

### System Requirements

- **Windows**: Windows 10 or later
- ~~**macOS**: macOS 10.15 (Catalina) or later~~ (macOS support at a later release)
- ~~**Linux**: Ubuntu 20.04 or equivalent~~ (Linux support at a later release)
- **Connectivity**: One of the following:
  - ~~Wi-Fi (2.4 GHz)~~ (Future release)
  - Bluetooth 5.0 compatible
  - ~~Thread or Zigbee network (if using those protocols)~~ (Future release)

## Development

### Repository Structure

```
presence-detection/
├── firmware/                 # ESP32-C6 firmware code
│   ├── presence_detection/   # Main sketch
│   ├── libraries/            # Custom libraries
│   │   ├── mmWaveRadar/      # mmWave radar driver library
│   │   └── PresenceUtils/    # Utility functions
│   └── examples/             # Example sketches
├── app/                      # Desktop application
│   ├── src/                  # Svelte components and TypeScript code
│   ├── src-tauri/            # Tauri (Rust) backend code
│   ├── public/               # Static assets
│   └── build/                # Build output
├── hardware/                 # Hardware design files
│   ├── schematics/           # Circuit schematics
│   └── enclosure/            # 3D printable enclosure files
└── docs/                     # Documentation
    ├── api/                  # API documentation
    ├── protocol/             # Communication protocol specifications
    ├── mmwave_commands/      # mmWave sensor command reference
    └── user-guide/           # User guides and tutorials
```

### Build Requirements

#### Firmware
- Arduino IDE 2.0 or later
- ESP32 Arduino Core (using Boards Manager)
- Required libraries:
  - Arduino library for mmWave Radar SEN0395
  - ArduinoBLE (for BLE functionality)

#### Desktop Application
- Rust 1.65 or later
- Platform-specific build dependencies for Tauri:
  - **Windows**: Microsoft Visual Studio C++ Build Tools
  - **macOS**: Xcode Command Line Tools
  - **Linux**: `build-essential`, `libwebkit2gtk-4.0-dev`, `libssl-dev`, and other Tauri dependencies 

### Communication Protocols

The system supports multiple communication methods:

1. **Serial Communication with mmWave Sensor**
   - UART at 115200 baud rate, 8 data bits, no parity, 1 stop bit
   - ASCII command strings format for configuration
   - Structured data output for presence detection results
   - Extended protocol for detailed target information (distance, angle, confidence, movement)
   - Details in the [mmWave sensor protocol documentation](./docs/protocol/mmwave_protocol.md)

2. **ESP32-C6 to Application Communication**
   - **Bluetooth LE**: Custom GATT service for presence data and configuration
   - **USB**: Serial communication for debugging and direct control
   - Real-time data streaming protocol for multi-target information
   - Full protocol specifications in the [communication documentation](./docs/protocol/README.md)

3. **Data Format**
   - JSON-structured data for target information:
     ```json
     {
       "targets": [
         {
           "id": 0,
           "distance": 2.75,
           "angle": -15.2,
           "confidence": 204,
           "movement": "approaching",
           "velocity": 0.3,
           "isActive": true
         },
         ...
       ]
     }
     ```

## Troubleshooting

- WiP

## Use Cases

### Smart Home
- Room presence detection for lighting and HVAC control
- Security monitoring with multi-person tracking
- Sleep monitoring without privacy concerns of cameras
- Energy optimization based on occupancy patterns

### Office Environments
- Meeting room utilization analytics
- Desk occupancy monitoring
- Traffic flow analysis for space optimization
- Triggering of presentation systems based on presence

### Retail and Public Spaces
- Customer flow analysis
- Queue management
- Occupancy counting with directional information
- Triggering of interactive displays when approached

### Industrial Applications
- Safety zone monitoring
- Process automation based on worker presence
- Equipment activation/deactivation based on proximity
- Unauthorized access detection

## Contributing

N/A

## License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.

## Acknowledgments

- [DFRobot](https://www.dfrobot.com/) for the SEN0395 mmWave radar sensor
- [Espressif](https://www.espressif.com/) for the ESP32-C6 microcontroller
- [ArduinoIDE](https://docs.espressif.com/projects/arduino-esp32/en/latest/getting_started.html#about-arduino-esp32) for Arduino IDE and ESP32 Support
- [Tauri](https://tauri.app/) for the desktop application framework
- [Svelte](https://svelte.dev/) for the frontend framework
- [Tailwind CSS](https://tailwindcss.com/) for UI styling

## References

- [ESP32-C6 Technical Documentation](https://www.espressif.com/en/products/socs/esp32-c6)
- [DFRobot SEN0395 mmWave Radar Sensor Documentation](https://wiki.dfrobot.com/mmWave_Radar_Human_Presence_Detection_SKU_SEN0395)
- [ESP-IDF Programming Guide](https://docs.espressif.com/projects/esp-idf/en/latest/esp32c6/index.html)

