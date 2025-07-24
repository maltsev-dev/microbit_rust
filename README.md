
## 🧪 nrf52833 BBC micro:bit
**Experiments, Prototypes & Notes with the micro:bit v2.21**

This repository contains my projects, experiments, and notes using the **micro:bit v2.21**, built on the **nrf52833** microcontroller.  
It serves as a playground for testing ideas, learning embedded concepts, and building small prototypes.


## **Goals:**

* Explore nrf52833 features with `microbit::hal`
* Interface with peripherals (GPIO, I2C, SPI, UART, ADC, PWM)
* Experiment with power management, timers, interrupts
* Integrate with sensors and modules (temperature, motion, light, etc.)

---

## ⚙️ Hardware Used

* micro:bit v2.21
* Breadboard & jumper wires
* Sensors: DHT11, MPU6050, BH1750, HW-416A, HC-SR04, HW-504, etc
* Power: USB / battery / external supply 

---

## 🧩 Projects & Experiments

| Project Name            | Description                         | Status         |
| ----------------------- | ----------------------------------- | -------------- |
| `led_circle`  | led matrix circle controlled by bsc           | ✅ Done         |
| `xxx`             | planned | ⏳ Planned       |

*More to come.*

---

## 🔧 Tools & Tech Stack

* Languages:  Rust
* IDEs: VSCode
* Libraries: `microbit::bsc`, `microbit::hal`, `panic_rtt_target`, `cortex_m_rt`
* Protocols: GPIO, MQTT, UDP, TCP
* Flash storage & OTA update experiments (planned)

---

## 📎 Notes

This is an open lab space – not a polished library or framework. Things may be messy, experimental, or half-finished.
If you're tinkering with the micro:bit too, feel free to fork, comment, or share ideas.
