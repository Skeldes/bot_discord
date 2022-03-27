use serenity::{
    framework::standard::{macros::command,Args,  CommandResult},
    model::prelude::*,
    prelude::*,
};

#[command]
async fn statut(ctx :&Context, msg: &Message, args :Args) -> CommandResult {

    let statut = args.rest();
    ctx.set_activity(Activity::playing(statut)).await;

    msg.channel_id.say(ctx, "Statut change").await?;

    Ok(())
}