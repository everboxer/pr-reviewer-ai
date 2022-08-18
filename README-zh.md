
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