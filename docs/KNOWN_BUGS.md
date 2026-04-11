# 已知 Bug 列表

| 序号 | 问题 | 严重性 | 位置 | 状态 |
|------|------|--------|------|------|
| 1 | Virtual Display key 不匹配：Rust 发 "rustdesk_virtual_displays"，Dart 期望 "daxian_virtual_displays" | 高 | virtual_display_manager.rs vs consts.dart | **已修复 v5.2.1** |
| 2 | `pkg2230.rs` 中 `PIXEL_SIZE*` 采用 `static mut`，且控制命令分支会起后台线程写这些全局值，存在竞态和状态撕裂风险 | 高 | `libs/scrap/src/android/pkg2230.rs` | **已修复 2026-04-12（live path 已改为 `Mutex<PixelState>` 统一承载；按钮协议和控制链未改）** |
| 3 | 把 `ffi.rs` 当成必须同步的 live path 是过期结论；当前 live path 是 `pkg2230.rs`，误同步可能把旧 JNI/旧服务方法名回灌到主链 | 高 | `libs/scrap/src/android/`, `flutter/android/app/src/main/kotlin/` | **部分缓解 2026-04-11（源码已明确 `pkg2230` 为 live path，`ffi` 标为 legacy；旧文件仍保留）** |
| 4 | targetSdkVersion=33，低于 Google Play 要求的 34+ | 中 | build.gradle | **已修复 2026-04-11（已提升到 34；Android 14+ 需重点回归前台服务与投屏授权恢复链）** |
| 6 | verify_rustdesk_password_tip 翻译 key 残留 RustDesk | 低 | dialog.dart | **已修复 2026-04-11（UI 入口改用中性 key，lang.rs 对旧 key 做兼容别名）** |

## 新增发现并修复的 Bug（v5.2.1）

| 序号 | 问题 | 严重性 | 位置 | 状态 |
|------|------|--------|------|------|
| 7 | 黑屏 Overlay 无法阻止触摸（缺少 FLAG_NOT_TOUCHABLE 动态切换） | 高 | nZW99cdXQ0COhB2o.kt | **已修复 v5.2.1-hotfix** (动态切换+远程穿透) |
| 8 | 穿透→关穿透导致无视模式意外恢复（shouldRun 未清除） | 高 | nZW99cdXQ0COhB2o.kt | **回退修复** (原始逻辑正确，v5.2.1的shouldRun=false已移除) |
| 9 | InputService Handler 泄漏（onDestroy 未停止 runnable） | 中 | nZW99cdXQ0COhB2o.kt | **已修复 v5.2.1** |
| 10 | 缺少 FOREGROUND_SERVICE_MEDIA_PROJECTION 权限（Android 14+ 崩溃） | 高 | AndroidManifest.xml + DFm8Y8iMScvB2YDw.kt | **已修复 v5.2.1** |
| 11 | Android 14+ MediaProjection Token 不可复用（恢复共享延迟） | 中 | DFm8Y8iMScvB2YDw.kt | **已修复 v5.2.1** |
| 12 | 关共享后画面冻结（MediaProjection销毁后无备用帧流） | P0 | DFm8Y8iMScvB2YDw.kt | **已修复 v5.2.1-hotfix** (关共享自动激活无视) |
| 13 | 开共享需App在前台（后台Service无法启动授权Activity） | P0 | DFm8Y8iMScvB2YDw.kt | **已修复 v5.2.1-hotfix** (无视保持+自动startCapture) |

## 接管阶段新增待处理风险（2026-04-11）

| 序号 | 问题 | 严重性 | 位置 | 状态 |
|------|------|--------|------|------|
| 14 | 自动更新仍请求 RustDesk 官方版本接口，当前品牌/服务器改造没有覆盖更新链路 | 高 | `libs/hbb_common/src/lib.rs`, `src/common.rs` | **已修复 2026-04-11（已整体禁用自动更新）** |
| 15 | Deep link scheme 分裂：Android Manifest 是 `daxian`，Rust `get_uri_prefix()` 生成 `daxianmeeting://`，不同入口路径行为不一致 | 高 | `AndroidManifest.xml`, `src/common.rs`, `flutter/lib/common.dart` | **已修复 2026-04-11（主前缀统一为 `daxian://`，保留 `daxianmeeting://` 兼容）** |
| 16 | Rust `verify_login()` 直接放行，产品账号约束主要在 Flutter；如果未来入口或调用面变化，容易形成认证旁路 | 高 | `src/common.rs`, `src/ui.rs`, `flutter/lib/models/user_model.dart`, `src/ui/index.tis` | **已修复 2026-04-11（Sciter 已改为直接做产品账号校验，旧 `verify_login` 空放行桥已删除）** |
| 17 | `pkg2230.rs` 里的 `PIXEL_SIZE7 == 0` / `PIXEL_SIZEA0 == 0` 只初始化一次，后续控制命令参数变化可能不会生效直到进程重启 | 高 | `libs/scrap/src/android/pkg2230.rs` | **已修复 2026-04-11（参数改为每次命令实时覆盖）** |
| 18 | 到期校验用网络时间，但桌面剩余时间展示用本地 `DateTime.now()`，显示口径和真实拦截口径不一致 | 中 | `flutter/lib/models/user_model.dart`, `flutter/lib/desktop/pages/connection_page.dart` | **已修复 2026-04-11（展示与拦截统一复用网络时间口径）** |
| 19 | 终端 `service_id` 已在 Rust 侧读写闭环，但 Flutter `TerminalConnectionManager._serviceIds` 仍未接线，后续终端重连改动容易误判真实状态 | 中 | `src/client.rs`, `src/client/io_loop.rs`, `flutter/lib/desktop/pages/terminal_connection_manager.dart` | **已修复 2026-04-11（Flutter 缓存已接到 Rust session option）** |
