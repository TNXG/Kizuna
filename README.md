# Kizuna

Kizuna 是一个基于 Tauri 的桌面应用程序，使用 Rust + Vue 3 开发。该应用程序可以监视当前前台活跃窗口的信息、系统的SMTC信息、窗口图标，并将上传到服务器。

实现方式照抄自 [TNXG/ProcessReporterWingo](https://github.com/TNXG/ProcessReporterWingo)，或许说就是其的Rust实现（附带了ui）


## 技术栈
- **前端**：
  - Rust
  - Vue 3
  - Tauri
  - Vite
  - TailwindCSS
  - DaisyUI

## 使用说明
1. **配置文件**：
   - 编辑 `config.yml` 文件，设置服务器端点和令牌。
```yaml
server_config:
  endpoint: "apiurl" # https://api.example.com/api/v2/fn/ps/update
  token: "apikey" # 设置的key
  report_time: 5 # 上报时间间隔，单位秒
rules: # 软件名的替换规则
  - match_application: WeChat
    replace:
      application: 微信
      description: 一个小而美的办公软件
  - match_application: QQ
    replace:
      application: QQ
      description: 一个多功能的通讯软件
  - match_application: Netease Cloud Music
    replace:
      application: 网易云音乐
      description: 一个音乐播放和分享的平台
```
2. **日志查看**：
   - 日志文件存储在 `logs` 目录下，每天生成一个日志文件。

3. **图标转换**：
   - 应用会获取当前窗口的图标，但是暂且未实现上传逻辑


## 其他问题

### Q：网易云音乐不能上报

A：网易云音乐不按照微软官方的媒体渠道上报媒体信息（即 Windows system media Transport control 集成）

`从 Windows 10 版本 1607 开始，默认情况下，使用 MediaPlayer 类或 AudioGraph 类播放媒体的 UWP 应用会自动与 SMTC 集成。 只需实例化 MediaPlayer 的新实例，并将 MediaSource、MediaPlaybackItem 或 MediaPlaybackList 分配给玩家的 Source 属性，然后用户将在 SMTC 中看到你的应用名称，并且可以使用 SMTC 控件播放、暂停和在播放列表中移动。  -- Windows文档`

这时需要其他方法来使本程序的media上报结构生效
- 通过插件使其通过SMTC上报信息
    - 网易云音乐：[MicroCBer/BetterNCM](https://github.com/MicroCBer/BetterNCM) 和 [BetterNCM/InfinityLink](https://github.com/BetterNCM/InfinityLink) 搭配使用
- Pr Welcome

## 推荐的IDE设置

- [VS Code](https://code.visualstudio.com/) + [Volar](https://marketplace.visualstudio.com/items?itemName=Vue.volar) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## 联系我们

- 个人博客: [tnxgmoe.com](https://tnxgmoe.com/about-me#:re:%E8%81%94%E7%B3%BB%E6%96%B9%E5%BC%8F)

2024 © TNXG 本项目遵循 AGPL 3.0 license 开源

本 README 部分内容由 Kimi.ai 生成，请注意甄别内容。