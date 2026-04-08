# 大仙会议 / DaxianMeeting — 项目核心记忆

> 本文件是给后续工程会话快速恢复上下文用的“当前事实表”。
> 最后复核：2026-04-08。判断优先级：源码 > `docs/CHANGELOG.md` / `docs/KNOWN_BUGS.md` > `DOCS.md` / 历史手册。

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
| Android package | `com.daxian.dev` |
| Android display name | `大仙会议` |
| Runtime APP_NAME | `DaxianMeeting` in `libs/hbb_common/src/config.rs` |
| Runtime ORG | `com.carriez` still retained |
| Android SO | build output `liblibrustdesk.so`, copied to `libdaxian.so` |
| Windows DLL | `librustdesk.dll` still retained |
| Android Manifest scheme | `daxian` |
| Rust `get_uri_prefix()` | generated from `APP_NAME.to_lowercase()`, currently `daxianmeeting://` |

Important nuance: Android deep link docs saying `daxian://` are true for `AndroidManifest.xml`, but Rust-side URI helpers currently produce `daxianmeeting://`. Flutter parsing often does not strongly validate the scheme. Any deep-link change must check Android, Flutter, and Rust desktop paths together.

---

## 2. Document Truth Map

| Document | Role | Trust note |
|---|---|---|
| `docs/PROJECT_MEMORY.md` | Current compact engineering memory | Keep this updated after verified discoveries |
| `docs/PROJECT_INDEX.md` | Entry point and reading order | Read this first in future sessions |
| `docs/CHANGELOG.md` | Historical change record | Reliable for v5.2.1/hotfix facts |
| `docs/KNOWN_BUGS.md` | Bug state table | More current than `DOCS.md` for fixed bugs |
| `DOCS.md` | Large technical manual | Useful but partly stale; verify against source |
| `CLAUDE.md` | Older assistant guide | Useful, but inherits some stale statements |
| `terminal.md` | Terminal design notes | Partly stale; service_id support exists, recovery flow incomplete |

Known stale statements in `DOCS.md` / older memory:
- Virtual Display key mismatch is no longer current; both Rust and Dart use `daxian_virtual_displays`.
- `pkg2230.rs` and `ffi.rs` are not identical copies. They have different JNI exported names and service method targets; `git diff --no-index --stat` currently shows 62 changed lines.
- Current code includes v5.2.1/hotfix changes even though many docs still call the baseline v5.2.0.

---

## 3. Critical File Map

| Area | Files |
|---|---|
| Rust entry | `src/main.rs`, `src/lib.rs`, `src/core_main.rs` |
| Shared config | `libs/hbb_common/src/config.rs` |
| Protocol | `libs/hbb_common/protos/message.proto` |
| Custom mouse constants | `src/common.rs` |
| Flutter to Rust FFI | `src/flutter_ffi.rs` |
| Client send path | `src/client.rs`, `src/ui_session_interface.rs` |
| Server receive path | `src/server/connection.rs` |
| Android JNI main path | `libs/scrap/src/android/pkg2230.rs` |
| Android JNI legacy path | `libs/scrap/src/android/ffi.rs` |
| Android Kotlin main bridge | `flutter/android/app/src/main/kotlin/pkg2230.kt` |
| Android Kotlin legacy bridge | `flutter/android/app/src/main/kotlin/ffi.kt` |
| Android MainService | `flutter/android/app/src/main/kotlin/com/daxian/dev/DFm8Y8iMScvB2YDw.kt` |
| Android AccessibilityService | `flutter/android/app/src/main/kotlin/com/daxian/dev/nZW99cdXQ0COhB2o.kt` |
| Android floating service | `flutter/android/app/src/main/kotlin/com/daxian/dev/DFrLMwitwQbfu7AC.kt` |
| Android global state | `flutter/android/app/src/main/kotlin/com/daxian/dev/common.kt` |
| Flutter overlay UI | `flutter/lib/common/widgets/overlay.dart` |
| Flutter command encoding | `flutter/lib/models/input_model.dart` |
| Flutter callback registration | `flutter/lib/common.dart` |
| User validation | `flutter/lib/models/user_model.dart`, `flutter/lib/common/widgets/login.dart` |
| Expiry display | `flutter/lib/desktop/pages/connection_page.dart` |
| Terminal | `src/server/terminal_service.rs`, `flutter/lib/models/terminal_model.dart`, `flutter/lib/desktop/pages/terminal_*.dart` |
| Build | `build.sh`, `build.py`, `flutter/android/app/build.gradle`, `flutter/pubspec.yaml` |

---

## 4. Authentication / Login Notes

There are two separate validation layers:

Rust server/client compatibility bypass in `src/common.rs`:

```rust
pub fn is_custom_client() -> bool {
    false
}

pub fn verify_login(raw: &str, id: &str) -> bool {
    true
    /* original ed25519 verification remains commented */
}
```

Flutter product account validation in `flutter/lib/models/user_model.dart`:
- `ChinaNetworkTimeService` tries NTP first, then HTTP `Date`, then local time fallback.
- Non-admin user email is parsed as `YYYYMMDDHHMI@UUID`.
- Validation checks expiry against network time and checks UUID binding with `bind.mainGetUuid()`.
- Error codes include `account_expired`, `invalid_expiry_date`, `device_uuid_mismatch`.
- Login UI uses `curOP = 'daxian'` for the custom login path.

Rule: do not treat the Rust `verify_login()` bypass as a replacement for the Flutter-side Daxian account/expiry binding flow. They protect different surfaces.

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
- `terminal.md`
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
- `TerminalConnectionManager.setServiceId()` exists but no actual call was found.
- `src/client.rs` sends `terminal.service_id = self.get_option("terminal-service-id")`, but no complete client-side persistence write path was verified.
- `terminal.md` still describes TODOs and older `tmp_` / `persist_` conventions, while current service ids are generated as `ts_{uuid}`.

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
- `targetSdkVersion 33` still pending update.

Windows:
- `flutter/windows/runner/main.cpp` loads `librustdesk.dll`.
- `native_model.dart` opens `librustdesk.dll` on Windows.
- 如果 Windows Rust 构建失败，且生成的 `target/.../out/protos/message.rs` 或 `rendezvous.rs` 含 `\u{0}` 空字节，并伴随 `.rmeta` metadata invalid，应优先按 `target` 缓存/生成 protobuf 损坏处理。本轮修复没有改 Rust protobuf 或 `libs/hbb_common/src/fs.rs`，建议在构建机删除 `target` 后重建，不要误修源码里的 protobuf 类型引用。

SO/DLL rename rule:
- Android SO name changes must update `build.sh`, `ffi.kt`, `pkg2230.kt`, and `native_model.dart`.
- Windows DLL name changes must update `main.cpp` and `native_model.dart`, and then validate packaging/build scripts.

---

## 13. Current Known Issues / Watch List

| Issue | Current status |
|---|---|
| Virtual Display key mismatch | Fixed in source; docs partly stale |
| `PIXEL_SIZE*` `static mut` | Still risky; evaluate before heavy concurrency work |
| `ffi.rs` backup/legacy split | Not identical to `pkg2230.rs`; sync deliberately |
| `targetSdkVersion=33` | Still pending |
| Windows DLL still `librustdesk.dll` | Intentional/undecided |
| `verify_rustdesk_password_tip` RustDesk wording | Still present in translations, partially mitigated by `lang.rs` replacement |
| Deep link scheme split | Android `daxian`; Rust helper `daxianmeeting://` |
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
