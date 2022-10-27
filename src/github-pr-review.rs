
use dotenv::dotenv;
use flowsnet_platform_sdk::logger;
use github_flows::{
    get_octo, listen_to_event,
    octocrab::models::events::payload::{IssueCommentEventAction, PullRequestEventAction},
    octocrab::models::CommentId,
    EventPayload, GithubLogin
};
use http_req::{
    request::{Method, Request},
    uri::Uri,
};
use openai_flows::{
    chat::{ChatModel, ChatOptions},
    OpenAIFlows,
};
use std::env;

//  The soft character limit of the input context size
//   the max token size or word count for GPT4 is 8192
//   the max token size or word count for GPT35Turbo is 4096
static CHAR_SOFT_LIMIT : usize = 9000;
static MODEL : ChatModel = ChatModel::GPT35Turbo;
// static MODEL : ChatModel = ChatModel::GPT4;

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() -> anyhow::Result<()> {
    dotenv().ok();
    logger::init();
    log::debug!("Running function at github-pr-review/main");

    let owner = env::var("github_owner").unwrap_or("juntao".to_string());
    let repo = env::var("github_repo").unwrap_or("test".to_string());
    let trigger_phrase = env::var("trigger_phrase").unwrap_or("flows review".to_string());

    let events = vec!["pull_request", "issue_comment"];
    println!("MAGIC");
    listen_to_event(&GithubLogin::Default, &owner, &repo, events, |payload| {
        handler(
            &owner,
            &repo,
            &trigger_phrase,
            payload,
        )
    })
    .await;