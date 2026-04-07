# 项目文档入口 / Project Index

> 目标：让后续工程会话用最少上下文恢复项目状态，避免被旧文档带偏。

## 必读顺序

1. `docs/PROJECT_MEMORY.md`
   - 当前事实表、关键链路、易错点。
2. `docs/CHANGELOG.md`
   - 已落地修改，尤其 v5.2.1 和 hotfix。
3. `docs/KNOWN_BUGS.md`
   - 当前 bug 状态和已修复项。
4. `DOCS.md`
   - 大型技术手册，只作为背景资料；使用前必须回源码验证。
5. `terminal.md`
   - 终端系统设计资料；部分实现状态已变化，需与源码交叉验证。

## 可信度规则

源码 > `docs/PROJECT_MEMORY.md` > `docs/CHANGELOG.md` / `docs/KNOWN_BUGS.md` > `DOCS.md` / `CLAUDE.md` / `terminal.md`。

若文档互相冲突：
- 先 `rg` 查源码。
- 再看 `CHANGELOG` 是否说明修复或回退。
- 最后更新 `PROJECT_MEMORY.md`，避免下次重复踩坑。

## 协作铁律

- Codex 可以修改代码和文档。
- Codex 不允许执行 `git commit`。
- Git 提交只能由用户审查后亲自完成。
- 修改前必须查看工作树，避免覆盖用户改动。

## 快速检索模板

```powershell
git -c safe.directory=C:/Users/Administrator/Desktop/Code/Test status --short
rg -n "<关键词>" src libs flutter docs DOCS.md
```

## 高风险改动入口

| 改动类型 | 必查文件 |
|---|---|
| Android 控制按钮 | `overlay.dart`, `input_model.dart`, `flutter_ffi.rs`, `pkg2230.rs`, `DFm8Y8iMScvB2YDw.kt`, `nZW99cdXQ0COhB2o.kt` |
| 协议字段 | `libs/hbb_common/protos/message.proto`, `src/client.rs`, `src/server/connection.rs`, Flutter/Dart 调用点 |
| SO 名称 | `build.sh`, `ffi.kt`, `pkg2230.kt`, `native_model.dart` |
| Windows DLL 名称 | `flutter/windows/runner/main.cpp`, `native_model.dart`, Windows packaging scripts |
| Deep link | `AndroidManifest.xml`, `src/common.rs`, `src/core_main.rs`, `flutter/lib/common.dart`, `flutter/lib/mobile/pages/home_page.dart` |
| 服务认证旁路 | `src/common.rs`, `src/ui.rs`, `src/flutter_ffi.rs` |
| 用户授权/到期 | `user_model.dart`, `login.dart`, `connection_page.dart` |
| 终端 | `terminal_service.rs`, `connection.rs`, `message.proto`, `terminal_model.dart`, `terminal_connection_manager.dart`, `terminal_tab_page.dart` |
| 插件 | `Cargo.toml` feature flags, `src/plugin/`, `flutter/lib/plugin/` |

## 当前已知文档偏差

- `DOCS.md` 仍说 Virtual Display key 未修复；源码已修复。
- `DOCS.md` / 旧 `PROJECT_MEMORY.md` 曾说 `pkg2230.rs` 与 `ffi.rs` 完全相同；实际不相同，且不能盲目双文件复制。
- Android Manifest scheme 是 `daxian`，Rust `get_uri_prefix()` 当前是 `daxianmeeting://`。
- `terminal.md` 的终端持久化说明不是完整当前实现；恢复闭环仍需源码确认。
