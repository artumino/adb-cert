# Usage

1. Install ADB
2. Mount `/system` as read-write if needed (e.g. `adb root && adb remount -R`)
3. Run `adbcert <path-to-pem> [--cert-path <cacerts_path>] [--device-serial <device_name>]`
4. Reboot the device
