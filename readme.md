# Windows-Bluetooth-Watcher

[中文](https://github.com/starwindv/windows_bluetooth_watcher/blob/main/readme_cn.md)

## Table of Contents

- [Windows-Bluetooth-Watcher](#Windows-Bluetooth-Watcher)
    - [Table of Contents](#table-of-contents)
    - [I. Introduction](#i-introduction)
    - [II. Usage](#ii-usage)
        - [2.1 Pre-built Package](#21-pre-built-package)
        - [2.2 Building from Source](#22-building-from-source)
            - [2.2.1 Prerequisites](#221-prerequisites)
            - [2.2.2 Clone](#222-clone)
            - [2.2.3 Build](#223-build)
            - [2.2.4 Installation](#224-installation)
    - [III. Documentation](#iii-documentation)
    - [IV. Breaking Changes](#iv-breaking-changes)
    - [V. License](#v-license)

---

## I. Introduction

This project is a secondary development based on the `windows crate`. It primarily wraps related methods for obtaining the connection status of Bluetooth devices on the Windows system and uses PyO3 for Python bindings to provide it as a Python library.

---

## II. Usage

### 2.1 Pre-built Package

You can use our pre-built version for `windows-amd64` devices with the following command:
```shell
pip install windows-bluetooth-watcher
```

### 2.2 Building from Source

#### 2.2.1 Prerequisites

- **System**: Windows 10 or later
- **Environment**: Rustup full toolchain, Python>=3.10, maturin>=1.9, git

#### 2.2.2 Clone

Execute the following commands to clone the project locally:
```shell
git clone https://github.com/starwindv/Windows-Bluetooth-Watcher.git
cd Windows-Bluetooth-Watcher
```

#### 2.2.3 Build

```shell
maturin build # or python -m build
```

Depending on the command used, the build artifacts are located in the following two locations:

**Using maturin**: `.\target\wheels\windows_bluetooth_watcher-{proj_version}-{py_version}-{py_version}-win_{architecture}.whl`

**Using python**:
`.\dist\windows_bluetooth_watcher-{proj_version}.tar.gz`
and
`.\dist\windows_bluetooth_watcher-{proj_version}-{py_version}-{py_version}-win_{architecture}.whl`

#### 2.2.4 Installation

```shell
python -m pip install {path_to_wheel}
```

---

## III. Documentation

You can generate the documentation using `cargo doc` in the project's root directory, or check our curated [detailed documentation](https://github.com/starwindv/windows-bluetooth-watcher/blob/main/doc/en)

---

## IV. Breaking Changes

You can find descriptions of possible breaking changes in each version in [BREAKING.md](https://github.com/starwindv/windows-bluetooth-watcher/blob/main/BREAKING.md).

---

## V. License

This project is licensed under the GPL-3.0 license. However, please note that developers listed in the [AUTHORS](https://github.com/starwindv/windows-bluetooth-watcher/blob/main/AUTHORS) file are granted an exception to the GPL-3.0 license. This exception allows them to use this project in the development of each `branch` project under the [corresponding project](https://github.com/Python-island/Python-island) without being subject to its copyleft requirements.

All other users must still comply with the full terms of the GPL-3.0.
