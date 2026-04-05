# 大仙会议 修改记录

> 每次代码修改后，在此文件顶部添加记录。格式：日期 | 修改内容 | 涉及文件

---

## [v5.2.0] 基线版本 — 2026-04-05

基于 RustDesk 1.4.x 完成全部二次开发，包括：
- 品牌指纹全面替换（25+ 指纹点）
- Android 类名混淆（8 个核心类）
- 字符串 XOR 加密（p50/q50）
- 双 FFI 桥架构（ffi.kt + pkg2230.kt）
- 三模式屏幕捕获（正常/穿透/无视）
- 自定义命令协议（MouseEvent.url + mask 37/39/40/41）
- 用户验证系统（ChinaNetworkTimeService + validateUser）
- 开机自启（BootReceiver + 厂商适配）
- 认证旁路（is_custom_client=false, verify_login=true）
