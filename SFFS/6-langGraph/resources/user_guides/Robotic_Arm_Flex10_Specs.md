# Flex-10 Robotic Unit - Technical Manual
**Supplier:** TechRobotics Inc.
**Model:** Flex-10

## 📦 Parts List
* **Servo Motors:** Axis 1-6 (Model SM-Flex-V2)
* **Encoder Cables:** Shielded CAT6
* **Cooling Fan:** 12V DC Brushless

## 🔧 Maintenance Procedures

### Servo Motor Replacement
**Required Tools:** Torx T20, Anti-static wrist strap.
1. Cut power using the main breaker.
2. Remove the plastic casing on the joint.
3. Disconnect the encoder cable carefully.
4. Unscrew the 4 mounting screws.
5. Mount the new motor and reconnect cables.

### Calibration Routine
1. Connect the diagnostics laptop via USB-C.
2. Run `calibrate_home_position.exe`.
3. Manually move the arm to the physical "Zero" marks.

## ⚠️ Safety Protocols
* **E-Stop:** The red emergency button cuts power immediately.
* **Safety Cage:** Do not bypass the door interlock switch (Circuit 44).