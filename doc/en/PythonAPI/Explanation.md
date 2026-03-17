# Python Side API Documentation

This document describes how to use the compiled binary library.

**Note**: This document is directly adapted from the documentation of a [sibling project](https://github.com/starwindv/windows-toast-notification-listener.git). If there are any omissions or errors, please submit an issue directly.

---

## Document Version

- 0.1.0

Note: The document version number follows the project version updates.

---

## Table of Contents

- [Python Side API Documentation](#python-side-api-documentation)
    - [Document Version](#document-version)
    - [Table of Contents](#table-of-contents)
    - [I. Introduction](#i-introduction)
        - [1.1 Import](#11-import)
        - [1.2 Classes](#12-classes)
    - [II. Listener](#ii-listener)
        - [2.1 Introduction](#21-introduction)
        - [2.2 Interfaces](#22-interfaces)
            - [2.2.1 request\_permission](#221-request_permission)
                - [2.2.1.1 Parameters](#2211-parameters)
                - [2.2.1.2 Type](#2212-type)
                - [2.2.1.3 Return Value](#2213-return-value)
                - [2.2.1.4 Example](#2214-example)
            - [2.2.2 get\_all\_notifications](#222-get_all_notifications)
                - [2.2.2.1 Parameters](#2221-parameters)
                - [2.2.2.2 Type](#2222-type)
                - [2.2.2.3 Return Value](#2223-return-value)
                - [2.2.2.4 Example](#2224-example)
    - [III. Device](#iii-device)
        - [3.1 Introduction](#31-introduction)
        - [3.2 Properties](#32-properties)
    - [IV. Diff](#iv-diff)
        - [4.1 Introduction](#41-introduction)
        - [4.2 Properties](#42-properties)
        - [4.3 Example](#43-example)
    - [V. DiffTool](#v-difftool)
        - [5.1 Introduction](#51-introduction)
        - [5.2 Interfaces](#52-interfaces)
            - [5.2.1 diff](#521-diff)
                - [5.2.1.1 Description](#5211-description)
                - [5.2.1.2 Parameters](#5212-parameters)
                - [5.2.1.3 Return Value](#5213-return-value)
                - [5.2.1.4 Example](#5214-example)
            - [5.2.2 to\_json\_str](#522-to_json_str)
                - [5.2.2.1 Description](#5221-description)
                - [5.2.2.2 Parameters](#5222-parameters)
                - [5.2.2.3 Return Value](#5223-return-value)
                - [5.2.2.4 Example](#5224-example)
            - [5.2.3 serialize\_to](#523-serialize_to)
                - [5.2.3.1 Description](#5231-description)
                - [5.2.3.2 Parameters](#5232-parameters)
                - [5.2.3.3 Return Value](#5233-return-value)
                - [5.2.3.4 Example](#5234-example)
    - [VI. SerializeFormat](#vi-serializeformat)
        - [6.1 Introduction](#61-introduction)
        - [6.2 Enum Values](#62-enum-values)
    - [VII. Polling](#vii-polling)
        - [7.1 Introduction](#71-introduction)
        - [7.2 Interfaces](#72-interfaces)
            - [7.2.1 `__init__`](#721-__init__)
                - [7.2.1.1 Parameters](#7211-parameters)
                - [7.2.1.2 Return Value](#7212-return-value)
                - [7.2.1.3 Example](#7213-example)
            - [7.2.2 `register_polling_event_callback`](#722-register_polling_event_callback)
                - [7.2.2.1 Description](#7221-description)
                - [7.2.2.2 Parameters](#7222-parameters)
                - [7.2.2.3 Return Value](#7223-return-value)
                - [7.2.2.4 Example](#7224-example)
            - [7.2.3 `unregister`](#723-unregister)
                - [7.2.3.1 Description](#7231-description)
                - [7.2.3.2 Parameters](#7232-parameters)
                - [7.2.3.3 Return Value](#7233-return-value)
            - [7.2.4 `on_type_callback`](#724-on_type_callback)
                - [7.2.4.1 Description](#7241-description)
                - [7.2.4.2 Parameters](#7242-parameters)
                - [7.2.4.3 Return Value](#7243-return-value)
                - [7.2.4.4 Example](#7244-example)
            - [7.2.5 `start_all`](#725-start_all)
                - [7.2.5.1 Description](#7251-description)
                - [7.2.5.2 Return Value](#7252-return-value)
            - [7.2.6 `stop_all`](#726-stop_all)
                - [7.2.6.1 Description](#7261-description)
                - [7.2.6.2 Return Value](#7262-return-value)
            - [7.2.7 `polling_for`](#727-polling_for)
                - [7.2.7.1 Description](#7271-description)
                - [7.2.7.2 Parameters](#7272-parameters)
                - [7.2.7.3 Return Value](#7273-return-value)
            - [7.2.8 `stop_for`](#728-stop_for)
                - [7.2.8.1 Description](#7281-description)
                - [7.2.8.2 Parameters](#7282-parameters)
                - [7.2.8.3 Return Value](#7283-return-value)
            - [7.2.9 `change_interval`](#729-change_interval)
                - [7.2.9.1 Description](#7291-description)
                - [7.2.9.2 Parameters](#7292-parameters)
                - [7.2.9.3 Example](#7293-example)
    - [VIII. CallbackToken](#viii-callbacktoken)
        - [8.1 Introduction](#81-introduction)
        - [8.2 Properties](#82-properties)
    - [IX. PollingStatus](#ix-pollingstatus)
        - [9.1 Introduction](#91-introduction)
        - [9.2 Enum Values](#92-enum-values)
    - [X. EventsType](#x-eventstype)
        - [10.1 Introduction](#101-introduction)
        - [10.2 Enum Values](#102-enum-values)

---

## I. Introduction

### 1.1 Import

You can import our library using the following code:

```python
import windows_bluetooth_watcher as wbw
```

### 1.2 Classes

The library provides the following classes:
- [Listener](#ii-listener)
- [Device](#iii-device)
- [Diff](#iv-diff)
- [DiffTool](#v-difftool)
- [SerializeFormat](#vi-serializeformat)

---

## II. Listener

### 2.1 Introduction

`Listener` is the core class of this library, used to create device listener instances, request permissions, and obtain all current Device devices in the system.

### 2.2 Interfaces

This class provides the following interfaces:

- [`request_permission`](#221-request_permission)
- [`get_all_notifications`](#222-get_all_notifications)

---

#### 2.2.1 request_permission

##### 2.2.1.1 Parameters

| Parameter | Type | Description     |
|-----------|------|-----------------|
| `self`    | -    | Instance object |

##### 2.2.1.2 Type

- **Asynchronous method**: Requires `await` to get the result.

##### 2.2.1.3 Return Value

Returns a permission status of type `str`. Possible values are:

| Return Value    | Description                                                              |
|-----------------|--------------------------------------------------------------------------|
| `"Unspecified"` | User has neither allowed nor denied access                               |
| `"Allowed"`     | User has granted access to `UserNotificationListener`                    |
| `"Denied"`      | User has denied access to `UserNotificationListener`                     |
| `"Unknown"`     | Unknown error, usually doesn't occur, retained for Rust pattern matching |

##### 2.2.1.4 Example

```python
import windows_bluetooth_watcher as wbw
import asyncio

async def main():
    listener = wbw.Listener()
    permission = await listener.request_permission()
    print(permission)

asyncio.run(main())
```

---

#### 2.2.2 get_all_notifications

##### 2.2.2.1 Parameters

| Parameter | Type | Description     |
|-----------|------|-----------------|
| `self`    | -    | Instance object |

##### 2.2.2.2 Type

- **Asynchronous method**: Requires `await` to get the result.

##### 2.2.2.3 Return Value

Returns an array of type `list[Device]`, containing all Device devices currently in the system.  
Returns an empty list `[]` if permission is not granted.

##### 2.2.2.4 Example

```python
import windows_bluetooth_watcher as wbw
import asyncio

async def main():
    listener = wbw.Listener()
    status = await listener.request_permission()
    match status:
        case x if x != "Allowed": return
    devices = await listener.get_all_notifications()
    for device in devices:
        print(wbw.DiffTool.serialize_to(device, wbw.SerializeFormat.Json))

asyncio.run(main())
```

---

## III. Device

### 3.1 Introduction

`Device` is the core data structure in this library representing a single Bluetooth device.  
It contains most metadata fields of Bluetooth devices; some information (such as battery level) cannot be uniformly obtained due to different device APIs.

This class is implemented in Rust and exported to Python via PyO3. It does not support instantiation and is typically returned by `Listener.get_all_notifications()`.

---

### 3.2 Properties

All fields of this class are read-only properties, accessible via dot notation:

| Property  | Type   | Description      |
|-----------|--------|------------------|
| `id`      | `str`  | Device ID        |
| `name`    | `str`  | Device name      |
| `addr`    | `int`  | Device address   |
| `status`  | `str`  | Device status    |

---

## IV. Diff

### 4.1 Introduction

`Diff` is a data structure representing device status differences, typically returned by the `DiffTool` diff calculation method. It contains two fields: a list of newly connected devices and a list of newly disconnected devices.

---

### 4.2 Properties

| Property | Type           | Description                 |
|----------|----------------|-----------------------------|
| `new`    | `list[Device]` | List of newly added devices |
| `remove` | `list[Device]` | List of removed devices     |

---

### 4.3 Example

```python
import windows_bluetooth_watcher as wbw

diff = wbw.Diff(
    new=[device1, device2],
    remove=[device3]
)

for device in diff.new:
    print(device.name)
```

---

## V. DiffTool

### 5.1 Introduction

`DiffTool` is a utility class that provides multiple ways to calculate differences between two device lists and supports serializing device lists into various formats.

---

### 5.2 Interfaces

This class provides the following interfaces:

- [`diff`](#521-diff)

---

#### 5.2.1 diff

##### 5.2.1.1 Description

Compares differences between two device lists based on complete fingerprints.

##### 5.2.1.2 Parameters

| Parameter | Type             | Description           |
|-----------|------------------|-----------------------|
| `old`     | `list[Device]`   | Old device list       |
| `new`     | `list[Device]`   | New device list       |

##### 5.2.1.3 Return Value

Returns a [`Diff`](#iv-diff) object containing newly connected and newly disconnected devices.

##### 5.2.1.4 Example

```python
diff = wbw.DiffTool.diff(old_devices, new_devices)
print(len(diff.new), len(diff.remove))
```

---
#### 5.2.2 to_json_str

##### 5.2.2.1 Description

Serializes a device list to a formatted JSON string. Returns `"[]"` if serialization fails.

##### 5.2.2.2 Parameters

| Parameter  | Type             | Description                |
|------------|------------------|----------------------------|
| `devices`  | `list[Device]`   | Device list to serialize   |

##### 5.2.2.3 Return Value

`str`: Formatted JSON string.

##### 5.2.2.4 Example

```python
json_str = wbw.DiffTool.to_json_str(devices)
print(json_str)
```

---

#### 5.2.3 serialize_to

##### 5.2.3.1 Description

Serializes a device list to a string in the specified format.

##### 5.2.3.2 Parameters

| Parameter | Type                                     | Description                 |
|-----------|------------------------------------------|-----------------------------|
| `devices` | `list[Device]`                           | Device list to serialize    |
| `to`      | [`SerializeFormat`](#vi-serializeformat) | Target serialization format |

##### 5.2.3.3 Return Value

`str`: Formatted string, returns `"[]"` on failure.

##### 5.2.3.4 Example

```python
yaml_str = wbw.DiffTool.serialize_to(devices, wbw.SerializeFormat.Yaml)
print(yaml_str)
```

---

## VI. SerializeFormat

### 6.1 Introduction

`SerializeFormat` is an enumeration type used to specify serialization formats. This enumeration is **immutable** in Python and cannot be modified.

---

### 6.2 Enum Values

| Enum Value | Description       |
|------------|-------------------|
| `Json`     | JSON format       |
| `Yaml`     | YAML format       |

---

## VII. Polling

### 7.1 Introduction

`Polling` is an event loop manager based on a polling mechanism, used to continuously monitor changes in device connection status and trigger registered callbacks when changes are detected. It supports registering callbacks by event type (new, remove, or all) and dynamically adjusting the polling interval.

---

### 7.2 Interfaces

This class provides the following interfaces:

- [`__init__`](#721-__init__)
- [`register_polling_event_callback`](#722-register_polling_event_callback)
- [`unregister`](#723-unregister)
- [`on_type_callback`](#724-on_type_callback)
- [`start_all`](#725-start_all)
- [`stop_all`](#726-stop_all)
- [`polling_for`](#727-polling_for)
- [`stop_for`](#728-stop_for)
- [`change_interval`](#729-change_interval)

---

#### 7.2.1 `__init__`

##### 7.2.1.1 Parameters

| Parameter  | Type       | Description                           |
|------------|------------|---------------------------------------|
| `listener` | `Listener` | Listener instance for getting devices |
| `interval` | `int`      | Polling interval (in milliseconds)    |

##### 7.2.1.2 Return Value

Returns a new `Polling` instance.

##### 7.2.1.3 Example

```python
import windows_bluetooth_watcher as wbw

listener = wbw.Listener()
polling = wbw.features.Polling(listener, interval=1000)
```

---

#### 7.2.2 `register_polling_event_callback`

##### 7.2.2.1 Description

Registers a global callback function that receives all types of events (new + remove).

##### 7.2.2.2 Parameters

| Parameter   | Type         | Description                     |
|-------------|--------------|---------------------------------|
| `handler`   | `Callable`   | Receives a `Diff` parameter     |

##### 7.2.2.3 Return Value

Returns a [`CallbackToken`](#viii-callbacktoken) token for subsequent unregistration.

##### 7.2.2.4 Example

```python
def on_event(diff):
    print(f"New: {len(diff.new)}, Removed: {len(diff.remove)}")

token = polling.register_polling_event_callback(on_event)
```

---

#### 7.2.3 `unregister`

##### 7.2.3.1 Description

Unregisters the callback function corresponding to the specified token.

##### 7.2.3.2 Parameters

| Parameter | Type            | Description           |
|-----------|-----------------|-----------------------|
| `token`   | `CallbackToken` | Callback token object |

##### 7.2.3.3 Return Value

Returns [`PollingStatus`](#ix-pollingstatus) enum value: `Success` on success, `Failed` on failure.

---

#### 7.2.4 `on_type_callback`

##### 7.2.4.1 Description

Registers a callback function for a specific event type only.

##### 7.2.4.2 Parameters

| Parameter  | Type         | Description                            |
|------------|--------------|----------------------------------------|
| `handler`  | `Callable`   | Receives a `Diff` parameter            |
| `for_type` | `EventsType` | Specifies the event type to respond to |

##### 7.2.4.3 Return Value

Returns a [`CallbackToken`](#viii-callbacktoken) token.

##### 7.2.4.4 Example

```python
def on_new(diff):
    for device in diff.new:
        print(device.name)

token = polling.on_type_callback(on_new, wbw.EventsType.New)
```

---

#### 7.2.5 `start_all`

##### 7.2.5.1 Description

Starts the event loop, begins polling for device changes, and triggers callbacks. Returns success immediately if polling is already running.

##### 7.2.5.2 Return Value

Returns [`PollingStatus.Success`](#ix-pollingstatus).

---

#### 7.2.6 `stop_all`

##### 7.2.6.1 Description

Stops all polling tasks.

##### 7.2.6.2 Return Value

Returns [`PollingStatus.Success`](#ix-pollingstatus).

---

#### 7.2.7 `polling_for`

##### 7.2.7.1 Description

Activates the callback function for the specified token, enabling it to process events.

##### 7.2.7.2 Parameters

| Parameter | Type            | Description           |
|-----------|-----------------|-----------------------|
| `token`   | `CallbackToken` | Callback token object |

##### 7.2.7.3 Return Value

Returns [`PollingStatus`](#ix-pollingstatus).

---

#### 7.2.8 `stop_for`

##### 7.2.8.1 Description

Pauses the callback function for the specified token, preventing it from processing events.

##### 7.2.8.2 Parameters

| Parameter | Type            | Description           |
|-----------|-----------------|-----------------------|
| `token`   | `CallbackToken` | Callback token object |

##### 7.2.8.3 Return Value

Returns [`PollingStatus`](#ix-pollingstatus).

---

#### 7.2.9 `change_interval`

##### 7.2.9.1 Description

Dynamically changes the polling interval.

##### 7.2.9.2 Parameters

| Parameter  | Type  | Description                            |
|------------|-------|----------------------------------------|
| `interval` | `int` | New polling interval (in milliseconds) |

##### 7.2.9.3 Example

```python
polling.change_interval(2000)
```

---

## VIII. CallbackToken

### 8.1 Introduction

`CallbackToken` is the unique identifier for each callback function, automatically generated by the system during registration. It is used for subsequent unregistration or controlling the callback's enabled/disabled status.

### 8.2 Properties

| Property   | Type    | Description         |
|------------|---------|---------------------|
| `id`       | `int`   | Unique identifier   |

---

## IX. PollingStatus

### 9.1 Introduction

`PollingStatus` is an enumeration type used to represent the result status of operations.

### 9.2 Enum Values

| Enum Value | Description                      |
|------------|----------------------------------|
| `Success`  | Operation completed successfully |
| `Failed`   | Operation failed                 |

---

## X. EventsType

### 10.1 Introduction

`EventsType` is an enumeration type used to specify the event type a callback function responds to.

### 10.2 Enum Values

| Enum Value | Description                                |
|------------|--------------------------------------------|
| `New`      | Responds only to new device events         |
| `Remove`   | Responds only to removed device events     |
| `All`      | Responds to all event types (new + remove) |

---

Last edited: March 18, 2026
