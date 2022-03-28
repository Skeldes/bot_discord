use serenity::{
    collector::MessageCollectorBuilder,
    framework::standard::{
        Args,
        CommandResult,
        macros::command,
    },
    futures::stream::StreamExt,
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

    let react_msg = msg
        .reply(ctx, "React with the reaction representing 1, you got 10 seconds !")
        .await
        .unwrap();

    if let Some(reaction) = &react_msg
        .await_reaction(&ctx)
        .timeout(Duration::from_secs(10))
        .author_id(msg.author.id)
        .await {
            let emoji = &reaction.as_inner_ref().emoji;
            let _ = match emoji.as_data().as_str() {
                "1️⃣" => {
                    score +=1;
                    msg.reply(ctx, "That's correct!").await
                },
                _ => msg.reply(ctx, "wrong !").await,
            };
    } else {
        let _ = msg.reply(ctx, "No reaction within 10 seconds").await;
    };


    let _ = msg.reply(ctx, "Write 5 message within 10secondes !").await;

    let collector = MessageCollectorBuilder::new(&ctx)
        .author_id(msg.author.id)
        .channel_id(msg.channel_id)
        .collect_limit(5u32)
        .timeout(Duration::from_secs(10))
        .await;

    let http = &ctx.http;
    let collected: Vec<_> = collector
        .then(|msg| async move {
            let _ = msg.reply(http, format!("I repeat: {}", msg.content)).await;

            msg
        })
        .collect()
        .await;

    if collected.len() >= 5 {
        score += 1;
        let _ = msg.reply(ctx, "Good job !").await;
    } else {
        let _ = msg.reply(ctx, format!("You fail ! you only send {} messages", collected.len())).await;
    }

    Ok(())
}