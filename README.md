# Archlauncher

Archlauncher is a simple program that launches a program in the folder of the architecture it is running on.
This is useful on windows steam since steam doesn't differentiate arm and x86_64 architectures, so this is needed.

## Usage

```
archlauncher <program> [args]
```

## Example

```
archlauncher hello.exe
```

This will launch the program `hello.exe` in the folder of the architecture it is running on.

## Supported architectures

- x86_64
- aarch64