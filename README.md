# Unofficial builds of Zed for Windows

**NOTE: This is not a support channel for Zed on Windows.**

These builds are for those who want to live on the bleeding edge or just want to test Zed out on Windows.

Any issues with the Windows build should go through official channels, as this repository does not concern itself with the source code of Zed or issues found therein.

## Vulkan doesn't work for you?

Install the OpenGL version

### For Windows 10 users

Zed may not start unless you install the [Microsoft Visual C++ Redistributable 2022](https://learn.microsoft.com/en-us/cpp/windows/latest-supported-vc-redist?view=msvc-170#visual-studio-2015-2017-2019-and-2022) package. If you are using Scoop, you can install it using the following command:

```pwsh
scoop bucket add extras
scoop install vcredist2022
```

## Warning

This is a personal repo built for my CPU with `-C target-cpu=alderlake`. You should not use this repo or its builds. Use the parent repo as it is generalized for any CPU.

But if you're interested in getting nightly builds customized to your specific CPU, then fork this repo and run this cmd in your terminal:

`rustc --target=x86_64-pc-windows-msvc --print target-cpus`

At the top you'll see something like
```
Available CPUs for this target:
    native                  - Select the CPU of the current host (currently alderlake).
```
In my case it was `alderlake`. In [config.toml](./.cargo/config.toml), change the `alderlake` part in the `target-cpu=alderlake` to your specific CPU.

## App

This repo also contains a Rust app that downloads the latest release from your repo and extracts the file to the same directory. That way you don't have to bother downloading from the repo and extracting the archive. If you wish to use it with your own repo, change the owner and repo name in the file so it points to your repo. Technically you could also use this app with the parent repo which was generalized for any CPU.

![image](https://github.com/user-attachments/assets/ead0848d-c1c3-4a9c-8bff-090fb45927b3)

## Is it safe?

This repository is just a [simple GitHub workflow](./.github/workflows/build.yml) that builds Zed from `main` and publishes a release every night at UTC+0000. (Additionally on push for testing).

See the [Zed homepage](https://zed.dev/) or [official repository](https://github.com/zed-industries/zed) for more details.
