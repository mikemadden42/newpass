# newpass
[![build](https://github.com/mikemadden42/newpass/actions/workflows/build.yml/badge.svg)](https://github.com/mikemadden42/newpass/actions/workflows/build.yml)

Generate random passwords.

## Usage

```
newpass [OPTIONS] [LENGTH]...
```

Prints one password per `LENGTH` (default 16, maximum 1024) using letters and digits.

| Option | Description |
| --- | --- |
| `-s`, `--symbols` | Include symbols as well as letters and digits |
| `-h`, `--help` | Print help |

```console
$ newpass
q8ZrT3mWbX0kLfPa
$ newpass -s 12 20
b#4K}x9@Qe;T
Wm^2r(Hz7P!c_Lq5]dVa
```

MSRV: 1.63
