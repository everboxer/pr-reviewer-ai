
<div align="right">

[English](README.md)

</div>

# <p align="center">ChatGPT/4 GitHub PR 代码检查机器人</p>

<p align="center">
  <a href="https://discord.gg/ccZn9ZMfFf">
    <img src="https://img.shields.io/badge/chat-Discord-7289DA?logo=discord" alt="flows.network Discord">
  </a>
  <a href="https://twitter.com/flows_network">
    <img src="https://img.shields.io/badge/Twitter-1DA1F2?logo=twitter&amp;logoColor=white" alt="flows.network Twitter">
  </a>
   <a href="https://flows.network/flow/createByTemplate/code-review-for-github-pull-requests">
    <img src="https://img.shields.io/website?up_message=deploy&url=https%3A%2F%2Fflows.network%2Fflow%2Fnew" alt="Create a flow">
  </a>
</p>

[部署此函数到 flows.network](#deploy-your-own-code-review-bot-in-3-simple-steps)，你将获得一个帮你检查代码和总结拉取请求的 GitHub 机器人。它可以帮助忙碌的开源贡献者更快地理解并对 PR 采取行动!下面是一些示例!

* [[C++] 优化 WasmEdge C++ SDK](https://github.com/WasmEdge/WasmEdge/pull/2428#issuecomment-1524733889)
* [[C++] 为 WasmEdge 创建 OpenCV plugin](https://github.com/WasmEdge/WasmEdge/pull/2403#issuecomment-1509595889)
* [[Haskell] 优化 WasmEdge Component Model tooling](https://github.com/second-state/witc/pull/73#issuecomment-1509586233)

该机器人可以检查 PR 中更改的文件。 或者，可以使用[这个 bot](https://github.com/flows-network/github-pr-summary) 来总结 PR 中提交的信息。

## 如何工作

当在指定的 GitHub repo 中提出新的 PR 时，将触发此 flow 函数（或🤖）。 flow 函数收集 PR 中的更改文件，并让 ChatGPT/4 对其进行检查和总结。 然后将结果作为评论发回 PR。 flow 函数是用 Rust 编写的，并在 [flows.network](https://flows.network/) 上在托管的 [WasmEdge Runtimes](https://github.com/wasmedge) 中运行。

* 每次将新的提交推送到此 PR 时，都会自动更新代码检查评论。
* 当有人在 PR 的评论中说出一个魔法*触发词*时，可以触发新的代码检查。默认的触发词是"flows summarize"。

## 简单3步部署自己的代码检查机器人

1. 从模板创建一个机器人
2. 添加你的 OpenAI API密钥
3. 配置机器人以检查指定 GitHub repo 上的PR

### 0 事前准备

需要使用自己的 [OpenAI API 密钥](https://openai.com/blog/openai-api)。如果还没有密钥，请[在此处注册](https://platform.openai.com/signup)。

还需要使用你的 GitHub 帐户登录 [flows.network](https://flows.network/)。这是免费的。

### 1 从模板创建机器人

[**单击此处**](https://flows.network/flow/createByTemplate/Code-Review-Pull-Request)

请检查 `trigger_phrase` 变量。这是你在 PR 评论中手动召唤检查机器人的魔法词。

单击 **Create and Build** 按钮。

### 2 添加你的 OpenAI API 密钥

现在你将设置 OpenAI integration。单击**连接**，输入您的密钥并为其命名。

[<img width="450" alt="image" src="https://user-images.githubusercontent.com/45785633/222973214-ecd052dc-72c2-4711-90ec-db1ec9d5f24e.png">](https://user-images.githubusercontent.com/45785633/222973214-ecd052dc-72c2-4711-90ec-db1ec9d5f24e.png)

完成后关闭选项卡并返回 flow.network 页面。 点击**继续**。

## 3 配置机器人以访问 GitHub

接下来，你需要告诉机器人它需要监控哪个 GitHub repo 以查看即将到来的 PR 进行检查。

* `github_owner`:  *你想要为 repo 部署 🤖 的* GitHub org
* `github_repo` :  *你想部署 🤖 的* GitHub repo

> 让我们看一个例子。您想要部署机器人来检查`WasmEdge/wasmedge_hyper_demo` repo 中的PR代码。这里 `github_owner = WasmEdge` 且 `github_repo = wasmedge_hyper_demo`。

点击 **Connect** 或 **+ Add new authentication** 按钮，以使函数可以访问 GitHub repo 并部署🤖️。你将被重定向到一个新页面，在此页面须授予 [flows.network](https://flows.network/) 对该 repo 的权限。

[<img width="450" alt="image" src="https://github.com/flows-network/github-pr-summary/assets/45785633/6cefff19-9eeb-4533-a20b-03c6a9c89473">](https://github.com/flows-network/github-pr-summary/assets/45785633/6cefff19-9eeb-4533-a20b-03c6a9c89473)

完成后请关闭标签页并返回 flow.network 页面。点击 **Deploy**.

### 等待魔法的到来！

这就好了！你现在处在 flow 详细信息页面，正在等待 flow 函数构建。一旦 flow 状态变为 `运行中`，机器人就准备好进行代码检查了！每个新PR、每个新提交以及PR评论中的魔法词（即`trigger_phrase`），都会召唤机器人。