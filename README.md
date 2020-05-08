# i3-auto-layout
Automatic, optimal tiling for i3wm inspired by the deprecated `i3-alternating-layouts` and bspwm. An appropriate split is set for each window based on its geometry. 

Improvements over `i3-alternating-layouts`:
- single compiled binary with no dependencies (except i3 of course)
- written in Rust for maximum performance and low resource usage (~0% CPU, ~0% MEM)
- works asynchronously over IPC

### Before

![image](https://user-images.githubusercontent.com/11352152/67165362-f207aa80-f351-11e9-92e7-7294bfd678c0.png)

### After
![image](https://user-images.githubusercontent.com/11352152/67165367-f7fd8b80-f351-11e9-8f1c-3ef53528c5ca.png)

# Installation

Grab a binary from [releases](https://github.com/chmln/i3-auto-layout/releases) OR `cargo install --git https://github.com/chmln/i3-auto-layout`

Then somewhere in your i3 config

```
exec_always --no-startup-id i3-auto-layout
```
