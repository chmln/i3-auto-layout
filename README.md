# i3-auto-layout
Automatic, optimal tiling for i3wm. An appropriate split is set for each window based on its geometry. Communication with i3 happens asynchronously over IPC.

## Before

![image](https://user-images.githubusercontent.com/11352152/67165362-f207aa80-f351-11e9-92e7-7294bfd678c0.png)

## After
![image](https://user-images.githubusercontent.com/11352152/67165367-f7fd8b80-f351-11e9-8f1c-3ef53528c5ca.png)

# Installation

Grab a binary from [releases](https://github.com/chmln/i3-auto-layout/releases) OR `cargo install --git https://github.com/chmln/i3-auto-layout`

Then somewhere in your i3 config

```
exec_always --no-startup-id i3-auto-layout
```
