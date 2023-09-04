# Usage

1. Install ADB
2. Run emulator with `-writable-system` flag
3. Mount `/system` as read-write if needed (e.g. `adb root && adb remount -R`)
4. Run `adbcert <path-to-pem> [--cert-path <cacerts_path>] [--device-serial <device_name>]`
5. Reboot the device
