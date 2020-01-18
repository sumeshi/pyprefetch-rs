# pyprefetch-rs

[![MIT License](http://img.shields.io/badge/license-MIT-blue.svg?style=flat)](LICENSE)

Python bindings for `https://docs.rs/libprefetch/0.1.1/libprefetch/`

A Python library for fast parsing of Windows prefetch(.pf) files powered by Rust.

# Usage
The prefetch used as an example is from `https://github.com/PoorBillionaire/Windows-Prefetch-Parser`.

```python
from prefetch import Prefetch

p = Prefetch('/path/to/your/prefetch.pf')

print(p.name, p.exec_count, last_exec_time)

> test.pf 2 130974496211967500

for metrics in p.get_metrics():
    print(metrics)

> {'id': 0, 'filename': '\\DEVICE\\HARDDISKVOLUME2\\WINDOWS\\SYSTEM32\\NTDLL.DLL', 'start_time': 0, 'duration': 144}
> ...

for volume in p.get_volumes():
    print(volume)

> {'id': 0, 'path': '\\DEVICE\\HARDDISKVOLUME2', 'creation_time': 130974525181093750, 'serial_number': 2281737263, 'directories': ['\\DEVICE\\HARDDISKVOLUME2\\WINDOWS', '\\DEVICE\\HARDDISKVOLUME2\\WINDOWS\\FONTS', '\\DEVICE\\HARDDISKVOLUME2\\WINDOWS\\GLOBALIZATION', '\\DEVICE\\HARDDISKVOLUME2\\WINDOWS\\GLOBALIZATION\\SORTING', '\\DEVICE\\HARDDISKVOLUME2\\WINDOWS\\SYSTEM32', '\\DEVICE\\HARDDISKVOLUME2\\WINDOWS\\SYSTEM32\\EN-US', '\\DEVICE\\HARDDISKVOLUME2\\WINDOWS\\WINSXS\\AMD64_MICROSOFT.WINDOWS.COMMON-CONTROLS_6595B64144CCF1DF_6.0.7601.17514_NONE_FA396087175AC9AC', '\\DEVICE\\HARDDISKVOLUME2\\WINDOWS\\WINSXS\\AMD64_MICROSOFT.WINDOWS.GDIPLUS_6595B64144CCF1DF_1.1.7601.17514_NONE_2B24536C71ED437A']}
> ...

```

# Installation

via pip

```
$ pip install git+https://github.com/sumeshi/pyprefetch-rs
```

The source code for pyprefetch-rs is hosted at GitHub, and you may download, fork, and review it from this repository(https://github.com/sumeshi/pyprefetch-rs).

Please report issues and feature requests. :sushi: :sushi: :sushi:

# License
pyprefetch-rs is released under the MIT License.

Powered by [libprefetch](https://docs.rs/libprefetch/0.1.1/libprefetch/).  
Inspired by [pyevtx-rs](https://github.com/omerbenamram/pyevtx-rs), [pymft-rs](https://github.com/omerbenamram/pymft-rs).  