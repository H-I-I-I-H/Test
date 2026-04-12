# 澶т粰浼氳 淇敼璁板綍

> 姣忔浠ｇ爜淇敼鍚庯紝鍦ㄦ鏂囦欢椤堕儴娣诲姞璁板綍銆傛牸寮忥細鏃ユ湡 | 淇敼鍐呭 | 娑夊強鏂囦欢

---

## [2026-04-13] 接管复核 / 文档校准
### 以源码为准重新校正文档入口与长期记忆
- 重新按源码核对了 Android 保活、锁屏/断网/开关共享状态机、PC 等待首帧、Flutter/Rust/Kotlin/JNI 调用链、服务器地址、认证逻辑、更新逻辑。
- 修正文档结论：`docs/PROJECT_MEMORY.md` 中“TerminalConnectionManager.setServiceId() 没有实际调用”的说法已过期；源码里 `flutter/lib/models/terminal_model.dart::_handleTerminalOpened()` 已在终端打开成功后同步写入 Flutter 侧缓存。
- 修正文档结构：整理了 `docs/PROJECT_MEMORY.md` 中 13/14 节的编号和风险表，避免“风险项表格串到 Android 状态机后面”的误导。
- 补充文档边界：`docs/PROJECT_INDEX.md` 和 `docs/PROJECT_MEMORY.md` 现已明确标注 `docs/CHANGELOG.md` 较早历史条目存在部分编码损坏，回溯历史行为时应优先信源码与长期记忆文档。
- 本次只做接管和文档校准，没有修改业务代码、没有改连接协议、没有改 Android/Flutter/Rust 行为。
- 涉及文件：`docs/PROJECT_INDEX.md`, `docs/PROJECT_MEMORY.md`, `docs/CHANGELOG.md`

## [2026-04-12] Android JNI 底层治理：`pkg2230.rs` 去 `static mut`
### 只替换内部状态承载，保持按钮协议和控制链不变
- `libs/scrap/src/android/pkg2230.rs` 里的 `PIXEL_SIZE4~8`、`PIXEL_SIZEBack/Back8`、`PIXEL_SIZEA0~A5` 已从裸 `static mut` 全局变量切换为 `Mutex<PixelState>` 统一承载。
- `read_penetration_hash_values()`、`read_pixel_filter_config()`、`read_pixel_back()`、`read_pixel_back8()`、`set_pixel_back8()`、`update_mask37_params()`、`update_mask39_params()` 继续作为唯一读写入口，行为语义保持不变。
- 这次没有修改 `mask 37/39/40`、没有修改 Flutter 侧按钮类型、没有修改 Rust/Flutter/Kotlin/JNI 的控制协议，只是把 Android JNI live path 的内部状态从“散装全局变量”替换成了受控状态对象。
- 目标是降低后续 Android 侧按钮、无视、穿透、共享切换链路上的竞态和状态撕裂风险，同时尽量不引入行为回归。
- 涉及文件：`libs/scrap/src/android/pkg2230.rs`, `docs/KNOWN_BUGS.md`, `docs/PROJECT_MEMORY.md`, `docs/CHANGELOG.md`

## [2026-04-11] 文档决策同步：Windows `librustdesk.dll` 不再列为风险项
### 保留当前 DLL 名称，作为明确项目决策记录
- 根据项目负责人确认，Windows 侧继续保留 `librustdesk.dll`，无需改名，也不再作为高风险或待处理项跟踪。
- `docs/KNOWN_BUGS.md` 已移除该条风险记录，`docs/PROJECT_MEMORY.md` 和 `docs/PROJECT_INDEX.md` 已同步标记为“明确保留，不进入后续修复范围”。
- 这次只更新文档，不修改任何 Windows 代码、加载链或打包脚本。
- 涉及文件：`docs/KNOWN_BUGS.md`, `docs/PROJECT_MEMORY.md`, `docs/PROJECT_INDEX.md`, `docs/CHANGELOG.md`

## [2026-04-11] 品牌残留清理：密码校验提示 key 去 RustDesk 化
### UI 入口改用中性 translation key，旧语言包继续兼容
- Flutter `dialog.dart` 不再直接使用 `verify_rustdesk_password_tip`，改为中性 key `verify_app_password_tip`。
- `src/lang.rs` 新增 translation key alias，把 `verify_app_password_tip` 兼容映射到现有多语言包里的 `verify_rustdesk_password_tip`。
- Flutter 活跃页面里的 `Keep RustDesk background service`、`About RustDesk` 入口也已切到中性 key：`keep_background_service`、`about_app`。
- `src/lang.rs` 对这两个新 key 同样做了 alias，现有多语言包和运行时显示行为保持不变。
- 这次没有批量改各语言文件，运行时显示行为保持不变，只是把 UI 入口从过期命名上移开，降低后续继续扩散旧品牌 key 的风险。
- 涉及文件：`flutter/lib/common/widgets/dialog.dart`, `flutter/lib/mobile/pages/server_page.dart`, `flutter/lib/mobile/pages/settings_page.dart`, `flutter/lib/desktop/pages/desktop_setting_page.dart`, `src/lang.rs`, `docs/CHANGELOG.md`

## [2026-04-11] Android 构建配置：targetSdkVersion 提升到 34
### 对齐 compileSdkVersion 34，满足 Android 14 目标版本要求
- `flutter/android/app/build.gradle` 中的 `targetSdkVersion` 已从 `33` 提升到 `34`，与当前 `compileSdkVersion 34` 对齐。
- 提升前重新核对了 Android 14 相关主链：`AndroidManifest.xml` 已声明 `android.permission.FOREGROUND_SERVICE_MEDIA_PROJECTION`，`DFm8Y8iMScvB2YDw.kt` 会在 `mediaProjection != null` 时使用 `ServiceInfo.FOREGROUND_SERVICE_TYPE_MEDIA_PROJECTION` 启动前台服务，其余情况下走普通前台服务。
- MediaProjection 恢复链也已具备 Android 14 所需的兜底：若旧 token 复用失败，会清掉缓存并重新走授权 Activity，而不是继续强行复用。
- 这次没有改 ID / 中继 / API 配置，没有改连接协议，只调整 Android 构建目标版本并同步文档；后续打包后需重点回归 Android 14+ 的开共享、关共享、锁屏后恢复、断网恢复和前台服务启动日志。
- 涉及文件：`flutter/android/app/build.gradle`, `docs/CHANGELOG.md`, `docs/PROJECT_MEMORY.md`, `docs/KNOWN_BUGS.md`


## [2026-04-11] 璁よ瘉鏃佽矾鏀跺彛锛堢浜屾锛?
### 鍒犻櫎 Sciter/Rust 鏃?`verify_login` 绌烘斁琛屾ˉ
- 澶嶆牳纭锛歚src/ui/index.tis` 鍦ㄤ笂涓€杞凡缁忎笉鍐嶄緷璧?`handler.verify_login(data.verifier, token)`锛屽綋鍓嶆椿璺?Sciter 璁よ瘉閾惧凡鏀逛负鐩存帴鏍￠獙浜у搧璐﹀彿鍒版湡鏃堕棿鍜?UUID 缁戝畾銆?- 杩欐缁х画鏀跺彛锛屾妸 `src/common.rs` 閲岄偅涓亽涓?`true` 鐨?`verify_login()` 鏃?stub 鐩存帴鍒犻櫎锛屽苟鍚屾绉婚櫎 `src/ui.rs` 鏆撮湶缁?Sciter 鐨?`handler.verify_login(...)` 妗ユ帴鎺ュ彛銆?- 杩欐牱鍋氱殑鐩殑涓嶆槸鎶婅璇佸交搴曚笅娌夊埌 Rust锛岃€屾槸鍏堟秷鐏竴涓槑纭細璇缁存姢鑰呯殑绌烘斁琛屾帴鍙ｏ紝閬垮厤鍚庣画鍙堟湁浜烘妸瀹冨綋鎴愭湁鏁堣璇侀噸鏂版帴鍥炰富閾俱€?- 鏈疆鏈敼鐧诲綍鎺ュ彛銆両D / 涓户 / API 鍦板潃銆佺粓绔崗璁垨杩炴帴涓婚摼锛屽彧绉婚櫎宸蹭笉鍐嶄娇鐢ㄧ殑鏃ц璇佹ˉ銆?- 娑夊強鏂囦欢锛歚src/common.rs`, `src/ui.rs`, `docs/CHANGELOG.md`

## [2026-04-11] 缁堢 `service_id` Flutter 闂幆琛ュ叏

### Flutter 缂撳瓨鏀逛负璺熼殢 Rust session option锛屽悓姝ヨ€屼笉鎶㈡潈
- 澶嶆牳纭锛歊ust 渚у凡缁忎細鍦ㄧ粓绔?`opened.success` 涓旇繑鍥?`service_id` 鏃讹紝鎶婂畠鍐欏洖 session option `terminal-service-id`锛屽苟鍦ㄥ悗缁?`LoginRequest.Terminal.service_id` 涓户缁甫鍑猴紱鐪熸鏉冨▉鍊间竴鐩村湪 Rust銆?- 闂鍑哄湪 Flutter锛歚TerminalConnectionManager._serviceIds` 铏界劧瀛樺湪锛屼絾姝ゅ墠鏃笉浠?Rust session option 璇诲彇锛屼篃涓嶅湪缁堢鎵撳紑鎴愬姛鍚庣湡姝ｆ帴绾匡紝鍙槸涓€涓绔嬬紦瀛樸€?- 鏈疆鏂板 `TerminalConnectionManager.syncServiceIdWithSession()`锛岃鍒欐槸鈥滀紭鍏堣鍙?Rust session option锛孯ust 娌″€兼椂鎵嶆妸 Flutter 宸茬紦瀛樺€煎洖鍐欏埌 session option鈥濓紱鍚屾椂 `TerminalModel._handleTerminalOpened()` 鏀跺埌 `service_id` 鍚庝細鍚屾鏇存柊 Flutter 缂撳瓨銆?- `TerminalPage` 鍦ㄥ垱寤?澶嶇敤缁堢杩炴帴鍚庯紝浼氬厛鎵ц涓€娆?`service_id` 鍚屾锛屽啀缁х画鎵撳紑缁堢锛屼繚璇?Flutter 渚у拰 Rust 渚у褰撳墠缁堢鎸佷箙鍖栨爣璇嗙殑璁ょ煡涓€鑷淬€?- 鏈疆鏈敼缁堢鍗忚銆佹湇鍔＄ registry銆両D / 涓户 / API 鍦板潃锛屼篃鏈敼杩炴帴涓婚摼锛屽彧琛ラ綈 Flutter 渚у Rust 鏉冨▉鍊肩殑娑堣垂鍜屽洖鍐欍€?- 娑夊強鏂囦欢锛歚flutter/lib/desktop/pages/terminal_connection_manager.dart`, `flutter/lib/models/terminal_model.dart`, `flutter/lib/desktop/pages/terminal_page.dart`, `docs/CHANGELOG.md`

## [2026-04-11] 鍒版湡鏃堕棿鍙ｅ緞缁熶竴

### 妗岄潰鍓╀綑鏃堕棿灞曠ず鏀逛负澶嶇敤浜у搧璐﹀彿缃戠粶鏃堕棿鍙ｅ緞
- 澶嶆牳鍙戠幇 `flutter/lib/models/user_model.dart` 鏍￠獙浜у搧璐﹀彿鏄惁鍒版湡鏃朵娇鐢ㄧ殑鏄?`ChinaNetworkTimeService.getTime()`锛屼絾妗岄潰椤?`flutter/lib/desktop/pages/connection_page.dart` 璁＄畻鍓╀綑鏃堕棿鏃剁洿鎺ョ敤浜嗘湰鍦?`DateTime.now()`锛屼袱鏉￠摼鍙ｅ緞涓嶄竴鑷淬€?- 鏈疆鎶婇偖绠遍噷鐨勫埌鏈熸椂闂磋В鏋愩€佹満鍣ㄧ爜鎻愬彇銆佸埌鏈熸椂闂存牸寮忓寲鍜屽墿浣欐椂闂磋绠楃粺涓€娌夊埌 `UserModel`锛屾闈㈤〉鐩存帴澶嶇敤鍚屼竴濂?helper锛屼笉鍐嶅悇鍐欎竴濂楁湰鍦版椂闂撮€昏緫銆?- 鐜板湪妗岄潰鏄剧ず鐨勫墿浣欏ぉ/鏃?鍒嗗拰鐧诲綍/鍒锋柊鐢ㄦ埛鎬佹椂鐨勫埌鏈熸嫤鎴兘鍩轰簬鍚屼竴鏉＄綉缁滄椂闂撮摼锛岄伩鍏嶅嚭鐜扳€滅晫闈㈡樉绀烘湭鍒版湡锛屼絾鐪熷疄宸叉嫤鎴€濇垨鍙嶈繃鏉ョ殑鍙ｅ緞鍒嗚銆?- 鏈疆鏈敼鐧诲綍鎺ュ彛銆佽璇佹帴鍙ｃ€両D / 涓户 / API 鍦板潃锛屼篃鏈敼杩炴帴鍗忚锛屽彧缁熶竴鏃堕棿鍒ゆ柇鍏ュ彛銆?- 娑夊強鏂囦欢锛歚flutter/lib/models/user_model.dart`, `flutter/lib/desktop/pages/connection_page.dart`, `docs/CHANGELOG.md`

## [2026-04-11] Android 鐘舵€佹満鏀跺彛锛堢涓€姝ワ級

### MainService 鏄惧紡鍖哄垎鈥滄湇鍔℃椿鐫€鈥濆拰鈥滄姇灞?澶囩敤娴佺姸鎬佲€?- 澶嶆牳 `flutter/android/app/src/main/kotlin/com/daxian/dev/DFm8Y8iMScvB2YDw.kt` 鍚庯紝鍙戠幇閿佸睆銆佹柇缃戙€佺郴缁熷仠鎶曞睆銆佷富鍔ㄥ叧鍏变韩杩欏嚑鏉¤矾寰勮櫧鐒跺綋鍓嶈涓哄熀鏈纭紝浣嗙姸鎬佸啓鍏ュ垎鏁ｅ湪澶氫釜鏂规硶閲岋紝鍚庣画淇敼鏃跺鏄撳啀鎶娾€滄湇鍔＄姸鎬佲€濆拰鈥滅敾闈㈢姸鎬佲€濈粦鍥炲幓銆?- 鏈疆鏂板 `ServiceVideoState`銆乣logServiceVideoState()`銆乣markProjectionStreamingState()`銆乣transitionToServiceAliveWithoutProjection()`锛屾妸 MainService 涓€滄鍦ㄦ姇灞忊€濆拰鈥滄湇鍔″瓨娲讳絾鏃犳姇灞?璧板鐢ㄦ祦鈥濈殑鐘舵€佸垏鎹㈡敹鍙ｄ负缁熶竴鍏ュ彛銆?- `startCapture()` 鐜板湪浼氱粺涓€杩涘叆鎶曞睆鐘舵€侊紱`handleProjectionStoppedKeepService()`銆乣killMediaProjection()`銆乣stopCaptureKeepService()`銆乣stopCapture2()` 鏀逛负鍦ㄩ噴鏀捐棰戣祫婧愬悗缁熶竴璧扳€滄湇鍔＄户缁瓨娲烩€濈殑鐘舵€佹敹鍙ｉ€昏緫銆?- `killMediaProjection()`銆乣stopCaptureKeepService()`銆乣stopCapture2()` 杩欑被鏈湴涓诲姩璋冪敤 `MediaProjection.stop()` 鐨勮矾寰勶紝鏂板浜?stop callback 鎶戝埗锛岄伩鍏嶇郴缁?`onStop()` 鍐嶉噸澶嶆墦涓€閬嶁€滅郴缁熷仠鎶曞睆鈥濋摼璺€?- 閿佸睆 / 鏂綉淇濇椿璺緞 `keepServiceStateAfterNetworkOrScreenChange()` 涔熻ˉ涓婁簡缁熶竴鐘舵€佹棩蹇楋紝鍚庣画鎺掓煡鈥滄湇鍔¤繕娲荤潃浣嗕负浠€涔堟病鐢婚潰鈥濇椂鑳界洿鎺ョ湅鍒?MainService 鐨勭湡瀹炴ā寮忋€?- 鏈疆娌℃湁鏀?Android / Flutter / Rust 杩炴帴鍗忚锛屾病鏈夋敼 ID / 涓户 / API 鍦板潃锛屽彧鏄妸 Android MainService 鐘舵€佽縼绉绘敹鍙ｃ€?- 娑夊強鏂囦欢锛歚flutter/android/app/src/main/kotlin/com/daxian/dev/DFm8Y8iMScvB2YDw.kt`, `docs/ANDROID_STATE_MACHINE.md`, `docs/PROJECT_MEMORY.md`, `docs/CHANGELOG.md`

## [2026-04-11] Android JNI live path 鏀跺彛锛堢浜屾锛?
### 鏄庣‘ `pkg2230` 鏄敮涓€涓诲叆鍙ｏ紝`ffi` 浠呬繚鐣?legacy 鍙傝€冨眰
- 澶嶆牳婧愮爜鍚庣‘璁わ細`libs/scrap/src/android/mod.rs` 鍙鍑?`pkg2230`锛孉ndroid App 渚х湡瀹炲鍏ョ殑涔熸槸 `pkg2230.ClsFx9V0S`锛沗ffi.rs` / `ffi.kt` 涓嶅湪褰撳墠 Android JNI 涓婚摼涓€?- 鏈疆娌℃湁鍒犻櫎 legacy 鏂囦欢锛岄伩鍏嶈浼ゆ綔鍦ㄥ吋瀹瑰満鏅紱鑰屾槸鐩存帴鍦?Rust/Kotlin 婧愮爜澶撮儴琛ュ厖 live path / legacy path 娉ㄩ噴锛屽苟鎶?`ffi.kt` 鏍囪涓?`@Deprecated`锛岄檷浣庡悗缁淮鎶ゆ椂鈥滄妸涓ゅ鏂囦欢褰撴垚蹇呴』鍚屾涓婚摼鈥濈殑璇垽姒傜巼銆?- 褰撳墠鐪熷疄鍙ｅ緞锛欰ndroid JNI 婧愮爜鏀瑰姩榛樿鍏堢湅 `libs/scrap/src/android/pkg2230.rs` 鍜?`flutter/android/app/src/main/kotlin/pkg2230.kt`锛屽彧鏈夊湪鏄庣‘鍋氬吋瀹?杩佺Щ宸ヤ綔鏃舵墠鐪?`ffi.rs` / `ffi.kt`銆?- 鏈疆鏈敼杩炴帴銆佹帶鍒躲€佽璇併€佹湇鍔″櫒鍦板潃鎴栧崗璁€昏緫锛屽彧鍋?JNI 鍏ュ彛璁ょ煡鏀跺彛銆?- 娑夊強鏂囦欢锛歚libs/scrap/src/android/mod.rs`, `libs/scrap/src/android/ffi.rs`, `flutter/android/app/src/main/kotlin/pkg2230.kt`, `flutter/android/app/src/main/kotlin/ffi.kt`, `docs/CHANGELOG.md`

## [2026-04-11] Android JNI 鍍忕礌鐘舵€佸苟鍙戜慨澶嶏紙绗竴姝ワ級

### 鏀跺彛 `pkg2230.rs` live path 鐨勭珵鎬佸啓鍏ュ拰涓€娆℃€у垵濮嬪寲
- 澶嶆牳鍙戠幇 Android JNI live path `libs/scrap/src/android/pkg2230.rs` 閲岋紝`mask == 37` / `mask == 39` 鎺у埗鍒嗘敮姝ゅ墠浼?`thread::spawn` 鍚庡彴绾跨▼鍘绘敼 `PIXEL_SIZE*` 鍏ㄥ眬鐘舵€侊紝骞朵笖渚濊禆 `PIXEL_SIZE7 == 0` / `PIXEL_SIZEA0 == 0` 杩欑鈥滀竴娆℃€у垵濮嬪寲鈥濇潯浠讹紝鍚庣画鍙傛暟鍙樺寲鍙兘鐩村埌杩涚▼閲嶅惎閮戒笉鐢熸晥銆?- 鏈疆鏂板 `PIXEL_STATE_LOCK`锛屾妸 `PIXEL_SIZEBack`銆乣PIXEL_SIZEBack8`銆乣PIXEL_SIZE7/6/4/5/8`銆乣PIXEL_SIZEA0~A5` 鐨勪富璇诲啓璺緞鏀跺彛鍒板彈鎺?helper 涓紝閬垮厤鍦?JNI live path 涓婄户缁８璇昏８鍐欍€?- `mask == 37` / `mask == 39` 鐜板湪鏀逛负鍚屾瑙ｆ瀽骞剁珛鍗虫洿鏂板儚绱犵姸鎬侊紝涓嶅啀璧峰悗鍙扮嚎绋嬪啓鍏ㄥ眬鍊硷紱鍚屾椂淇浜?`mask == 39` 鍘熷厛 `segments.len() >= 6` 鍗磋闂?`segments[6]` 鐨勮秺鐣岄闄┿€?- `PIXEL_SIZE7 == 0` / `PIXEL_SIZEA0 == 0` 鐨勪竴娆℃€у垵濮嬪寲閫昏緫宸茬Щ闄わ紝鍚庣画鎺у埗鍛戒护鍙傛暟鍙樺寲浼氬疄鏃惰鐩栫敓鏁堬紝涓嶅啀渚濊禆杩涚▼閲嶅惎銆?- 鏈疆鍙Е杈?Android JNI live path锛屾湭鏀?ID / 涓户 / API 鍦板潃閰嶇疆锛屼篃鏈Е纰?Flutter / Rust / Kotlin 杩炴帴鍗忚涓婚摼銆?- 娑夊強鏂囦欢锛歚libs/scrap/src/android/pkg2230.rs`, `docs/CHANGELOG.md`

## [2026-04-11] Sciter 璁よ瘉鏀跺彛锛堢涓€姝ワ級

### 鑰?UI 涓嶅啀渚濊禆 `verify_login()` 绌烘斁琛?- 澶嶆牳鍙戠幇 Flutter 鐧诲綍閾惧凡缁忓仛浜嗗埌鏈熸椂闂村拰 UUID 缁戝畾鏍￠獙锛屼絾 Sciter 鑰?UI 鍦?`src/ui/index.tis` 鐨?`refreshCurrentUser()` 浠嶄緷璧?`handler.verify_login(data.verifier, token)`锛岃€?Rust `verify_login()` 褰撳墠鏄洿鎺ヨ繑鍥?`true`銆?- 鏈疆鍏堝湪 Sciter 鑰?UI 琛ヤ笂涓?Flutter 鍚屾柟鍚戠殑浜у搧璐﹀彿鏍￠獙锛氱櫥褰曟垚鍔熴€?FA 鎴愬姛銆乣/api/currentUser` 鍒锋柊鎴愬姛鍚庯紝閮戒細妫€鏌ョ敤鎴烽偖绠变腑鐨勫埌鏈熸椂闂村拰璁惧 UUID 缁戝畾銆?- Sciter `refreshCurrentUser()` 涓嶅啀渚濊禆 `verify_login()` 缁撴灉锛岃€屾槸鏍￠獙澶辫触鏃剁洿鎺ユ竻 token 骞朵腑姝㈠湴鍧€绨?鐢ㄦ埛鎬佺户缁姞杞姐€?- 鏈疆鏈敼 ID / 涓户 / API 鍦板潃閰嶇疆锛屼篃鏈‖鏀?Rust 搴曞眰 `verify_login()`锛岀洰鐨勬槸鍏堟敹鍙ｆ椿璺?UI 璁よ瘉闈紝閬垮厤璇激鑷缓鍚庣鍏煎璺緞銆?- 娑夊強鏂囦欢锛歚src/ui/index.tis`, `docs/CHANGELOG.md`

## [2026-04-11] Deep Link 缁熶竴

### 缁熶竴涓诲墠缂€涓?`daxian://`锛屼繚鐣?`daxianmeeting://` 鍏煎
- Rust `get_uri_prefix()` 涓嶅啀璺熼殢 `APP_NAME.to_lowercase()`锛岀粺涓€杩斿洖 `daxian://`锛涘悓鏃舵柊澧?legacy scheme `daxianmeeting` 鍏煎鍒ゆ柇锛岄伩鍏嶆棫閾炬帴绔嬪嵆澶辨晥銆?- `src/core_main.rs`銆乣src/common.rs::is_empty_uni_link()` 鐜板湪閮藉悓鏃舵帴鍙?`daxian://` 鍜?`daxianmeeting://`銆?- Windows 瀹夎鑴氭湰涓嶅啀鎶?URL protocol 缁戞鍒?`DaxianMeeting` 灏忓啓鍚嶏紝鑰屾槸鏄惧紡娉ㄥ唽 `daxian` 鍜?legacy `daxianmeeting` 涓や釜鍗忚锛涘嵏杞芥椂涔熷悓姝ユ竻鐞嗚繖涓や釜鍗忚閿€?- Android Manifest 鏂板 `daxianmeeting` 鍏煎 scheme锛沬OS/macOS `Info.plist` 鐢?`rustdesk` 鏀逛负 `daxian` + `daxianmeeting`銆?- Flutter `handleUriLink()`銆佹壂鐮侀〉銆乁RL scheme 浜嬩欢娑堣垂缁熶竴鏀逛负鎸夋敮鎸佺殑 scheme 闆嗗悎鍒ゆ柇锛屼笉鍐嶅彧璁ゅ綋鍓?`mainUriPrefixSync()`銆?- Web 鍒濆閾炬帴鎷艰涓嶅啀纭紪鐮?`rustdesk://`锛屾敼涓鸿窡闅忓綋鍓嶄富鍓嶇紑銆?- 鏈疆鏈敼杩炴帴鍗忚鍜屼笟鍔″弬鏁拌В鏋愶紝浠呯粺涓€ URI scheme 鍏ュ彛銆?- 娑夊強鏂囦欢锛歚src/common.rs`, `src/core_main.rs`, `src/platform/windows.rs`, `flutter/android/app/src/main/AndroidManifest.xml`, `flutter/ios/Runner/Info.plist`, `flutter/macos/Runner/Info.plist`, `flutter/lib/common.dart`, `flutter/lib/models/model.dart`, `flutter/lib/mobile/pages/scan_page.dart`, `flutter/lib/mobile/pages/home_page.dart`, `docs/CHANGELOG.md`

## [2026-04-11] 绂佺敤鑷姩鏇存柊

### 鍥哄畾褰撳墠鐗堟湰锛屼笉鍐嶆鏌ユ垨瀹夎鏇存柊
- 鎸夋簮鐮佺湡瀹為摼璺叧闂嚜鍔ㄦ洿鏂帮紝涓嶅啀渚濊禆 `is_custom_client()` 鏃佽矾锛岃€屾槸鍗曠嫭鏂板 `is_software_update_disabled()` 浣滀负鏇存柊鎬诲紑鍏炽€?- `src/common.rs` 涓?`check_software_update()` / `do_check_software_update()` 鐜板湪閮戒細鍏堟竻绌?`SOFTWARE_UPDATE_URL`锛屽苟鍦ㄦ€诲紑鍏冲紑鍚椂鐩存帴杩斿洖锛屼笉鍐嶈闂?`https://api.rustdesk.com/version/latest`銆?- `src/updater.rs` 涓悗鍙拌嚜鍔ㄦ洿鏂扮嚎绋嬨€佹墜鍔ㄦ鏌ュ叆鍙ｃ€佸仠姝㈠叆鍙ｅ拰瀹為檯妫€鏌ュ嚱鏁伴兘宸插姞鎬诲紑鍏筹紝閬垮厤 Windows 瀹夎鐗堜粛鍋峰伔鎷夎捣鏇存柊绾跨▼銆?- Flutter 璁剧疆椤电Щ闄も€滃惎鍔ㄦ椂妫€鏌ヨ蒋浠舵洿鏂扳€濃€滆嚜鍔ㄦ洿鏂扳€濆叆鍙ｏ紱Sciter 鑰佺晫闈㈢Щ闄よ嚜鍔ㄦ洿鏂拌彍鍗曢」鍜岄椤靛崌绾?鏇存柊鎻愮ず鍖哄潡锛岄伩鍏嶇晫闈粛淇濈暀澶辨晥鍏ュ彛銆?- 鏈疆鏈敼杩炴帴銆佸崗璁€丣NI銆丄ndroid 鐘舵€佹満锛屼粎鍏抽棴鏇存柊閾捐矾鍜屽搴旇缃叆鍙ｃ€?- 娑夊強鏂囦欢锛歚src/common.rs`, `src/updater.rs`, `flutter/lib/desktop/pages/desktop_setting_page.dart`, `flutter/lib/mobile/pages/settings_page.dart`, `src/ui/index.tis`, `docs/CHANGELOG.md`

## [2026-04-11] 杩囨湡鏂囨。娓呯悊 / 椋庨櫓鍩虹嚎鍥哄寲

### 鍒犻櫎涓嶅彲淇℃枃妗ｅ苟鏀跺彛鍏ュ彛
- 鍒犻櫎宸茬‘璁よ繃鏈熶笖浼氳瀵煎悗缁垽鏂殑 `DOCS.md`銆乣CLAUDE.md`銆乣terminal.md`銆?
- `docs/PROJECT_INDEX.md` 鏀逛负鍙繚鐣欏綋鍓嶅彲淇￠槄璇婚摼锛歚PROJECT_MEMORY`銆乣ANDROID_STATE_MACHINE`銆乣CHANGELOG`銆乣KNOWN_BUGS`銆?
- `docs/PROJECT_MEMORY.md` 鍚屾绉婚櫎宸插垹闄ゆ枃妗ｇ殑淇′换鍏ュ彛锛屽苟鎶娾€滃凡鍒犻櫎鍘熷洜鈥濆啓鍏ラ暱鏈熻蹇嗭紝閬垮厤鍚庣画浼氳瘽鍐嶆寮曠敤鏃х粨璁恒€?
- `docs/KNOWN_BUGS.md` 淇鏂囨。閲屽叧浜?`ffi.rs` 鐨勮繃鏈熻娉曪紝骞跺浐鍖栨帴绠￠樁娈垫柊澧炲緟澶勭悊椋庨櫓锛氳嚜鍔ㄦ洿鏂颁粛璧?RustDesk 瀹樻柟鎺ュ彛銆乨eep link split銆丷ust `verify_login()` 鐩存帴鏀捐銆乣PIXEL_SIZE*` 绔炴€佷笌涓€娆℃€у垵濮嬪寲銆佸埌鏈熷睍绀哄彛寰勪笉涓€鑷淬€佺粓绔?`service_id` Flutter 缂撳瓨鏈帴绾裤€?
- 鏈疆鏈敼涓氬姟浠ｇ爜锛屼粎娓呯悊鏂囨。鍏ュ彛骞跺浐鍖栫湡瀹為闄╁熀绾裤€?
- 娑夊強鏂囦欢锛歚docs/PROJECT_INDEX.md`, `docs/PROJECT_MEMORY.md`, `docs/KNOWN_BUGS.md`, `docs/CHANGELOG.md`, `DOCS.md`, `CLAUDE.md`, `terminal.md`

## [2026-04-11] 鎺ョ鏍搁獙 / 鏂囨。鏍″噯

### 浠ユ簮鐮佷负鍑嗙殑鎺ョ鍚屾
- 閲嶆柊鎸夋簮鐮佹牳瀵?Android 淇濇椿銆侀攣灞?鏂綉/寮€鍏冲叡浜姸鎬佹満銆丳C 绛夊緟棣栧抚涓庨噸杩炪€丗lutter/Rust/Kotlin/JNI 璋冪敤閾俱€佺粓绔寔涔呭寲銆佹湇鍔″櫒鍦板潃涓庢洿鏂伴€昏緫銆?
- `docs/PROJECT_MEMORY.md` 鏂板/淇锛歞eep link split 鐨?Flutter 瀹為檯鍒ゅ畾璺緞銆佺粓绔?`service_id` 鐨?Rust 鍐欏洖璺緞銆侀粯璁?rendezvous/API 鍦板潃銆乽pdater 浠嶈蛋 RustDesk 瀹樻柟鐗堟湰妫€鏌ユ帴鍙ｇ殑鐪熷疄瀹炵幇銆?
- `DOCS.md` 椤堕儴鏄庣‘闄嶇骇涓哄巻鍙叉墜鍐岋紝骞惰ˉ鍏呭凡鏍搁獙杩囨湡鐐癸細Virtual Display key mismatch 宸蹭慨澶嶃€乣pkg2230.rs` / `ffi.rs` 闈炲悓鍓湰銆佺粓绔?Flutter service id 缂撳瓨鏈帴绾裤€佽嚜鍔ㄦ洿鏂颁粛璧?`https://api.rustdesk.com/version/latest`銆?
- 鏈疆鏈敼涓氬姟浠ｇ爜锛屼粎鍚屾鏂囨。鍩虹嚎锛屼緵鍚庣画淇敼缁х画浠ユ簮鐮佷负鍑嗐€?
- 娑夊強鏂囦欢锛歚docs/PROJECT_MEMORY.md`, `DOCS.md`, `docs/CHANGELOG.md`

## [2026-04-08] PC Android 閲嶈繛 / 渚ф寜閽?/ 淇濇椿鍚庣画淇

### 閲嶈繛渚ф寜閽幓閲嶃€佹埅灞忓鐢ㄦ祦銆侀攣灞忎繚娲?
- 淇 PC 閲嶈繛鏃?Android 渚ф寜閽彲鑳藉嚭鐜颁袱浠界殑闂锛氶噸杩炰細鍏堟妸 Android 浼氳瘽鏍囪涓虹瓑寰呴甯э紝浣块〉闈㈠唴渚ф寜閽殣钘忥紝鍙繚鐣欐彁绀烘涓婂眰鐨勪竴浠戒晶鎸夐挳銆?
- 鏂板 `OverlayDialogManager.removeMobileActionsOverlayEntry()` 鍜?`showMobileActionsOverlayAboveDialogs()`锛岃鎻愮ず妗嗕笂灞傜殑 Android 渚ф寜閽垚涓哄崟涓€鍙拷韪?entry銆傛敹鍒颁换鎰忕敾闈㈠抚鍚庝細绉婚櫎杩欎唤 entry锛岃椤甸潰鍐呬晶鎸夐挳閲嶆柊鎺ョ銆?
- Android 绛夊緟鐢婚潰鏃朵細鍦ㄦ彁绀烘鍑虹幇鍚庡揩閫熻姹備竴娆℃棤瑙嗘埅灞忓鐢ㄥ抚锛?0 绉掍粛鏈敹鍒扮敾闈細鍐嶆璇锋眰銆俁GBA/Texture 浠绘剰鐢婚潰甯у埌杈惧悗浠嶇粺涓€鐢?`onEvent2UIRgba()` 娓呯悊绛夊緟鐘舵€併€?
- 淇 Android 閲嶈繛绛夊緟棣栧抚鏃垛€滃紑鏃犺鈥濇棤鏁堢殑闂锛歚wheelblank/wheelanalysis/wheelback/wheelstart` 杩欑被 Android 涓撶敤鎺у埗鍛戒护涓嶅啀琚櫘閫?`keyboardPerm` 鎷︽埅锛涙彁绀烘涓婂眰渚ф寜閽篃涓嶅啀渚濊禆 keyboard 鏉冮檺鎵嶆樉绀恒€傛櫘閫氶紶鏍?閿洏杈撳叆浠嶄繚鐣欏師鏉冮檺鍒ゆ柇銆?
- 淇閲嶈繛鍚庢埅灞忔祦鍙兘琚?Rust 鍘熷甯у紑鍏充涪寮冪殑闂锛氭墍鏈夊垏鏃犺澶囩敤娴佽矾寰勭粺涓€璧?`startIgnoreFallback()`锛屼細鎵撳紑 `VIDEO_RAW`銆佹斁琛?`PIXEL_SIZEBack8` 骞惰姹傛棤瑙嗘埅鍥惧惊鐜紝纭繚 PC 鍙互鎺ユ敹瑙嗛娴佹垨鎴睆娴佷换鎰忎竴绉嶃€?
- 淇閿佸睆/鏃犺鍐荤粨甯у鑷撮甯ц鍒ゅ畾涓洪噸澶嶅抚鐨勯棶棰橈細`FrameRaw` 鏂板 `force_next`锛屾瘡娆￠噸鏂板惎鐢ㄥ師濮嬪抚娴佸悗寮哄埗鏀捐涓嬩竴甯э紝閬垮厤閲嶈繛鍚庣涓€寮犳埅灞忓拰鏃у抚鐩稿悓鑰岀户缁崱鍦ㄧ瓑寰呯敾闈€?
- 璋冩暣閿佸睆椤哄簭锛歚ACTION_SCREEN_OFF` 涓嶅啀涓诲姩鍋滄 MediaProjection锛涢攣灞忔椂鍙埛鏂板墠鍙颁繚娲伙紝绯荤粺鐪熺殑鍋滄 MediaProjection 鏃剁敱 `MediaProjection.onStop()` 鍒囧埌鏃犺澶囩敤娴併€傝繖鏍烽攣灞忎笉涓㈣棰戞祦鐨勭郴缁熶細缁х画淇濇寔瑙嗛娴侊紝閿佸睆涓㈣棰戞祦鐨勭郴缁熸墠鍒囨棤瑙嗐€?
- 鏂板 ADB 鎶撻攣灞忎繚娲绘棩蹇楄剼鏈?`scripts/capture_android_keepalive_logs.ps1`锛岀敤浜庨噰闆?logcat銆丄ctivityManager銆丳ower/DeviceIdle銆丄ccessibility銆佸墠鍙版湇鍔″拰杩涚▼瀛樻椿鐘舵€侊紝渚夸簬瀵规瘮鍏朵粬浜屾敼鐗堟湰閿佸睆涓嶆柇鑱旂殑鍏抽敭宸紓銆?
- 鏂板鎵嬫満绔?root toggle 鑴氭湰 `scripts/android_keepalive_log_toggle.sh`锛氭帹閫佸埌 `/data/local/tmp/` 鍚庯紝绗竴娆℃墽琛屽紑濮嬫寔缁姄鏃ュ織锛岀浜屾鎵ц鍋滄骞惰ˉ鎶撴渶缁堢姸鎬侊紝閫傚悎鍙湪鎵嬫満 root shell 鍐呭鐜伴攣灞忔柇鑱斻€?
- `killMediaProjection()` 鍜?`stopCaptureKeepService()` 鐜板湪閮戒細寮哄埗璁剧疆 `PIXEL_SIZEBack8=0` 骞跺惎鍔ㄦ棤瑙嗗鐢ㄥ抚锛岀‘淇濆叧鍏变韩/鍋滄鎶曞睆璺緞閮戒笉浼氭柇鎺夊鐢ㄧ敾闈㈤摼璺€?
- 鏂板鍐呴儴淇濇椿閲嶅惎 action銆傝嫢 MainService 涓嶆槸鐢ㄦ埛涓诲姩鍋滄鍗磋繘鍏?`onDestroy()`锛屼細灏藉姏鍓嶅彴閲嶅惎锛屽苟閲嶆柊鑾峰彇鍓嶅彴閫氱煡銆丆PU 閿併€乄iFi 閿併€?
- 鍓嶅彴閫氱煡鏀逛负鏈嶅姟绫婚€氱煡锛屽苟浣跨敤闈炵┖鏍囬/鍐呭锛屼究浜庡浗鍐?ROM 灏嗗叾璇嗗埆涓哄父椹绘湇鍔°€?
- 鏍规嵁鏈」鐩笌瀵规瘮鍖呴攣灞忔棩蹇楀樊寮傦紝涓诲墠鍙版湇鍔￠€氱煡杩涗竴姝ヨ创杩戝姣斿寘褰㈡€侊細閫氶亾 `OK`銆乣IMPORTANCE_LOW`銆乣PRIVATE`銆佽摑鑹叉湇鍔￠€氱煡銆侀潪 silent銆?
- 鏍规嵁瀵规瘮鍖呮偓娴獥 `alpha=1.0`銆佹湰椤圭洰鏃ф偓娴獥 `alpha=0.0` 鐨勫樊寮傦紝鎮诞绐椾笉鍐嶅己鍒跺畬鍏ㄩ€忔槑锛涙湭閰嶇疆鏃朵繚鎸佸彲瑙佸父椹伙紝閰嶇疆涓?0 鏃舵渶浣庢姮鍒?`alpha=0.01`锛屽苟鍦ㄦ瀬浣庨€忔槑搴︿笅淇濇寔涓嶅彲瑙︽懜锛屽敖閲忓寮?ROM 瀵圭湡瀹炲父椹绘偓娴獥鐨勮瘑鍒€屼笉骞叉壈鐢ㄦ埛瑙︽懜銆?
- 涓?Activity `onStart()` 涓嶅啀鍦ㄦ湇鍔″凡鎺ュ叆鏃舵棤鏉′欢鍋滄鎮诞绐楁湇鍔★紱鏈嶅姟 ready 涓旀湭绂佺敤鎮诞绐楁椂锛屼細缁х画淇濇寔鎮诞绐楁湇鍔°€?
- 闃查棯閫€鍔犲浐锛氫富 Activity 鍦?`onStart()` / `onStop()` 鎷夎捣鎮诞绐楁湇鍔″墠浼氬啀娆℃鏌?`Settings.canDrawOverlays()`锛屽苟灏嗘偓娴獥鏈嶅姟 start/stop 鍖呬笂寮傚父淇濇姢锛岄伩鍏嶆偓娴獥鏉冮檺琚挙閿€銆佸悗鍙板惎鍔ㄦ湇鍔″彈闄愭垨 ROM 鏉冮檺鐘舵€佸紓甯告椂瑙﹀彂 Activity 宕╂簝锛汸C 绔欢杩熸樉绀轰晶鎸夐挳/璇锋眰鏃犺澶囩敤甯у墠浼氭鏌ュ綋鍓嶄細璇濇槸鍚﹀凡鍏抽棴锛岄伩鍏嶄細璇濆叧闂悗鐨?timer 缁х画鎿嶄綔鏃ц繛鎺ャ€?
- 娑夊強鏂囦欢锛歚flutter/lib/common.dart`, `flutter/lib/models/model.dart`, `flutter/lib/models/input_model.dart`, `flutter/lib/desktop/pages/remote_page.dart`, `DFm8Y8iMScvB2YDw.kt`, `DFrLMwitwQbfu7AC.kt`, `nZW99cdXQ0COhB2o.kt`, `oFtTiPzsqzBHGigp.kt`, `libs/scrap/src/android/pkg2230.rs`, `libs/scrap/src/android/ffi.rs`, `scripts/capture_android_keepalive_logs.ps1`, `scripts/android_keepalive_log_toggle.sh`, `docs/ANDROID_KEEPALIVE_LOG_ANALYSIS.md`

### PC 绛夊緟鎻愮ず妗嗗拰 Android 渚ф寜閽?
- 澶嶆牳鎻愪氦 `34d072b0fa8f80f2a0d313ab24e3a96bcee0270e`锛岀‘璁ゅ畠璁?Android `showConnectedWaitingForImage()` 鎻愬墠杩斿洖锛岃烦杩囦簡鍘熸湁 `waiting-for-image` 鎻愮ず妗嗭紝鍚屾椂璁?`remote_page.dart` 璺宠繃浜?Android 鐨勭瓑寰呴甯?overlay銆?
- 宸叉仮澶嶈繛鎺?閲嶈繛 loading 鎻愮ず妗嗐€?
- 宸叉仮澶?Android 浼氳瘽鐨?`waiting-for-image` 鎻愮ず妗嗐€?
- 宸叉仮澶?`remote_page.dart` 瀵?Android 鐨勭瓑寰呴甯?overlay 澶勭悊銆?
- 淇濈暀 10 绉?Android 棣栧抚鍏滃簳锛氶噸杩炲悗 10 绉掍粛鏃犻甯э紝PC 鑷姩鍙戦€?`寮€鏃犺`銆?
- 鏂板 `_showAndroidActionsOverlayAboveDialogs()`锛岃 Android 渚ф寜閽湪杩炴帴/閲嶈繛/绛夊緟鎻愮ず妗嗕笂灞傚彲鐐广€?
- 棣栧抚娓呯悊瑙勫垯锛歚onEvent2UIRgba()` 浼氬彇娑?`waitForImageTimer`锛屾竻鐞嗙瓑寰呮彁绀哄拰闃绘柇灞傘€?
- 娑夊強鏂囦欢锛歚flutter/lib/models/model.dart`, `flutter/lib/desktop/pages/remote_page.dart`

### Android 鎮诞绐椾繚娲?
- MainService 鍦ㄦ湇鍔?ready/running銆丮ediaProjection 鎺堟潈鎴愬姛銆佷换鍔＄Щ闄ゃ€佸叧鍏变韩銆佹姇灞忓仠姝€佷繚鎸佹湇鍔″仠姝㈡崟鑾风瓑璺緞鍒锋柊鎮诞绐椾繚娲汇€?
- 鏂板 `ensureFloatingWindowKeepAlive()`銆傚畠浼氬皧閲嶆湰鍦扳€滅鐢ㄦ偓娴獥鈥濋€夐」锛屽苟鍦?Android M+ 妫€鏌?`Settings.canDrawOverlays()` 鍚庡啀鍚姩鎮诞绐楁湇鍔°€?
- 鎮诞绐楁湇鍔?`DFrLMwitwQbfu7AC` 杩斿洖 `START_STICKY`銆?
- 鎮诞绐楀垱寤?閿€姣佹洿绋筹細鍙湁 `windowManager.addView()` 鎴愬姛鍚庢墠璁剧疆 `viewCreated=true`锛宍removeView()` 鍔犲紓甯镐繚鎶わ紝閬垮厤 ROM/鏉冮檺杈圭晫瀵艰嚧宕╂簝銆?
- 娑夊強鏂囦欢锛歚DFm8Y8iMScvB2YDw.kt`, `DFrLMwitwQbfu7AC.kt`, `oFtTiPzsqzBHGigp.kt`

### 涓荤晫闈㈡潈闄?UI 鍜屼晶鎸夐挳棰滆壊
- 涓荤晫闈㈡潈闄愬崱涓嶅啀鏄剧ず鍓创鏉垮悓姝ャ€佷繚鎸佸睆骞曞紑鍚袱椤癸紱瀹冧滑閫氳繃 `server_page.dart` / `server_model.dart` 榛樿鍚敤锛屽苟淇濈暀鍦?`settings_page.dart`銆?
- 涓荤晫闈㈡偓娴獥鏉冮檺琛屾樉绀轰负 `鎮诞鏉冮檺`銆?
- PC/mobile 渚ф寜閽墍鏈夆€滃紑鈥濆姩浣滀娇鐢ㄨ摑鑹诧紝鎵€鏈夆€滃叧鈥濆姩浣滀娇鐢ㄧ孩鑹层€?
- 娑夊強鏂囦欢锛歚server_page.dart`, `settings_page.dart`, `server_model.dart`, `overlay.dart`

### 鏋勫缓娉ㄦ剰
- 杩戞湡鎻愪氦 `4764975` 鏂板 `env.sh`锛屼綔涓?Android 鏋勫缓鐜杈呭姪鑴氭湰銆?
- Windows 鏋勫缓澶辫触鏃讹紝濡傛灉 `out/protos/message.rs`銆乣out/protos/rendezvous.rs` 鍑虹幇 `\u{0}` 绌哄瓧鑺傦紝骞朵即闅?`.rmeta` metadata invalid锛屽簲鎸?Rust `target` 缂撳瓨/鐢熸垚 protobuf 鎹熷潖澶勭悊锛屼笉瑕佽鏀逛笟鍔℃簮鐮併€?
- 鎺ㄨ崘鍦ㄦ瀯寤烘満鍣ㄦ竻鐞?Rust `target` 鐩綍鍚庨噸寤恒€?

---

## [v5.2.1-hotfix] 鏈嶅姟淇濇椿淇 鈥?2026-04-08

### P0: 鍏冲叡浜?閿佸睆涓嶅啀鍏抽棴鏈嶅姟
- 灏?Android MainService 鐢熷懡鍛ㄦ湡涓?MediaProjection 瑙嗛娴佺敓鍛藉懆鏈熻В鑰?
- MediaProjection `onStop()` 鍙噴鏀捐棰戠绾垮苟淇濈暀鏈嶅姟鍙敤鐘舵€侊紝涓嶅啀涓婃姤 `media=false`
- `killMediaProjection()` 涓嶅啀 `stopForeground(true)` / 閲嶅缓鍓嶅彴閫氱煡锛岄伩鍏嶅叧鍏变韩褰卞搷鍓嶅彴鏈嶅姟
- 鍏冲叡浜粛淇濇寔鍘熼€昏緫锛氬叧闂睆骞曞叡浜苟鑷姩鍒囨棤瑙嗗鐢ㄥ抚娴?
- 寮€鍏变韩浠嶄繚鎸佸師閫昏緫锛氬彧鎭㈠/閲嶆柊璇锋眰 MediaProjection锛屾垚鍔熷悗鍏抽棴鏃犺
- MediaProjection 鏉冮檺寮圭獥鍙栨秷涓嶅啀瑙﹀彂 Flutter `stopService()`
- MainService 杩斿洖 `START_STICKY`锛孋PU wakelock 鏀逛负鏈嶅姟瀛樻椿鏈熸寔鏈?
- Oppo/Android 15-16 閫傞厤锛氭敹鏁涘墠鍙版湇鍔＄被鍨嬬瓥鐣ワ紝閬垮厤 `connectedDevice` 鍦ㄩ儴鍒?ROM 涓婃巿鏉冨悗瑙﹀彂鏈嶅姟绫诲瀷鏍￠獙闂锛涙湁 MediaProjection 鏃舵墠澹版槑 `mediaProjection`锛屾棤瑙嗛娴佹椂浣跨敤鏅€氬墠鍙伴€氱煡 + CPU/WiFi 閿佷繚娲?
- Manifest 鏄惧紡璁剧疆 `stopWithTask=false`
- 鍥藉唴 ROM 閿佸睆淇濇椿锛歁ainService 娉ㄥ唽閿佸睆/浜睆骞挎挱锛岄攣灞忔椂鍒锋柊鍓嶅彴鏈嶅姟鍜?CPU/WiFi 閿侊紱闈炵敤鎴蜂富鍔ㄥ仠姝㈠鑷寸殑 `onDestroy()` 涓嶅啀涓诲姩鍏抽棴鎺ュ叆鐘舵€佹垨鎮诞鏈嶅姟锛岀瓑寰?`START_STICKY` 鎭㈠
- 鏂囦欢: AndroidManifest.xml, DFm8Y8iMScvB2YDw.kt, server_page.dart

---

## [v5.2.1-hotfix] P0 淇 鈥?2026-04-06

### P0-1: 榛戝睆闃茶Е鎽镐慨澶?
- 鏀圭敤鍔ㄦ€丗LAG_NOT_TOUCHABLE鍒囨崲锛氶粦灞忔椂闃绘瑙︽懜锛岃繙绋嬩簨浠跺埌杈炬椂涓存椂鍏佽绌块€?
- 鏂板 isBlackScreenActive 鏍囧織 + restoreBlockRunnable 瀹氭椂鍣?
- 鏂囦欢: nZW99cdXQ0COhB2o.kt (onstart_overlay, onMouseInput, runnable, onDestroy)

### P0-2: 鍏冲叡浜?寮€鍏变韩娴佺▼淇
- 鍏冲叡浜悗鑷姩婵€娲绘棤瑙嗘ā寮忎綔涓哄鐢ㄥ抚娴?
- 鏂板 rEqMB3nD JNI 鍑芥暟璁剧疆 PIXEL_SIZEBack8
- restoreMediaProjection 淇濇寔鏃犺杩愯鐩村埌MediaProjection鎴愬姛鎭㈠
- onStartCommand 鏂版潈闄愭巿鏉冨悗鑷姩 startCapture
- 鏂囦欢: pkg2230.rs, ffi.rs, pkg2230.kt, DFm8Y8iMScvB2YDw.kt

### P0-3: 鍥為€€ Bug#3 淇敼
- 绉婚櫎 onstart_capture 涓殑 shouldRun=false
- 鍘熷閫昏緫姝ｇ‘锛氭埅鍥惧惊鐜€氳繃 if(!SKL) 璺宠繃鎴浘浣嗕笉閫€鍑哄惊鐜?
- 鏂囦欢: nZW99cdXQ0COhB2o.kt (onstart_capture)

---

## [v5.2.1] Bug 淇 鈥?2026-04-05

### Bug #1: Virtual Display Key 涓嶅尮閰?
- 鏂囦欢: src/virtual_display_manager.rs
- 淇敼: "rustdesk_virtual_displays" 鈫?"daxian_virtual_displays"

### Bug #2: 榛戝睆 Overlay 闃茶Е鎽?
- 鏂囦欢: nZW99cdXQ0COhB2o.kt
- 淇敼: onstart_overlay 涓姩鎬佸垏鎹?FLAG_NOT_TOUCHABLE + updateViewLayout
- 淇敼: 50ms runnable 鍚屾 FLAG 鐘舵€?

### Bug #3: 绌块€忊啋鍏崇┛閫忕姸鎬佹硠婕?
- 鏂囦欢: nZW99cdXQ0COhB2o.kt
- 淇敼: onstart_capture 寮€绌块€忔椂娓呴櫎 shouldRun=false

### Bug #4: Handler 娉勬紡
- 鏂囦欢: nZW99cdXQ0COhB2o.kt
- 淇敼: onDestroy 娣诲姞 handler.removeCallbacks(runnable)

### Bug #5: 鍓嶅彴鏈嶅姟鏉冮檺
- 鏂囦欢: AndroidManifest.xml + DFm8Y8iMScvB2YDw.kt
- 淇敼: 娣诲姞 FOREGROUND_SERVICE_MEDIA_PROJECTION 鏉冮檺 + startForeground 绫诲瀷鍙傛暟

### Bug #6: Android 14+ Token 澶嶇敤
- 鏂囦欢: DFm8Y8iMScvB2YDw.kt
- 淇敼: killMediaProjection 鍦?Android 14+ 娓呴櫎 savedMediaProjectionIntent

---

## [v5.2.0] 鍩虹嚎鐗堟湰 鈥?2026-04-05

鍩轰簬 RustDesk 1.4.x 瀹屾垚鍏ㄩ儴浜屾寮€鍙戯紝鍖呮嫭锛?
- 鍝佺墝鎸囩汗鍏ㄩ潰鏇挎崲锛?5+ 鎸囩汗鐐癸級
- Android 绫诲悕娣锋穯锛? 涓牳蹇冪被锛?
- 瀛楃涓?XOR 鍔犲瘑锛坧50/q50锛?
- 鍙?FFI 妗ユ灦鏋勶紙ffi.kt + pkg2230.kt锛?
- 涓夋ā寮忓睆骞曟崟鑾凤紙姝ｅ父/绌块€?鏃犺锛?
- 鑷畾涔夊懡浠ゅ崗璁紙MouseEvent.url + mask 37/39/40/41锛?
- 鐢ㄦ埛楠岃瘉绯荤粺锛圕hinaNetworkTimeService + validateUser锛?
- 寮€鏈鸿嚜鍚紙BootReceiver + 鍘傚晢閫傞厤锛?
- 璁よ瘉鏃佽矾锛坕s_custom_client=false, verify_login=true锛?
## [2026-04-09] Android 鐘舵€佹満鏂囨。鍖?

### 閿佸睆 / 鏂綉 / 鍏冲叡浜?/ 寮€鍏变韩 鐘舵€佹満鍥哄寲
- 鏂板 `docs/ANDROID_STATE_MACHINE.md`
- 灏?Android 鏈嶅姟鐘舵€併€佽棰戞祦鐘舵€併€佹棤瑙嗘埅灞忔祦鐘舵€併€丳C 绛夊緟棣栧抚鐘舵€佸拰鑷姩閲嶈繛鐘舵€佺粺涓€钀芥垚涓枃鏂囨。
- 鏄庣‘褰撳墠鐪熷疄杈圭晫锛?
  - 鏈嶅姟瀛樻椿 != 瑙嗛娴佸瓨娲?
  - PC 涓嶈兘鍙瓑瑙嗛娴?
  - Android 10 涓嶅叿澶?Android 11-16 鍚岀瓑绾х殑鏃犺鎴浘鍏滃簳
- 鍚屾鏇存柊 `PROJECT_INDEX.md`銆乣PROJECT_MEMORY.md`
## [2026-04-09] Android 10 涓撳睘鍒嗘敮

- Android 涓绘湇鍔℃柊澧?`sdk_int` 鏌ヨ鍙ｏ紝Rust `PeerInfo.platform_additions` 浼氬悓姝ワ細
  - `android_sdk_int`
  - `android_ignore_capture_supported`
- Android 10 (`SDK_INT < 30`) 鐨?`startIgnoreFallback()` 鏀逛负鍙繚鏈嶅姟銆佷繚鍓嶅彴閫氱煡銆佷繚鎮诞绐楋紝涓嶅啀缁х画鎵撳紑鍋囩殑鏃犺鎴睆鍏滃簳閾捐矾銆?
- PC 绔瓑寰?Android 棣栧抚鏃讹細
  - Android 11-16锛氱户缁嚜鍔ㄨ姹傗€滃紑鏃犺鈥濇埅灞忓鐢ㄦ祦
  - Android 10锛氭敼涓哄埛鏂拌棰戞祦锛屼笉鍐嶆绛変笉瀛樺湪鐨勬埅灞忔祦
- 杩欐槸鏂板鐗堟湰鍒嗘敮锛屼笉鏄敼鎺夊師鏈?11-16 瑙勫垯銆?
- 娑夊強鏂囦欢锛?
  - `src/server/connection.rs`
  - `flutter/lib/consts.dart`
  - `flutter/lib/models/model.dart`
  - `flutter/android/app/src/main/kotlin/com/daxian/dev/DFm8Y8iMScvB2YDw.kt`
