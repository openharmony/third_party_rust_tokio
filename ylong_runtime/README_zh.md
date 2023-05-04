# ylong_runtime
该库是一个``tokio``的封装库，提供了以下运行时组件：
1. 异步网络io
2. 异步文件io
3. 同步原语
4. 定时器

## ylong_runtime简介
ylong_runtime库封装了tokio的基础接口，并在上层提供了一个全局单例的运行时实例。用户在调用
``ylong_runtime::spawn``或是``ylong_runtime::spawn_blocking``时，ylong_runtime
将会自动启动一个默认配置的运行时。用户也可以在启动运行时前，通过``RuntimeBuilder``进行配置。

## 如何使用

在您的 BUILD.gn 需要的地方添加依赖即可。

```json
deps += ["//third_party/rust/crate/ylong_runtime"]
```
[tokio的原生README](../README.md)