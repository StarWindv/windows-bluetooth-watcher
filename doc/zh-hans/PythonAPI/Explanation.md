# Python 侧 API 文档

本文档用于介绍编译后二进制库的使用

**注意**: 本文档直接基于[兄弟项目](https://github.com/starwindv/windows-toast-notification-listener.git)的文档改写而来, 如有纰漏或错误请直接提出 issue.

---

## 文档版本

 - 0.1.0

注意, 文档版本号跟随项目版本一同更迭.

---

## 目录

- [Python 侧 API 文档](#python-侧-api-文档)
  - [文档版本](#文档版本)
  - [目录](#目录)
  - [一、介绍](#一介绍)
    - [1.1 导入](#11-导入)
    - [1.2 类](#12-类)
  - [二、Listener](#二listener)
    - [2.1 介绍](#21-介绍)
    - [2.2 接口](#22-接口)
      - [2.2.1 request\_permission](#221-request_permission)
        - [2.2.1.1 参数](#2211-参数)
        - [2.2.1.2 类型](#2212-类型)
        - [2.2.1.3 返回值](#2213-返回值)
        - [2.2.1.4 示例](#2214-示例)
      - [2.2.2 get\_all\_notifications](#222-get_all_notifications)
        - [2.2.2.1 参数](#2221-参数)
        - [2.2.2.2 类型](#2222-类型)
        - [2.2.2.3 返回值](#2223-返回值)
        - [2.2.2.4 示例](#2224-示例)
  - [三、Device](#三device)
    - [3.1 介绍](#31-介绍)
    - [3.1 属性](#31-属性)
  - [四、Diff](#四diff)
    - [4.1 介绍](#41-介绍)
    - [4.2 属性](#42-属性)
    - [4.3 示例](#43-示例)
  - [五、DiffTool](#五difftool)
    - [5.1 介绍](#51-介绍)
    - [5.2 接口](#52-接口)
      - [5.2.1 diff](#521-diff)
        - [5.2.1.1 说明](#5211-说明)
        - [5.2.1.2 参数](#5212-参数)
        - [5.2.1.3 返回值](#5213-返回值)
        - [5.2.1.4 示例](#5214-示例)
      - [5.2.2 to\_json\_str](#522-to_json_str)
        - [5.2.2.1 说明](#5221-说明)
        - [5.2.2.2 参数](#5222-参数)
        - [5.2.2.3 返回值](#5223-返回值)
        - [5.2.2.4 示例](#5224-示例)
      - [5.2.3 serialize\_to](#523-serialize_to)
        - [5.2.3.1 说明](#5231-说明)
        - [5.2.3.2 参数](#5232-参数)
        - [5.2.3.3 返回值](#5233-返回值)
        - [5.2.3.4 示例](#5234-示例)
  - [六、SerializeFormat](#六serializeformat)
    - [6.1 介绍](#61-介绍)
    - [6.2 枚举值](#62-枚举值)
  - [七、Polling](#七polling)
    - [7.1 介绍](#71-介绍)
    - [7.2 接口](#72-接口)
      - [7.2.1 `__init__`](#721-__init__)
        - [7.2.1.1 参数](#7211-参数)
        - [7.2.1.2 返回值](#7212-返回值)
        - [7.2.1.3 示例](#7213-示例)
      - [7.2.2 `register_polling_event_callback`](#722-register_polling_event_callback)
        - [7.2.2.1 说明](#7221-说明)
        - [7.2.2.2 参数](#7222-参数)
        - [7.2.2.3 返回值](#7223-返回值)
        - [7.2.2.4 示例](#7224-示例)
      - [7.2.3 `unregister`](#723-unregister)
        - [7.2.3.1 说明](#7231-说明)
        - [7.2.3.2 参数](#7232-参数)
        - [7.2.3.3 返回值](#7233-返回值)
      - [7.2.4 `on_type_callback`](#724-on_type_callback)
        - [7.2.4.1 说明](#7241-说明)
        - [7.2.4.2 参数](#7242-参数)
        - [7.2.4.3 返回值](#7243-返回值)
        - [7.2.4.4 示例](#7244-示例)
      - [7.2.5 `start_all`](#725-start_all)
        - [7.2.5.1 说明](#7251-说明)
        - [7.2.5.2 返回值](#7252-返回值)
      - [7.2.6 `stop_all`](#726-stop_all)
        - [7.2.6.1 说明](#7261-说明)
        - [7.2.6.2 返回值](#7262-返回值)
      - [7.2.7 `polling_for`](#727-polling_for)
        - [7.2.7.1 说明](#7271-说明)
        - [7.2.7.2 参数](#7272-参数)
        - [7.2.7.3 返回值](#7273-返回值)
      - [7.2.8 `stop_for`](#728-stop_for)
        - [7.2.8.1 说明](#7281-说明)
        - [7.2.8.2 参数](#7282-参数)
        - [7.2.8.3 返回值](#7283-返回值)
      - [7.2.9 `change_interval`](#729-change_interval)
        - [7.2.9.1 说明](#7291-说明)
        - [7.2.9.2 参数](#7292-参数)
        - [7.2.9.3 示例](#7293-示例)
  - [八、CallbackToken](#八callbacktoken)
    - [8.1 介绍](#81-介绍)
    - [8.2 属性](#82-属性)
  - [九、PollingStatus](#九pollingstatus)
    - [9.1 介绍](#91-介绍)
    - [9.2 枚举值](#92-枚举值)
  - [十、EventsType](#十eventstype)
    - [10.1 介绍](#101-介绍)
    - [10.2 枚举值](#102-枚举值)

---

## 一、介绍

### 1.1 导入

你可以使用以下代码来导入我们的库

```python
import windows_bluetooth_watcher as wbw
```

### 1.2 类

库提供如下几个类:
- [Listener](#二Listener)
- [Device](#三Device)
- [Diff](#四Diff)
- [DiffTool](#五DiffTool)
- [SerializeFormat](#六Serializeformat)

---

## 二、Listener

### 2.1 介绍

`Listener` 是本库的核心类, 用于创建设备监听器实例、申请权限以及获取当前系统所有 Device 设备.

### 2.2 接口

此类提供如下接口(可点击):

- [`request_permission`](#221-request_permission)
- [`get_all_notifications`](#222-get_all_notifications)

---

#### 2.2.1 request_permission

##### 2.2.1.1 参数

| 参数名    | 类型 | 说明   |
|--------|----|------|
| `self` | -  | 实例对象 |

##### 2.2.1.2 类型

- **异步方法**: 需要使用 `await` 来获取结果.

##### 2.2.1.3 返回值

返回一个 `str` 类型的权限状态, 可能的值如下: 

| 返回值             | 描述                                      |
|-----------------|-----------------------------------------|
| `"Unspecified"` | 用户尚未允许或拒绝访问                             |
| `"Allowed"`     | 用户已授予对 `UserNotificationListener` 的访问权限 |
| `"Denied"`      | 用户拒绝访问 `UserNotificationListener`       |
| `"Unknown"`     | 未知错误, 通常不会出现, 仅为配合 Rust 模式匹配而保留         |

##### 2.2.1.4 示例

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

##### 2.2.2.1 参数

| 参数名    | 类型 | 说明   |
|--------|----|------|
| `self` | -  | 实例对象 |

##### 2.2.2.2 类型

- **异步方法**: 需要使用 `await` 来获取结果.

##### 2.2.2.3 返回值

返回一个 `list[Device]` 类型的数组, 包含当前系统中所有 Device 设备.  
若未获得权限, 则返回空列表 `[]`.

##### 2.2.2.4 示例

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

## 三、Device

### 3.1 介绍

`Device` 是本库中用于表示单条蓝牙设备的核心数据结构.   
它包含了蓝牙设备的大部分元数据字段, 部分信息由于不同设备 API 的不同无法统一获取(如电量) .

该类由 Rust 实现并通过 PyO3 导出到 Python, 不支持实例化, 通常由 `Listener.get_all()` 返回.

---

### 3.1 属性

该类所有字段均为只读属性, 可通过点号访问:

| 属性名      | 类型    | 说明    |
|----------|-------|-------|
| `id`     | `str` | 设备 ID |
| `name`   | `str` | 设备名称  |
| `addr`   | `int` | 设备地址  |
| `status` | `str` | 设备状态  |

---

## 四、Diff

### 4.1 介绍

`Diff` 是一个用于表示设备状态差异的数据结构, 通常由 `DiffTool` 的差异计算方法返回. 它包含两个字段: 新进入连接状态的设备列表和新断连的设备列表. 

---

### 4.2 属性

| 属性名      | 类型             | 说明      |
|----------|----------------|---------|
| `new`    | `list[Device]` | 新增的设备列表 |
| `remove` | `list[Device]` | 移除的设备列表 |

---

### 4.3 示例

```python
import windows_bluetooth_watcher as wbw

diff = wbw.Diff(
    new=[device1, device2],
    remove=[device3]
)

for device in diff.new:
    print(device.title)
```

---

## 五、DiffTool

### 5.1 介绍

`DiffTool` 是一个工具类, 提供多种方式计算两个设备列表之间的差异, 并支持将设备列表序列化为多种格式. 

---

### 5.2 接口

此类提供如下接口(可点击):
 - [`diff`](#521-diff)

---

#### 5.2.1 diff

##### 5.2.1.1 说明

基于完整指纹 (包含时间戳) 对比两个设备列表的差异. 

##### 5.2.1.2 参数

| 参数名   | 类型             | 说明    |
|-------|----------------|-------|
| `old` | `list[Device]` | 旧设备列表 |
| `new` | `list[Device]` | 新设备列表 |

##### 5.2.1.3 返回值

返回一个 [`Diff`](#四Diff) 对象, 包含新进入连接状态和新进入断连状态的设备. 

##### 5.2.1.4 示例

```python
diff = wbw.DiffTool.diff(old_devices, new_devices)
print(len(diff.connected), len(diff.disconnected))
```

---
#### 5.2.2 to_json_str

##### 5.2.2.1 说明

将设备列表序列化为格式化的 JSON 字符串. 若序列化失败, 返回 `"[]"`. 

##### 5.2.2.2 参数

| 参数名       | 类型             | 说明        |
|-----------|----------------|-----------|
| `devices` | `list[Device]` | 待序列化的设备列表 |

##### 5.2.2.3 返回值

`str`: 格式化的 JSON 字符串. 

##### 5.2.2.4 示例

```python
json_str = wbw.DiffTool.to_json_str(divices)
print(json_str)
```

---

#### 5.2.3 serialize_to

##### 5.2.3.1 说明

将设备列表序列化为指定格式的字符串. 

##### 5.2.3.2 参数

| 参数名       | 类型                                     | 说明        |
|-----------|----------------------------------------|-----------|
| `devices` | `list[device]`                         | 待序列化的设备列表 |
| `to`      | [`SerializeFormat`](#六Serializeformat) | 目标序列化格式   |

##### 5.2.3.3 返回值

`str`: 格式化的字符串, 失败时返回 `"[]"`. 

##### 5.2.3.4 示例

```python
yaml_str = wbw.DiffTool.serialize_to(devices, wbw.SerializeFormat.Yaml)
print(yaml_str)
```

---


## 六、SerializeFormat

### 6.1 介绍

`SerializeFormat` 是一个枚举类型, 用于指定序列化格式. 该枚举在 Python 中为**不可变类型**, 不可修改. 

---

### 6.2 枚举值

| 枚举值    | 说明      |
|--------|---------|
| `Json` | JSON 格式 |
| `Yaml` | YAML 格式 |

---

## 七、Polling

### 7.1 介绍

`Polling` 是一个基于轮询机制的事件循环管理器, 用于持续监听设备列表连接状态的变化, 并在检测到变化时触发注册的回调函数. 它支持按事件类型 (新增、移除或全部) 注册回调, 并支持动态调整轮询间隔.

---

### 7.2 接口

此类提供如下接口: 

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

##### 7.2.1.1 参数

| 参数名        | 类型         | 说明              |
|------------|------------|-----------------|
| `listener` | `Listener` | 用于获取设备的监听器实例    |
| `interval` | `int`      | 轮询间隔时间 (单位: 毫秒) |

##### 7.2.1.2 返回值

返回一个新的 `Polling` 实例. 

##### 7.2.1.3 示例

```python
import windows_bluetooth_watcher as wbw

listener = wbw.Listener()
polling = wbw.features.Polling(listener, interval=1000)
```

---

#### 7.2.2 `register_polling_event_callback`

##### 7.2.2.1 说明

注册一个全局回调函数, 该回调会接收所有类型的事件 (新增 + 移除) . 

##### 7.2.2.2 参数

| 参数名       | 类型         | 说明             |
|-----------|------------|----------------|
| `handler` | `Callable` | 接收一个 `Diff` 参数 |

##### 7.2.2.3 返回值

返回一个 [`CallbackToken`](#八CallbackToken) 令牌, 用于后续注销. 

##### 7.2.2.4 示例

```python
def on_event(diff):
    print(f"新增: {len(diff.new)}, 移除: {len(diff.remove)}")

token = polling.register_polling_event_callback(on_event)
```

---

#### 7.2.3 `unregister`

##### 7.2.3.1 说明

注销指定令牌对应的回调函数. 

##### 7.2.3.2 参数

| 参数名     | 类型              | 说明     |
|---------|-----------------|--------|
| `token` | `CallbackToken` | 回调令牌对象 |

##### 7.2.3.3 返回值

返回 [`PollingStatus`](#九PollingStatus) 枚举值: `Success` 表示成功, `Failed` 表示失败. 

---

#### 7.2.4 `on_type_callback`

##### 7.2.4.1 说明

注册一个仅针对特定事件类型的回调函数. 

##### 7.2.4.2 参数

| 参数名        | 类型           | 说明             |
|------------|--------------|----------------|
| `handler`  | `Callable`   | 接收一个 `Diff` 参数 |
| `for_type` | `EventsType` | 指定回调响应的事件类型    |

##### 7.2.4.3 返回值

返回一个 [`CallbackToken`](#八CallbackToken) 令牌. 

##### 7.2.4.4 示例

```python
def on_new(diff):
    for device in diff.new:
        print(device.title)

token = polling.on_type_callback(on_new, wbw.EventsType.New)
```

---

#### 7.2.5 `start_all`

##### 7.2.5.1 说明

启动事件循环, 开始轮询设备变化并触发回调. 如果轮询已在运行, 则立即返回成功. 

##### 7.2.5.2 返回值

返回 [`PollingStatus.Success`](#九PollingStatus). 

---

#### 7.2.6 `stop_all`

##### 7.2.6.1 说明

停止所有轮询任务. 

##### 7.2.6.2 返回值

返回 [`PollingStatus.Success`](#九PollingStatus). 

---

#### 7.2.7 `polling_for`

##### 7.2.7.1 说明

激活指定令牌的回调函数, 使其开始处理事件. 

##### 7.2.7.2 参数

| 参数名     | 类型              | 说明     |
|---------|-----------------|--------|
| `token` | `CallbackToken` | 回调令牌对象 |

##### 7.2.7.3 返回值

返回 [`PollingStatus`](#九PollingStatus). 

---

#### 7.2.8 `stop_for`

##### 7.2.8.1 说明

暂停指定令牌的回调函数, 使其不再处理事件. 

##### 7.2.8.2 参数

| 参数名     | 类型              | 说明     |
|---------|-----------------|--------|
| `token` | `CallbackToken` | 回调令牌对象 |

##### 7.2.8.3 返回值

返回 [`PollingStatus`](#九PollingStatus). 

---

#### 7.2.9 `change_interval`

##### 7.2.9.1 说明

动态修改轮询间隔时间. 

##### 7.2.9.2 参数

| 参数名        | 类型    | 说明                |
|------------|-------|-------------------|
| `interval` | `int` | 新的轮询间隔时间 (单位: 毫秒) |

##### 7.2.9.3 示例

```python
polling.change_interval(2000)
```

---

## 八、CallbackToken

### 8.1 介绍

`CallbackToken` 是每个回调函数的唯一标识符, 由系统在注册时自动生成, 用于后续注销或控制回调的启用/禁用状态. 

### 8.2 属性

| 属性名  | 类型    | 说明    |
|------|-------|-------|
| `id` | `int` | 唯一标识符 |

---

## 九、PollingStatus

### 9.1 介绍

`PollingStatus` 是一个枚举类型, 用于表示操作的结果状态. 

### 9.2 枚举值

| 枚举值       | 说明     |
|-----------|--------|
| `Success` | 操作成功完成 |
| `Failed`  | 操作失败   |

---

## 十、EventsType

### 10.1 介绍

`EventsType` 是一个枚举类型, 用于指定回调函数响应的事件类型. 

### 10.2 枚举值

| 枚举值      | 说明               |
|----------|------------------|
| `New`    | 仅响应新连接设备事件       |
| `Remove` | 仅响应断连设备事件        |
| `All`    | 响应所有类型事件 (连接+断连) |

---

末次编辑日期: 2026年3月18日
