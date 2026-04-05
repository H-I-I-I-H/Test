# 大仙会议 修改记录

> 每次代码修改后，在此文件顶部添加记录。格式：日期 | 修改内容 | 涉及文件

---

## [v5.2.1-hotfix] P0 修复 — 2026-04-06

### P0-1: 黑屏防触摸修复
- 改用动态FLAG_NOT_TOUCHABLE切换：黑屏时阻止触摸，远程事件到达时临时允许穿透
- 新增 isBlackScreenActive 标志 + restoreBlockRunnable 定时器
- 文件: nZW99cdXQ0COhB2o.kt (onstart_overlay, onMouseInput, runnable, onDestroy)

### P0-2: 关共享/开共享流程修复
- 关共享后自动激活无视模式作为备用帧流
- 新增 rEqMB3nD JNI 函数设置 PIXEL_SIZEBack8
- restoreMediaProjection 保持无视运行直到MediaProjection成功恢复
- onStartCommand 新权限授权后自动 startCapture
- 文件: pkg2230.rs, ffi.rs, pkg2230.kt, DFm8Y8iMScvB2YDw.kt

### P0-3: 回退 Bug#3 修改
- 移除 onstart_capture 中的 shouldRun=false
- 原始逻辑正确：截图循环通过 if(!SKL) 跳过截图但不退出循环
- 文件: nZW99cdXQ0COhB2o.kt (onstart_capture)

---

## [v5.2.1] Bug 修复 — 2026-04-05

### Bug #1: Virtual Display Key 不匹配
- 文件: src/virtual_display_manager.rs
- 修改: "rustdesk_virtual_displays" → "daxian_virtual_displays"

### Bug #2: 黑屏 Overlay 防触摸
- 文件: nZW99cdXQ0COhB2o.kt
- 修改: onstart_overlay 中动态切换 FLAG_NOT_TOUCHABLE + updateViewLayout
- 修改: 50ms runnable 同步 FLAG 状态

### Bug #3: 穿透→关穿透状态泄漏
- 文件: nZW99cdXQ0COhB2o.kt
- 修改: onstart_capture 开穿透时清除 shouldRun=false

### Bug #4: Handler 泄漏
- 文件: nZW99cdXQ0COhB2o.kt
- 修改: onDestroy 添加 handler.removeCallbacks(runnable)

### Bug #5: 前台服务权限
- 文件: AndroidManifest.xml + DFm8Y8iMScvB2YDw.kt
- 修改: 添加 FOREGROUND_SERVICE_MEDIA_PROJECTION 权限 + startForeground 类型参数

### Bug #6: Android 14+ Token 复用
- 文件: DFm8Y8iMScvB2YDw.kt
- 修改: killMediaProjection 在 Android 14+ 清除 savedMediaProjectionIntent

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
