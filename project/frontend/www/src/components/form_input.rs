use yew::prelude::*;

#[derive(PartialEq, Clone, Copy)]
pub enum InputStatus {
    Success,
    Error,
    Neutral
}

#[derive(PartialEq, Properties)]
pub struct TextInputProps {
    pub children: Children,
    pub id: AttrValue,
    pub type_handler: AttrValue,
    pub input_ref: NodeRef,
    pub status: InputStatus,
    
    #[prop_or_default]
    pub onkeypress: Callback<KeyboardEvent>
}

#[function_component(TextInput)]
pub fn form_text_input(props: &TextInputProps) -> Html {
    let mut input_class = vec!["block", "bg-white", "w-full", "py-[-10px]", "focus:outline-none", "focus:shadow-outline", "border", "rounded-md", "py-3", "px-3", "block", "appearance-none", "leading-normal", "transition-colors"];
    let mut label_class = vec!["absolute", "top-3", "left-0", "pointer-events-none", "transition", "duration-200", "ease-in-outbg-white", "px-2", "transition-colors"];

    match props.status {
        InputStatus::Success => {
            input_class.push("border-green-500");
            label_class.push("text-green-500")
        },
        InputStatus::Error => {
            input_class.push("border-red-500");
            label_class.push("text-red-500")
        },

        InputStatus::Neutral => {
            input_class.push("border-amber-700");
            label_class.push("text-amber-700")
        }
    }

    html! {
        <div class="relative mb-6 float-label-input">
            <input id={&props.id} onkeypress={props.onkeypress.clone()} ref={props.input_ref.clone()} type={&props.type_handler} class={classes!(input_class)} placeholder=" " />
            <label for={&props.id} class={classes!(label_class)}>
                {props.children.clone()}
            </label>
        </div>
    }
}