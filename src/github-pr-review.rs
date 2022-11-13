
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

    Ok(())
}

async fn handler(
    owner: &str,
    repo: &str,
    trigger_phrase: &str,
    payload: EventPayload,
) {
    // log::debug!("Received payload: {:?}", payload);
    let mut new_commit : bool = false;
    let (title, pull_number, _contributor) = match payload {
        EventPayload::PullRequestEvent(e) => {
            if e.action == PullRequestEventAction::Opened {
                log::debug!("Received payload: PR Opened");
            } else if e.action == PullRequestEventAction::Synchronize {
                new_commit = true;
                log::debug!("Received payload: PR Synced");
            } else {
                log::debug!("Not a Opened or Synchronize event for PR");
                return;
            }
            let p = e.pull_request;
            (
                p.title.unwrap_or("".to_string()),
                p.number,
                p.user.unwrap().login,
            )
        }
        EventPayload::IssueCommentEvent(e) => {
            if e.action == IssueCommentEventAction::Deleted {
                log::debug!("Deleted issue comment");
                return;
            }
            log::debug!("Other event for issue comment");

            let body = e.comment.body.unwrap_or_default();

            // if e.comment.performed_via_github_app.is_some() {
            //     return;
            // }
            // TODO: Makeshift but operational
            if body.starts_with("Hello, I am a [code review bot]") {
                log::info!("Ignore comment via bot");
                return;
            };

            if !body.to_lowercase().contains(&trigger_phrase.to_lowercase()) {