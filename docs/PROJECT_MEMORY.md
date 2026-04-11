# 大仙会议 / DaxianMeeting — 项目核心记忆

> 本文件是给后续工程会话快速恢复上下文用的“当前事实表”。
> 最后复核：2026-04-11。判断优先级：源码 > `docs/PROJECT_MEMORY.md` > `docs/ANDROID_STATE_MACHINE.md` / `docs/CHANGELOG.md` / `docs/KNOWN_BUGS.md`。

---

## 0. 协作铁律

- Codex 只能检索、分析和修改代码或文档。
- Codex 不允许执行 `git commit`，也不替用户提交任何 Git 变更。
- 提交只能由用户审查后亲自完成。
- 修改前先看当前工作树，不能覆盖用户已有改动。

---

## 1. 项目身份

| 项 | 当前事实 |
|---|---|
| 项目名 | 大仙会议 / DaxianMeeting |
| 上游基础 | RustDesk 1.4.x 二次开发 |
| Cargo package | `rustdesk` |
| Cargo version | `5.2.0` |
| Rust lib name | `librustdesk` |
| Flutter package | `flutter_hbb` |
| Flutter version | `5.2.0+58` |
| Android 包名 | `com.daxian.dev` |
| Android 显示名 | `大仙会议` |
| 运行时 APP_NAME | `libs/hbb_common/src/config.rs` 中为 `DaxianMeeting` |
| 运行时 ORG | 仍保留 `com.carriez` |
| Android SO | 构建产物 `liblibrustdesk.so`，复制后重命名为 `libdaxian.so` |
| Windows DLL | 按项目决策保留 `librustdesk.dll`，不是风险项 |
| Android Manifest scheme | 主 `daxian`，兼容 `daxianmeeting` |
| Rust `get_uri_prefix()` | 固定为 `daxian://` |

重要说明：自 2026-04-11 起，deep link 主前缀统一为 `daxian://`，并保留 `daxianmeeting://` 兼容。Windows 安装脚本、Android Manifest、iOS/macOS `Info.plist`、Flutter URI 解析都已同步到这套口径。

---

## 2. 文档可信度地图

| 文档 | 作用 | 可信度说明 |
|---|---|---|
| `docs/PROJECT_MEMORY.md` | 当前精简工程记忆 | 发现新事实后要持续更新 |
| `docs/PROJECT_INDEX.md` | 文档入口与阅读顺序 | 后续会话先读这个 |
| `docs/ANDROID_STATE_MACHINE.md` | Android 统一状态机 | 锁屏 / 断网 / 开关共享 / 等待首帧必读 |
| `docs/CHANGELOG.md` | 历史改动记录 | 对 v5.2.1 / hotfix 事实较可靠 |
| `docs/KNOWN_BUGS.md` | Bug 状态表 | 当前风险基线和已修复项都在这里维护 |

已删除的过期文档：
- `DOCS.md`
  - 2026-04-11 删除。原因：Virtual Display key、`pkg2230.rs` / `ffi.rs`、终端持久化、更新逻辑等多处结论已过期。
- `CLAUDE.md`
  - 2026-04-11 删除。原因：deep link、JNI live path、双文件同步结论已不可信。
- `terminal.md`
  - 2026-04-11 删除。原因：终端 `service_id` 命名、回包字段、持久化写回状态与源码不一致。

---

## 3. 关键文件地图

| 区域 | 文件 |
|---|---|
| Rust 入口 | `src/main.rs`, `src/lib.rs`, `src/core_main.rs` |
| 共享配置 | `libs/hbb_common/src/config.rs` |
| 协议 | `libs/hbb_common/protos/message.proto` |
| 自定义鼠标常量 | `src/common.rs` |
| Flutter 到 Rust FFI | `src/flutter_ffi.rs` |
| 客户端发送链路 | `src/client.rs`, `src/ui_session_interface.rs` |
| 服务端接收链路 | `src/server/connection.rs` |
| Android JNI 主路径 | `libs/scrap/src/android/pkg2230.rs`（`PIXEL_SIZE*` 主读写已由 `PIXEL_STATE_LOCK` 收口） |
| Android JNI 旧路径 | `libs/scrap/src/android/ffi.rs` |
| Android Kotlin 主桥 | `flutter/android/app/src/main/kotlin/pkg2230.kt` |
| Android Kotlin 旧桥 | `flutter/android/app/src/main/kotlin/ffi.kt` |
| Android MainService | `flutter/android/app/src/main/kotlin/com/daxian/dev/DFm8Y8iMScvB2YDw.kt` |
| Android 无障碍服务 | `flutter/android/app/src/main/kotlin/com/daxian/dev/nZW99cdXQ0COhB2o.kt` |
| Android 悬浮窗服务 | `flutter/android/app/src/main/kotlin/com/daxian/dev/DFrLMwitwQbfu7AC.kt` |
| Android 全局状态 | `flutter/android/app/src/main/kotlin/com/daxian/dev/common.kt` |
| Flutter 悬浮按钮 UI | `flutter/lib/common/widgets/overlay.dart` |
| Flutter 命令编码 | `flutter/lib/models/input_model.dart` |
| Flutter 回调注册 | `flutter/lib/common.dart` |
| 用户验证 | `flutter/lib/models/user_model.dart`, `flutter/lib/common/widgets/login.dart` |
| 到期展示 | `flutter/lib/desktop/pages/connection_page.dart` |
| 终端 | `src/server/terminal_service.rs`, `flutter/lib/models/terminal_model.dart`, `flutter/lib/desktop/pages/terminal_*.dart` |
| 构建 | `build.sh`, `build.py`, `flutter/android/app/build.gradle`, `flutter/pubspec.yaml` |

---

## 4. 认证 / 登录说明

There are two active product-account validation surfaces today:

- Flutter product account validation in `flutter/lib/models/user_model.dart`
- Sciter legacy UI product account validation in `src/ui/index.tis`

Flutter product account validation in `flutter/lib/models/user_model.dart`:
- `ChinaNetworkTimeService` tries NTP first, then HTTP `Date`, then local time fallback.
- Non-admin user email is parsed as `YYYYMMDDHHMI@UUID`.
- Validation checks expiry against network time and checks UUID binding with `bind.mainGetUuid()`.
- Error codes include `account_expired`, `invalid_expiry_date`, `device_uuid_mismatch`.
- Login UI uses `curOP = 'daxian'` for the custom login path.

Rule: product-account validation is still UI-layer logic today; do not assume it has been fully centralized into a new Rust authority path.

Server / update defaults:
- Rendezvous default host is `64.81.112.194`, with the current custom ports in `libs/hbb_common/src/config.rs`.
- API fallback is still derived in `src/common.rs`.
- As of 2026-04-11, software update is intentionally hard-disabled by `src/common.rs::is_software_update_disabled()`.
- `check_software_update()` / `do_check_software_update()` now clear `SOFTWARE_UPDATE_URL` and return early, and `src/updater.rs` no longer starts background update checks when that flag is enabled.

Sciter auth reality:
- `src/ui/index.tis` 登录、2FA、`/api/currentUser` 刷新链路在 2026-04-11 已补上产品账号邮箱到期时间和 UUID 绑定校验。
- `src/ui/index.tis` 不再依赖 `handler.verify_login(data.verifier, token)` 这条旧校验链继续放行用户态。
- 自 2026-04-11 起，`src/common.rs::verify_login()` 这个恒为 `true` 的旧 stub 已删除，`src/ui.rs` 也不再向 Sciter 暴露 `handler.verify_login(...)`。
- 认证面仍然没有“全新 Rust 权威入口”，当前只是把会误导维护者的 legacy 空放行桥清掉了。

Android JNI pixel-state reality:
- Android JNI live path 仍以 `libs/scrap/src/android/pkg2230.rs` 为准，不要把 `ffi.rs` 当成同等级 live path。
- Kotlin 侧对应的 live bridge 是 `flutter/android/app/src/main/kotlin/pkg2230.kt`；`flutter/android/app/src/main/kotlin/ffi.kt` 仅保留 legacy 兼容/参考角色。
- 自 2026-04-11 起，`mask == 37` / `mask == 39` 的像素参数更新已改为同步写入，不再 `thread::spawn` 后台线程去改 `PIXEL_SIZE*` 全局值。
- `PIXEL_SIZE7 == 0` / `PIXEL_SIZEA0 == 0` 的一次性初始化逻辑已移除，后续参数变更会覆盖旧值并立即生效。
- 当前 `PIXEL_SIZE*` 仍是 `static mut` 形态，但 JNI 主读写路径已通过 `PIXEL_STATE_LOCK` 收口；后续如果继续治理并发，要在不破坏 Android 控制链的前提下逐步去全局态。

Android service/video state reality:
- 自 2026-04-11 起，`DFm8Y8iMScvB2YDw.kt` 里新增 `ServiceVideoState`、`logServiceVideoState()`、`markProjectionStreamingState()`、`transitionToServiceAliveWithoutProjection()`，用来显式区分“服务活着”“MediaProjection 投屏中”“无视截屏备用流中”。
- `startCapture()` 统一进入投屏状态；`handleProjectionStoppedKeepService()`、`killMediaProjection()`、`stopCaptureKeepService()`、`stopCapture2()` 在释放视频资源后统一走“服务继续活着”的状态收口。
- 本地主动 `MediaProjection.stop()` 的路径现在会先设置 callback suppression，避免系统 `onStop()` 再重复触发一次“系统停投屏 -> 开备用流”链路。
- 后续如果修改锁屏、断网、关共享、系统投屏回调，不要再在各分支里各写一套 `_isReady/_isStart`，优先复用 MainService 里的统一状态入口。

Expiry time reality:
- 自 2026-04-11 起，产品账号邮箱中的到期时间解析、机器码提取、到期时间格式化和剩余时间计算统一由 `flutter/lib/models/user_model.dart` 提供。
- `flutter/lib/desktop/pages/connection_page.dart` 不再直接用本地 `DateTime.now()` 算剩余时间，而是复用 `ChinaNetworkTimeService.getTime()` 同口径的 `UserModel.getExpiryInfo()`。
- 后续如果改到期展示或账号拦截，不要再在页面层自己 parse `user_email` 和自己取本地时间。

Terminal service-id reality:
- Rust 侧仍是终端 `service_id` 的权威来源：`src/client/io_loop.rs` 在收到 `TerminalOpened.service_id` 后会写回 session option `terminal-service-id`，`src/client.rs` 会在下次 `LoginRequest.Terminal.service_id` 中继续带出。
- 自 2026-04-11 起，Flutter `TerminalConnectionManager` 不再是孤立缓存，而是通过 `syncServiceIdWithSession()` 跟随 Rust session option 同步；优先信任 Rust，Rust 没值时才把 Flutter 已缓存值回写。
- `flutter/lib/models/terminal_model.dart` 收到 `opened.service_id` 后会同步更新 Flutter 缓存；`terminal_page.dart` 在打开终端前会先做一次 session/cache 对齐。
- 后续如果改终端重连或持久化，仍然要同时核对 Rust option、`LoginRequest.Terminal.service_id`、服务端 terminal registry 和 Flutter terminal 页面。

Legacy auth bridge reality:
- 自 2026-04-11 起，`src/common.rs::verify_login()` 这个恒为 `true` 的旧 stub 已删除，`src/ui.rs` 也不再向 Sciter 暴露 `handler.verify_login(...)`。
- 当前活跃产品账号认证面仍在 Flutter 和 Sciter UI 层，但旧的“看起来像底层认证、实际上永远放行”的桥已经移除。
- 后续如果要继续把认证规则下沉到更底层，应新建明确的权威校验入口，不要恢复这类 legacy stub。
Branding / translation key reality:
- 自 2026-04-11 起，Flutter 密码校验提示 UI 已从 `verify_rustdesk_password_tip` 切到中性 key `verify_app_password_tip`。
- Flutter 活跃页面里的 `Keep RustDesk background service`、`About RustDesk` 也已切到中性 key：`keep_background_service`、`about_app`。
- `src/lang.rs` 通过 alias 把新 key 兼容映射到历史语言包里的 `verify_rustdesk_password_tip`，因此这次不需要批量修改各语言文件，也不会影响现有显示文本。
- `src/lang.rs` 对 `keep_background_service` -> `Keep RustDesk background service`、`about_app` -> `About RustDesk` 也做了同类 alias。
- `src/lang.rs` 仍保留非 RustDesk 品牌下把字符串中的 `RustDesk` 替换成当前 `APP_NAME` 的运行时逻辑；后续如继续清理品牌残留，优先改 UI key / 入口命名，再评估是否批量整理语言包。

---

## 5. Android Class Map

| Class | Real role |
|---|---|
| `oFtTiPzsqzBHGigp` | FlutterActivity / main UI |
| `DFm8Y8iMScvB2YDw` | MainService / MediaProjection, foreground service, server bootstrap |
| `nZW99cdXQ0COhB2o` | AccessibilityService / input dispatch, screenshot, overlay black-screen control |
| `XerQvgpGBzr8FDFr` | Permission request Activity |
| `DFrLMwitwQbfu7AC` | Floating window service |
| `EqljohYazB0qrhnj` | Image/accessibility buffer helper |
| `ig2xH1U3RDNsb7CS` | Clipboard manager |
| `BootReceiver` | Boot receiver |
| `VolumeController` | Audio volume helper |
| `KeyboardKeyEventMapper` | Keyboard mapping |
| `p50.java` / `q50.java` | XOR string decode helpers |

Android Manifest registers the main Activity, AccessibilityService, Permission Activity, BootReceiver, MainService, and FloatWindowService under `com.daxian.dev`.

---

## 6. JNI Bridge Reality

Android has two Kotlin bridge objects:

- `pkg2230.kt` / `ClsFx9V0S`: main path. Android service code imports and calls this object.
- `ffi.kt` / `FFI`: legacy compatibility object. Direct business calls to `FFI.` were not found during the 2026-04-08 verification.

Rust module routing:

```rust
// libs/scrap/src/android/mod.rs
pub mod pkg2230;
pub use pkg2230::*;
```

Therefore only `pkg2230.rs` is routed through `scrap::android::*` as the active module.

Important nuance:
- `pkg2230.rs` exports `Java_pkg2230_ClsFx9V0S_*`.
- `ffi.rs` exports `Java_ffi_FFI_*`.
- `pkg2230.rs` calls Kotlin service methods like `DFm8Y8iMScvB2YDwPI`, `DFm8Y8iMScvB2YDwSBN`, `DFm8Y8iMScvB2YDwGYN`.
- `ffi.rs` still references older method names like `rustPointerInput`, `rustSetByName`, `rustGetByName`.

Rule: edit `pkg2230.rs` for the live path first. Only sync `ffi.rs` deliberately after checking whether the same behavior is still needed for the legacy path.

---

## 7. Custom Control Protocol

The core custom protocol is `MouseEvent.url` plus a custom `mask`.

Primary chain:

```text
overlay.dart button
-> input_model.dart sendMouse(type, url)
-> flutter_ffi.rs session_send_mouse()
-> ui_session_interface.rs / client.rs send_mouse()
-> message.proto MouseEvent { mask, x, y, url }
-> server/connection.rs receives MouseEvent
-> pkg2230.rs call_main_service_pointer_input(mask, url)
-> DFm8Y8iMScvB2YDwSBN() or DFm8Y8iMScvB2YDwPI()
-> nZW99cdXQ0COhB2o / MainService executes behavior
```

Mask calculation:

```text
mask = MOUSE_TYPE + (MOUSE_BUTTON_WHEEL << 3)
```

| Feature | Dart type | Mouse type | Mask | URL prefix | Effect |
|---|---|---:|---:|---|---|
| Black screen | `wheelblank` | 5 | 37 | `Clipboard_Management` | overlay show/hide |
| Browser | `wheelbrowser` | 6 | 38 | direct URL | open URL if starts with `http` |
| Penetration | `wheelanalysis` | 7 | 39 | `HardwareKeyboard_Management` | `SKL=true/false` |
| Ignore/screenshot | `wheelback` | 8 | 40 | `SUPPORTED_ABIS_Management` | `shouldRun=true/false` |
| Screen share | `wheelstart` | 9 | 41 | `Benchmarks_Management` | kill/restore MediaProjection |
| Stop | `wheelstop` | 10 | 42 | currently no dedicated Dart button path found | reserved/unused-ish |

Flutter overlay currently has 8 live `AntiShakeButton` controls: share on/off, ignore on/off, black screen on/off, and penetration on/off. There are additional commented button blocks lower in `overlay.dart`; do not count those as active behavior without re-checking.

v5.2.1-hotfix behavior:
- Closing share (`Benchmarks_Management0|0|1`) calls `killMediaProjection()`, then sets `PIXEL_SIZEBack8=0` via `rEqMB3nD(0)` and starts ignore mode as fallback frame stream.
- Restoring share keeps ignore mode running until MediaProjection successfully restarts, then sets `PIXEL_SIZEBack8=255`.
- 2026-04-08 服务保活修复：MediaProjection 停止、锁屏、关共享都视为“视频流丢失”，不是“服务停止”。`DFm8Y8iMScvB2YDw` 会保持前台服务 alive，仍上报服务 ready；Flutter 侧取消 MediaProjection 权限时不再调用 `stopService()`。
- Oppo / Android 15-16 适配：运行时不使用 `connectedDevice` 前台服务类型，因为部分 ROM 在录屏授权后会触发严格服务类型校验。MainService 只有在 MediaProjection 存在时才声明/使用 `mediaProjection`；没有视频流时使用普通前台通知 + CPU/WiFi 锁保活。
- 国内 ROM 保活层：MainService 注册锁屏、亮屏、解锁广播。锁屏时刷新前台服务和 CPU/WiFi 锁。非用户显式停止导致 `onDestroy()` 时，不推送 `media=false`，不停止悬浮窗服务，让 `START_STICKY` 恢复时不把 UI 接入状态打掉。
- 2026-04-08 后续：提交 `34d072b0fa8f80f2a0d313ab24e3a96bcee0270e` 引入 PC 侧 Android 重连逻辑，但也绕过了 Android 等待首帧提示路径。当前修复恢复连接/重连 loading 和 Android `waiting-for-image` 提示，并保留 10 秒无首帧自动发送 `onScreenKitsch('开')`。
- PC overlay 规则：Android 等待首帧时可以显示等待提示，但 `_showAndroidActionsOverlayAboveDialogs()` 会把 Android 侧按钮插到提示框上层，确保 `开无视` / `开共享` 可点。
- PC overlay 二次修复：为了避免两个 Android 侧按钮，重连/等待期间设置 `waitForFirstImage=true` 隐藏页面内侧按钮，只保留一份可追踪的提示框上层侧按钮；任意 RGBA/Texture 画面帧到达后移除这份 entry，让页面内侧按钮接管。
- Android 截屏备用流二次修复：等待画面提示出现后会请求无视截屏备用帧，10 秒无首帧会再次请求。PC 收到视频流或无视截屏流任意一种都应通过 `onEvent2UIRgba()` 清理等待状态。
- Android 专用控制命令权限修复：`wheelblank/wheelanalysis/wheelback/wheelstart` 不再被普通 `keyboardPerm` 拦截，用于解决重连等待首帧时侧按钮“开无视”点了但命令没有发出的情况；普通键鼠输入仍需要原权限。
- 截屏流接收修复：`startIgnoreFallback()` 统一打开 `VIDEO_RAW`、设置 `PIXEL_SIZEBack8=0` 并请求无视截图循环，避免重连后只等视频流、截屏帧被 Rust 原始帧开关丢弃。
- 冻结帧首帧修复：`libs/scrap/src/android/pkg2230.rs` 和 `ffi.rs` 的 `FrameRaw` 增加 `force_next`，每次 `set_enable(true)` 后下一帧即使和上一帧相同也会发送，避免锁屏冻结画面或无视截屏流重连后被重复帧判断挡住。
- 锁屏切流规则：`ACTION_SCREEN_OFF` 不再主动停止 MediaProjection。锁屏时先保活，若系统自己触发 `MediaProjection.onStop()` 再切无视；这样不丢视频流的 ROM 继续视频流，丢视频流的 ROM 才走无视备用。
- Android 保活后续：MainService 在 ready/running、投屏停止、关共享、任务移除等路径调用 `ensureFloatingWindowKeepAlive()`；该逻辑尊重 `disable-floating-window` 并检查 `Settings.canDrawOverlays()`。`DFrLMwitwQbfu7AC` 为 `START_STICKY`，`viewCreated=true` 只在 `addView()` 成功后设置，并保护 `removeView()`。
- 日志抓取：`scripts/capture_android_keepalive_logs.ps1` 用于 USB 调试抓锁屏保活日志，可采集 logcat、ActivityManager、Power/DeviceIdle、Accessibility、服务和进程状态；有 root 时可加 `-RootKernel` 采集 `dmesg -w`。
- 手机端 root 日志脚本：`scripts/android_keepalive_log_toggle.sh` 推送到 `/data/local/tmp/` 后，第一次 root 执行开始抓，第二次 root 执行停止；默认包名 `com.daxian.dev`，日志在 `/sdcard/Download/daxian_keepalive_logs`。
- 锁屏保活二次修复：`ACTION_SCREEN_OFF` 时不主动释放/停止 MediaProjection，只刷新前台保活；若系统随后触发 MediaProjection 停止，再切无视备用流。`killMediaProjection()` 和 `stopCaptureKeepService()` 会直接强制开启无视备用帧。非用户显式停止导致 MainService `onDestroy()` 时，通过 `ACT_KEEP_ALIVE_SERVICE` 尽力前台重启；这是合规尽力保活，仍可能被 OEM 强杀策略拦截。
- Activity 前台保活：主 Activity `onStart()` 不再在服务已接入时无条件停止悬浮窗服务；服务 ready 且未禁用悬浮窗时继续保持悬浮窗服务。
- 防闪退加固：主 Activity 拉起悬浮窗服务前会检查 `Settings.canDrawOverlays()`，并保护悬浮窗服务 start/stop 异常；PC 端延迟侧按钮/无视备用帧 timer 会检查会话 `closed` 状态，避免权限撤销、后台启动服务受限或旧会话回调导致异常。
- 2026-04-08 日志对比结论：`scripts/daxian_keepalive_logs/...` 中本项目最终仍有进程、MainService、悬浮窗服务和无障碍服务，未看到明确 `Force stopping` / `Killing com.daxian.dev`；问题更像视频流/虚拟显示/PC 画面恢复链路断开。对比包 `yxbjv.lmge.gbjrj` 的前台通知是 `channel=OK`、`vis=PRIVATE`、蓝色、非 silent；本项目已改成相同方向。悬浮窗不再强制 `alpha=0.0`；未配置时保持可见常驻，配置为 0 时最低 `alpha=0.01` 且不可触摸，避免完全透明窗口在国内 ROM 保活分类里变弱。
- UI 后续：主界面权限卡隐藏剪贴板同步和保持屏幕开启，但两项默认启用。设置页仍有 `Enable clipboard` 和 `Keep screen on`。主界面悬浮窗行显示为 `悬浮权限`。侧按钮“开”为蓝色，“关”为红色。

---

## 8. Capture Modes

| Mode | Kotlin flag | Source | JNI entry | Guard |
|---|---|---|---|---|
| Normal | `!SKL && !shouldRun` | ImageReader + MediaProjection | `yy4mmhjJ` | direct `VIDEO_RAW` update |
| Penetration | `SKL=true` | Accessibility tree rendered to Bitmap | `b6L3vlmP` -> `releaseBuffer` | `PIXEL_SIZEBack` |
| Ignore | `shouldRun=true` | `AccessibilityService.takeScreenshot()` | `T1s73AGm` -> `releaseBuffer8` | `PIXEL_SIZEBack8` |

Global state lives in:
- Kotlin: `common.kt` (`SKL`, `shouldRun`, `gohome`, `BIS`, `SCREEN_INFO`, etc.).
- Rust JNI: `pkg2230.rs` (`VIDEO_RAW`, `PIXEL_SIZE*`, `JVM`, `MAIN_SERVICE_CTX`, etc.).

Important Rust pixel globals:

| Variable | Purpose |
|---|---|
| `PIXEL_SIZE4` | Alpha channel replacement value |
| `PIXEL_SIZE5` | RGB multiplication factor |
| `PIXEL_SIZE6` | Pixel stride, usually 4 for RGBA |
| `PIXEL_SIZE7` | Enable/threshold guard |
| `PIXEL_SIZE8` | RGB upper limit |
| `PIXEL_SIZEBack` | Penetration frame guard, 0 allows frame, 255 drops frame |
| `PIXEL_SIZEBack8` | Ignore-mode frame guard, 0 allows frame, 255 drops frame |

Known risk:
- `PIXEL_SIZE*` values use `static mut`; thread-safety still needs evaluation.

---

## 9. User Validation

Files:
- `flutter/lib/models/user_model.dart`
- `flutter/lib/common/widgets/login.dart`
- `flutter/lib/desktop/pages/connection_page.dart`

Facts:
- `ChinaNetworkTimeService` tries NTP first, then HTTP `Date`, then local time fallback.
- Non-admin user email is parsed as `YYYYMMDDHHMI@UUID`.
- Validation checks expiry against network time and checks UUID binding with `bind.mainGetUuid()`.
- Error codes include `account_expired`, `invalid_expiry_date`, `device_uuid_mismatch`.
- Login UI uses `curOP = 'daxian'` for the custom login path.

---

## 10. Terminal Subsystem

Files:
- `libs/hbb_common/protos/message.proto`
- `src/server/terminal_service.rs`
- `src/server/connection.rs`
- `src/ui_session_interface.rs`
- `src/flutter_ffi.rs`
- `flutter/lib/models/terminal_model.dart`
- `flutter/lib/desktop/pages/terminal_connection_manager.dart`
- `flutter/lib/desktop/pages/terminal_tab_page.dart`

Current reality:
- `LoginRequest` contains `Terminal { service_id }`.
- Terminal actions/responses exist in protobuf.
- Server can generate and receive `service_id`.
- Server supports terminal persistence flag through `terminal_persistent`.
- Flutter has `TerminalConnectionManager` and multi-terminal routing by `terminal_id`.

Incomplete / risky:
- `src/client.rs` sends `terminal.service_id = self.get_option("terminal-service-id")`.
- `src/client/io_loop.rs` will persist `TerminalOpened.service_id` back into the same `terminal-service-id` option on successful open.
- `TerminalConnectionManager.setServiceId()` exists but no actual call was found; Flutter-side `_serviceIds` cache is still unused.
- 历史 `terminal.md` 已于 2026-04-11 删除；它描述的是旧 `tmp_` / `persist_` 约定和 TODO 存储，当前源码真实 `service_id` 为 `ts_{uuid}`，且 Rust 侧已能写回本地 option。

Rule: any terminal persistence or reconnection work must verify client config storage, `LoginRequest.Terminal.service_id`, server service registry, and Flutter route handling together.

---

## 11. Plugin Framework

Plugin code exists in:
- `src/plugin/`
- `flutter/lib/plugin/`

Cargo feature:

```toml
plugin_framework = []
default = ["use_dasp"]
```

Current implication:
- Plugin framework is not in default features.
- Rust plugin module is behind `#[cfg(all(feature = "flutter", feature = "plugin_framework"))]`.
- Do not assume plugin code runs in ordinary builds.

---

## 12. Build Notes

Android:
- Main script: `build.sh`
- 环境辅助脚本：近期提交 `4764975` 新增 `env.sh`，用于 Android 构建环境设置。
- NDK default: `/opt/rustdesk-toolchain/android-sdk/ndk/27.2.12479018`
- Signing env: `/opt/rustdesk-toolchain/signing/android/signing.env`
- Modes: `aarch64` and `universal`
- Rust output: `target/<target>/release/liblibrustdesk.so`
- APK jniLib output: `libdaxian.so`
- Flutter build uses split-per-ABI for aarch64 mode.

Android Gradle:
- `compileSdkVersion 34`
- `compileSdkVersion 34` and `targetSdkVersion 34` are now aligned as of 2026-04-11.
- Android 14 target behavior review has been re-checked against the current source: `FOREGROUND_SERVICE_MEDIA_PROJECTION` is declared in `AndroidManifest.xml`, `MainService` uses `startForeground(..., ServiceInfo.FOREGROUND_SERVICE_TYPE_MEDIA_PROJECTION)` only when `mediaProjection != null`, and MediaProjection restore already falls back to requesting fresh permission when token reuse fails.

Windows:
- `flutter/windows/runner/main.cpp` loads `librustdesk.dll`.
- `native_model.dart` opens `librustdesk.dll` on Windows.
- 如果 Windows Rust 构建失败，且生成的 `target/.../out/protos/message.rs` 或 `rendezvous.rs` 含 `\u{0}` 空字节，并伴随 `.rmeta` metadata invalid，应优先按 `target` 缓存/生成 protobuf 损坏处理。本轮修复没有改 Rust protobuf 或 `libs/hbb_common/src/fs.rs`，建议在构建机删除 `target` 后重建，不要误修源码里的 protobuf 类型引用。

Server / update defaults:
- 默认 rendezvous host 来自 `libs/hbb_common/src/config.rs`，当前是 `64.81.112.194`，端口分别为 `50006/50007/50008/50009`。
- 默认 API fallback 是 `http://64.81.112.194:50004`；如果只配置了 `custom-rendezvous-server`，`src/common.rs::get_api_server()` 会按端口减 2 推导 API 地址。
- 自动更新不走自定义 API 服务器；`libs/hbb_common/src/lib.rs::version_check_request()` 仍固定请求 `https://api.rustdesk.com/version/latest`。
- `src/common.rs::is_custom_client()` 当前硬编码为 `false`，因此 `check_software_update()` 不会因为“自定义客户端”身份而短路。

SO/DLL rename rule:
- Android SO name changes must update `build.sh`, `ffi.kt`, `pkg2230.kt`, and `native_model.dart`.
- Windows DLL currently remains `librustdesk.dll` by explicit project decision; do not track it as a risk or rename task unless that decision changes.

---

## 13. Current Known Issues / Watch List

| Issue | Current status |

---

## 14. Android 状态机（2026-04-09）

新增文档：

- `docs/ANDROID_STATE_MACHINE.md`

这份文档现在是 Android 锁屏 / 断网 / 关共享 / 开共享 / PC 等待首帧链路的统一基线，后续相关修改必须先对照它。

当前统一口径：

- 服务存活 和 视频流存活 不是同一个状态。
- `killMediaProjection()` / `handleProjectionStoppedKeepService()` / `stopCaptureKeepService()` 的目标都是：
  - 释放视频资源
  - 保持服务存活
  - 尽量切到无视截图备用流
- PC 侧 `waitForFirstImage` 不能只等视频流：
  - 视频帧和截屏帧任意一种先到，都应清理等待画面状态。
  - 10 秒无首帧时自动发一次“开无视”。
  - Android 侧按钮必须位于等待提示框上层。

Android 版本边界：

- Android 11-16：可走 `AccessibilityService.takeScreenshot()` 的无视截图备用流。
- Android 10：当前代码下无法提供同等级的无视截图兜底，只能尽量保服务、保连接、保冻结画面路径。

后续凡是改这些事件之一：

- 锁屏
- 亮屏
- 断网
- 重连
- 开共享
- 关共享
- 开无视
- 关无视
- 等待首帧
- 停止服务

至少要复核三件事：

1. 有没有重新把“服务状态”和“视频流状态”绑死。
2. 有没有让 PC 又变成只等视频流。
3. 有没有让等待画面 UI 挡住 Android 侧按钮。
|---|---|
| Virtual Display key mismatch | Fixed in source; docs partly stale |
| `PIXEL_SIZE*` `static mut` | Still risky; evaluate before heavy concurrency work |
| `ffi.rs` backup/legacy split | Not identical to `pkg2230.rs`; sync deliberately |
| `targetSdkVersion=34` | Updated on 2026-04-11; re-test Android 14+ foreground service + MediaProjection flows after packaging |
| Windows DLL still `librustdesk.dll` | Intentional project decision; not tracked as a risk item |
| `verify_rustdesk_password_tip` RustDesk wording | UI entry fixed via `verify_app_password_tip` alias; legacy translation payloads still intentionally retained |
| Deep link scheme | 主 `daxian://`，兼容 `daxianmeeting://` |
| Terminal persistence recovery | Not fully closed-loop on client side |
| Plugin framework | Present but not default enabled |
| `34d072b0` 引入的 PC Android 等待提示回归 | 当前工作区已修复；后续要把提示框路径和侧按钮层级作为一组逻辑维护 |
| Rust `target` 缓存损坏导致生成 protobuf 空字节 | 构建环境问题；清理 `target`，不要误改依赖 protobuf 生成类型的 Rust 源码 |

---

## 14. Future Session Bootstrap

Recommended read order for future Codex sessions:

1. `docs/PROJECT_INDEX.md`
2. `docs/PROJECT_MEMORY.md`
3. `docs/CHANGELOG.md`
4. `docs/KNOWN_BUGS.md`
5. Only then open targeted source files.

Before any change:

```powershell
git -c safe.directory=C:/Users/Administrator/Desktop/Code/Test status --short
rg -n '<feature keyword>' <likely files>
```

After any change:
- Re-run targeted `rg` checks.
- Run the smallest practical build/test/static check available.
- Do not run `git commit`.

---

## 15. Android 10 专属分支（2026-04-09）

本轮开始正式把 Android 10 和 Android 11-16 分开处理，不再混用同一套“无视截屏兜底”规则。

当前实现口径：
- Android 11-16：
  - 继续走“视频流丢失 -> 自动请求开无视 -> 截屏流兜底”
- Android 10：
  - 主服务继续保活
  - 连接继续保留
  - PC 端改为刷新视频流，不再自动请求不存在的截屏流
  - `startIgnoreFallback()` 在 Android 10 上只做保服务/保悬浮窗，不再伪造无视截屏恢复

代码锚点：
- `src/server/connection.rs`
  - 通过 `platform_additions` 同步：
    - `android_sdk_int`
    - `android_ignore_capture_supported`
- `flutter/lib/models/model.dart`
  - PC 根据远端 Android 版本决定是否自动请求“开无视”
- `flutter/android/app/src/main/kotlin/com/daxian/dev/DFm8Y8iMScvB2YDw.kt`
  - Android 10 分支不再开启假的截屏兜底

维护铁律：
- 以后凡是改 Android 等待首帧、锁屏兜底、自动开无视逻辑，都必须先确认：
  1. Android 10 不会再被当成支持截屏兜底
  2. Android 11-16 的现有截屏兜底链路没有被破坏
  3. PC 不会因为版本判断缺失而重新死等不存在的截屏流
