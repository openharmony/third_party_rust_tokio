# ylong_runtime

A tokio-runtime wrapper crate, provides basic runtime functionalities:
1. async net io
2. async file io
3. sync
4. timer

## Overview
This crate wraps tokio's apis. The only difference from tokio is that the runtime instance is a global
singleton object. The first call to ``ylong_ruintime::spawn`` or ``ylong_runtime::spawn_blocking`` will
initialize the global runtime. Users could also configure the runtime using ``RuntimeBuilder`` before
spawning.

## Background
This crate's purpose is to uniform runtime's apis, so users could ignore the underlying scheduler.

## How to use
Please add the following dependency in your ``BUILD.gn``

```json
deps += ["//third_party/rust/crate/ylong_runtime"]
```
[tokio readme](../README.md)