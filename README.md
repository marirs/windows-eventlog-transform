Transform Windows Event Log XML
--------------------------------
[![Build Status](https://travis-ci.com/marirs/windows-eventlog-transform.svg?branch=main)](https://travis-ci.com/marirs/windows-eventlog-transform)

Transform Windows Event Log XML files or strings into JSON and or Common Event Format (CEF)

## Requirements
- Rust

### Compile
- Build Release
```bash
cargo b --release
```
- Test
```bash
cargo t
```

## Run Examples
```bash
cargo run --example eg1
```

## Example Transformed Json
```json
{
  "xmlns": "http://schemas.microsoft.com/win/2004/08/events/event",
  "System": {
    "Provider": {
      "Name": "BTHUSB",
      "Guid": null
    },
    "EventRecordID": 5662,
    "Event": {
      "EventID": 18,
      "EventName": "EventID-18"
    },
    "Level": "Information",
    "Task": "None",
    "Opcode": "Info",
    "Keywords": "Classic",
    "TimeCreated": "2021-01-29T12:37:19.5374683Z",
    "Correlation": {
      "ActivityID": null
    },
    "Execution": {
      "ProcessID": 4,
      "ThreadID": 1240
    },
    "Channel": "System",
    "Computer": "DESKTOP-G089JUF",
    "Security": {
      "UserID": null
    },
    "Version": 0
  },
  "EventData": {
    "Data": [
      null
    ],
    "Binary": "00000800010000000000000012000540000000000000000000000000000000000000000000000000E000000000000000"
  }
}

```
---
