use serenity::{
    framework::standard::{
        Args,
        CommandResult,
        macros::command,
    },
    model::prelude::*,
    prelude::*,
};

use std::time::Duration;

#[command]
async fn challenge(ctx : &Context, msg: &Message, _: Args) -> CommandResult {
    let mut score = 0u32;
    let _ = msg.reply(ctx, "How was that crusty crabs called again? 10secs !").await;

    if let Some(answer) = &msg.author.await_reply(&ctx).timeout(Duration::from_secs(10)).await {
        if answer.content.to_lowercase() == "ferris" {
            let _ = answer.reply(ctx, "That's correct !").await;
            score += 1;
        }
        else {
            let _ = msg.reply(ctx,"Wrong ! it's Ferris").await;
        }
    }else {
        let _ = msg.reply(ctx, "No answer within 10secs").await;
    }

    //let react_msg = msg.reply(ctx, "React with the reaction representing 1, yu got 10 seconds !");

    Ok(())
}