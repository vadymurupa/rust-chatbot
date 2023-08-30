use leptos::*;

const USER_MESSAGE_CLASS &str =""
const MODEL_MESSAGE_CLASS &str =""
#[component]
pub fn ChatArea(cx: Scope, conversation: ReadSignal<Conversation>) -> impl IntoView {
    let input_ref = create_node_ref::<div>(cx);

    view! {
        cx,
        <div>
        {
            move ||conversation.get().messages.iter().map(move |message| {
                let class_str= if message.user {USER_MESSAGE_CLASS} else {MODEL_MESSAGE_CLASS};
                view! {
                    cx, 
                    <div class={class_str}>
                    {message.text.clone()}
                    </div>
                }
            }).collect::<Vec<>>()
        }
        </div>
    }
}