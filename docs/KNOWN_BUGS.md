# 已知 Bug 列表

| 序号 | 问题 | 严重性 | 位置 | 状态 |
|------|------|--------|------|------|
| 1 | Virtual Display key 不匹配：Rust 发 "rustdesk_virtual_displays"，Dart 期望 "daxian_virtual_displays" | 高 | virtual_display_manager.rs vs consts.dart | 待修复 |
| 2 | PIXEL_SIZE 系列使用 static mut，线程安全隐患 | 中 | pkg2230.rs | 待评估 |
| 3 | ffi.rs 是 pkg2230.rs 冗余备份，修改需双份同步 | 中 | libs/scrap/src/android/ | 待决策 |
| 4 | targetSdkVersion=33，低于 Google Play 要求的 34+ | 中 | build.gradle | 待修复 |
| 5 | Windows DLL 名仍为 librustdesk.dll 未改名 | 低 | main.cpp, native_model.dart | 待决策 |
| 6 | verify_rustdesk_password_tip 翻译 key 残留 RustDesk | 低 | dialog.dart | 待修复 |
