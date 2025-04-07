<div align="center">
  <h1>Environmental Monitoring System</h1>
  <p>
    <img src="https://img.shields.io/badge/ESP32-S3-blue?style=flat-square" alt="ESP32-S3">
    <img src="https://img.shields.io/badge/Tauri-1.x-purple?style=flat-square" alt="Tauri">
    <img src="https://img.shields.io/badge/Svelte-3.x-orange?style=flat-square" alt="Svelte">
    <img src="https://img.shields.io/badge/TypeScript-5.x-blue?style=flat-square" alt="TypeScript">
    <img src="https://img.shields.io/badge/BME680-Sensor-green?style=flat-square" alt="BME680">
    <img src="https://img.shields.io/badge/MQTT-Protocol-brightgreen?style=flat-square" alt="MQTT">
  </p>
  <p>A Simple IoT Project for environmental monitoring using BME680 sensor technology, MQTT, and a cross-platform desktop application.</p>
</div>

## 📋 Overview

This project consists of two main components:
1. **IoT Device**: Lilygo T-Display S3 AMOLED with Adafruit BME680 environmental sensor
2. **Desktop Application**: Cross-platform application built with Tauri, Svelte, TypeScript, and Tailwind CSS

## 🔧 Technology Choices

This project leverages a carefully selected technology stack to provide an optimal balance of performance, developer experience, and cross-platform compatibility.

### Why Tauri + Rust

- **Incredibly Light Footprint**: Tauri applications are significantly smaller than Electron alternatives (3-10MB vs 100MB+), resulting in faster downloads, updates, and less resource usage.
- **Enhanced Security**: Rust's memory safety guarantees provide robust security without runtime overhead.
- **Performance Efficiency**: Rust's low-level control and zero-cost abstractions make it well-suited for processing real-time sensor data streams without introducing latency or performance bottlenecks.
- **Cross-Platform Compatibility**: Build once and deploy across Windows, macOS, Linux, and mobile with native look and feel, dramatically reducing maintenance overhead.

### Why Svelte + TypeScript

- **Reactive by Design**: Svelte's built-in reactivity system is ideal for displaying continuously updating sensor data with minimal code and maximum performance.
- **Reduced Bundle Size**: Unlike traditional frameworks that ship their entire runtime to the browser, Svelte compiles components into highly optimized vanilla JavaScript at build time. This "disappearing framework" approach eliminates runtime overhead and results in significantly smaller bundles, perfectly complementing Tauri's lightweight philosophy.
- **Type Safety**: TypeScript provides compile-time error checking, better IDE support, and self-documenting code through its type system. This reduces runtime errors and simplifies maintenance, particularly valuable when multiple developers collaborate on a complex codebase.
- **Developer Experience**: Svelte's straightforward, declarative approach to building UIs reduces development time and makes maintenance more intuitive.

### MQTT for Connectivity

The project uses MQTT for device-to-application communication, offering several advantages:

- **Lightweight Protocol**: Designed for constrained devices and low-bandwidth, high-latency networks—perfect for IoT applications.
- **Publish/Subscribe Pattern**: Enables flexible data distribution with minimal network overhead.
- **Quality of Service Options**: Configurable message delivery guarantees ensure critical environmental data is never lost.
- **Widespread Support**: Robust client libraries and broker implementations across platforms.

The desktop application leverages the operating system's native networking capabilities for WiFi connectivity, simplifying setup and improving reliability across platforms.

### Tailwind CSS for UI

- **Utility-First Approach**: Enables rapid UI development without leaving your HTML/Svelte files.
- **Consistent Design System**: Provides a cohesive look and feel across the entire application.
- **Minimal CSS Overhead**: Only includes the styles you actually use, keeping the application lightweight.

Together, this technology stack delivers a responsive, efficient, and secure application capable of handling complex real-time sensor data while providing an intuitive and pleasant user experience.

## 📡 IoT Device

### Hardware Components

<table>
  <tr>
    <td><b>Microcontroller</b></td>
    <td>Lilygo T-Display S3 AMOLED (with ESP32-S3 chip, built-in display)</td>
  </tr>
  <tr>
    <td><b>Sensor</b></td>
    <td>Adafruit BME680 (temperature, humidity, pressure, gas, altitude)</td>
  </tr>
  <tr>
    <td><b>Power</b></td>
    <td>3.6-5V DC input (USB-C connector)</td>
  </tr>
  <tr>
    <td><b>Connectivity</b></td>
    <td>Wi-Fi</td>
  </tr>
  <tr>
    <td><b>Display</b></td>
    <td>Built-in AMOLED display for on-device data visualization</td>
  </tr>
</table>

### Features

- ✅ Comprehensive environmental monitoring with BME680 sensor
- ✅ Temperature measurement with high accuracy
- ✅ Humidity sensing for ambient moisture levels
- ✅ Barometric pressure monitoring
- ✅ Air quality assessment through gas resistance measurement
- ✅ Altitude calculation based on pressure readings
- ✅ Built-in AMOLED display for real-time data visualization
- ✅ Custom UI created with Squareline Studio
- ✅ MQTT connectivity for data transmission to desktop application
- ✅ Low power consumption design
- ✅ Status indicators for easy troubleshooting

### Setup Instructions

1. **Hardware Assembly**
   - Connect the BME680 sensor to the Lilygo T-Display S3 board using SPI:
     - VIN → 3.3V (use the same voltage that the microcontroller logic is based on)
     - GND → GND (common power/data ground)
     - SCK → GPIO 15
     - SDO → GPIO 14
     - SDI → GPIO 13
     - CS → GPIO 12
   - Position the sensor in an appropriate location for accurate readings
   - Power the device using a USB-C cable

2. **Firmware Installation**
   - Clone this repository
   - Open the project in PlatformIO
   - Configure WiFi and MQTT settings in the config file
   - Build and upload the firmware to the Lilygo T-Display S3
   - The device will boot up and display the UI created with Squareline Studio

3. **Sensor Configuration**
   - The default configuration supports basic environmental monitoring
   - For advanced configuration:
     - Modify sampling rates for different environmental parameters
     - Adjust filtering parameters for noise reduction
     - Configure display settings and UI elements
   - Use the desktop application to manage device settings

### Technical Specifications

<details>
<summary><b>Lilygo T-Display S3 AMOLED</b></summary>

- **Processor**: ESP32-S3 dual-core processor
- **Display**: Built-in AMOLED display
- **Wi-Fi**: 2.4 GHz Wi-Fi
- **Flash Memory**: Onboard flash
- **Operating Voltage**: 3.3V
- **USB Interface**: USB-C for power and programming
</details>

<details>
<summary><b>BME680 Environmental Sensor</b></summary>

- **Temperature Range**: -40 to +85°C
- **Temperature Accuracy**: ±1.0°C
- **Humidity Range**: 0-100% RH
- **Humidity Accuracy**: ±3% RH
- **Pressure Range**: 300-1100 hPa
- **Pressure Accuracy**: ±0.6 hPa
- **Gas Sensor**: VOC gas sensor with resistance range of 0-10000 ohms
- **Interface**: SPI (as configured in this project)
- **Operating Voltage**: 3.3V to 5V
- **Current Consumption**: 3.7 μA (sleep mode) to 12 mA (operation)
</details>

## 💻 Desktop Application

### Features

- Real-time environmental data visualization
- Historical data tracking and analysis
- Multiple sensor parameter display

### Visualization Interface

The application features comprehensive environmental data visualization that displays:

- **Temperature Monitoring**: Real-time temperature readings with historical trends
- **Humidity Tracking**: Current humidity levels with visual indicators
- **Pressure Analysis**: Barometric pressure readings and changes over time
- **Air Quality Assessment**: Gas resistance measurements for air quality monitoring
- **Altitude Display**: Calculated altitude based on pressure readings
- **Historical Data**: Timeline view of all environmental parameters
- **Data Export**: Options to export collected data for further analysis
- **Light/Dark Mode**: UI theme switching for comfortable viewing

### Installation

#### TODO

### Key Application Features

#### 1. Dashboard
- At-a-glance system status
- Real-time environmental information
- Visual indicators for parameter status (normal, warning, alert)

#### 2. Data History
- Historical data visualization
- Time-based filtering options
- Data comparison across different time periods

#### 3. MQTT Management
- Connection configuration
- Topic management
- Connection status monitoring

#### 4. Device Management
- Sensor parameter adjustment
- Display configuration
- Update management

### System Requirements

<table>
  <tr>
    <td><b>Windows</b></td>
    <td>Windows 10 or later</td>
  </tr>
  <tr>
    <td><b>macOS</b></td>
    <td><s>macOS 10.15 (Catalina) or later</s></td>
  </tr>
  <tr>
    <td><b>Linux</b></td>
    <td><s>Ubuntu 20.04 or equivalent</s></td>
  </tr>
  <tr>
    <td><b>Connectivity</b></td>
    <td>
      Wi-Fi (2.4 GHz)
    </td>
  </tr>
</table>

## 🛠️ Development

### Repository Structure

```
environmental-monitor/
├── firmware/                 # ESP32-S3 firmware code
│   ├── src/                  # Main PlatformIO source code
│   ├── include/              # Header files
│   ├── lib/                  # Custom libraries
│   │   ├── BME680_Driver/    # BME680 sensor driver library
│   │   └── EnvironmentUtils/ # Utility functions
│   └── ui/                   # Squareline Studio UI files
├── app/                      # Desktop application
│   ├── src/                  # Svelte components and TypeScript code
│   ├── src-tauri/            # Tauri (Rust) backend code
│   ├── public/               # Static assets
│   └── build/                # Build output
└── docs/                     # Documentation
```

### Build Requirements

<details>
<summary><b>Firmware</b></summary>

- PlatformIO IDE (VSCode extension or CLI)
- Required libraries:
  - Adafruit BME680 Library (configured for SPI)
  - Adafruit Unified Sensor
  - Arduino WiFi libraries
  - Arduino MQTT libraries
  - TFT_eSPI for display control
  - Squareline Studio generated UI code
</details>

<details>
<summary><b>Desktop Application</b></summary>

- Rust 1.65 or later
- pnpm
- Platform-specific build dependencies for Tauri:
  - **Windows**: Microsoft Visual Studio C++ Build Tools
  - ~~**macOS**: Xcode Command Line Tools~~
  - ~~**Linux**: `build-essential`, `libwebkit2gtk-4.0-dev`, `libssl-dev`, and other Tauri dependencies~~
</details>

### Communication Protocols

The system supports multiple communication methods:

1. **SPI Communication with BME680 Sensor**
   - SPI protocol for reading sensor data (pins 15, 14, 13, 12)
   - Configurable sampling rates and filtering parameters
   - Structured data output for environmental readings

2. **ESP32-S3 to Desktop Communication**
   - **Wi-Fi**: Leverages application's operating system native WiFi solution for connectivity
   - **MQTT**: Protocol used to transmit data over WiFi connectivity
   - **USB**: Serial communication for debugging and direct control
   - Real-time data streaming for environmental information

3. **Data Format**
   ```json
   {
     "readings": {
       "temperature": 25.4,
       "humidity": 37.7,
       "pressure": 992.9,
       "gas": 11.71,
       "altitude": 171.1,
       "timestamp": "2025-04-06T10:43:57"
     }
   }
   ```

## 🏠 Use Cases

<div style="display: grid; grid-template-columns: repeat(2, 1fr); gap: 20px;">
<div>

### Smart Home
- Indoor air quality monitoring
- Climate control optimization
- Energy efficiency improvements
- Weather trend analysis
</div>

<div>

### Office Environments
- Workplace comfort monitoring
- HVAC system optimization
- Meeting room environmental quality tracking
- Sick building syndrome prevention
</div>

<div>

### Agricultural Applications
- Greenhouse climate monitoring
- Optimal growing condition maintenance
- Early detection of environmental anomalies
- Crop yield optimization
</div>

<div>

### Industrial Applications
- Storage environment monitoring
- Process control environmental factors
- Equipment operation condition monitoring
- Regulatory compliance for environmental conditions
</div>
</div>

## 👥 Contributing

N/A

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.

## 🙏 Acknowledgments

- [Adafruit](https://www.adafruit.com/) for the BME680 sensor and libraries
- [Lilygo](https://www.lilygo.cc/) for the T-Display S3 AMOLED development board
- [Espressif](https://www.espressif.com/) for the ESP32-S3 microcontroller
- [Tauri](https://tauri.app/) for the desktop application framework
- [Svelte](https://svelte.dev/) for the frontend framework
- [Tailwind CSS](https://tailwindcss.com/) for UI styling
- [Squareline Studio](https://squareline.io/) for UI design tools

## 📚 References

- [ESP32-S3 Technical Documentation](https://www.espressif.com/en/products/socs/esp32-s3)
- [Adafruit BME680 Documentation](https://learn.adafruit.com/adafruit-bme680-humidity-temperature-barometic-pressure-voc-gas)
- [Lilygo T-Display S3 Documentation](https://github.com/Xinyuan-LilyGO/T-Display-S3)
- [PlatformIO Documentation](https://docs.platformio.org/)
- [Squareline Studio Documentation](https://docs.squareline.io/)
