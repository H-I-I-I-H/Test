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
- 2026-04-08 service keepalive fix: MediaProjection stop/lock-screen/close-share is treated as video stream loss, not service stop. `DFm8Y8iMScvB2YDw` keeps the foreground service alive, reports media as still service-ready, and Flutter no longer calls `stopService()` when MediaProjection permission is canceled.
- Oppo/Android 15-16 adaptation: do not use `connectedDevice` foreground service type at runtime because some ROMs may fail strict service-type checks after screen-capture authorization. MainService now declares/uses `mediaProjection` only when MediaProjection is active; when video is unavailable it keeps a normal foreground notification plus CPU/WiFi locks.
- Domestic ROM keepalive layer: MainService registers screen off/on/user-present receiver. On lock screen it refreshes foreground service and CPU/WiFi locks. If `onDestroy()` is not caused by explicit user stop, it does not push media=false or stop the floating service, so `START_STICKY` can restore the service without toggling the UI off.

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
