# bootc update daemon - DBus version

**Notice: This is completely experimental and incomplete.**

## Goals

- Rust DBus daemon/client with polkit access control
- Unprivileged operations for any active and interactive user:
    - Status
    - Check for update
- Unprivileged operations for any local, active and interactive user:
    - Update
- Privileged operations
    - None planed for now, use bootc directly instead

See:
- https://github.com/containers/bootc/issues/2
- https://github.com/containers/bootc/issues/4
- https://github.com/containers/bootc/issues/522

## How to

```
$ sudo install -m644 -o 0 -g 0 daemon/bootc-daemon.conf -t /etc/dbus-1/system.d/
$ sudo restorecon -RFv /etc/dbus-1/system.d/bootc-daemon.conf
$ sudo systemctl reload dbus-broker.service
```

```
$ cargo build --bin bootc-daemon
$ sudo ./target/debug/bootc-daemon
...
```

```
$ cargo build --bin bootc-client
$ cargo run --bin bootc-client
...
```
