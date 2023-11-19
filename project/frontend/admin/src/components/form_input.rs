use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SubmitProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or(false)]
    pub is_loading: bool,
}

#[function_component(SubmitButton)]
pub fn submit_button(props: &SubmitProps) -> Html {
    let button_classes = vec![
        "inline-block", 
        "w-full", 
        "rounded", 
        "bg-red-500", 
        "px-7", 
        "pb-2.5", 
        "pt-3", 
        "text-sm", 
        "font-medium", 
        "uppercase", 
        "leading-normal", 
        "text-white", 
        "shadow-[0_4px_9px_-4px_#3b71ca]", 
        "transition", 
        "duration-150", 
        "ease-in-out", 
        "hover:bg-red-600", 
        "hover:shadow-[0_8px_9px_-4px_rgba(59,113,202,0.3),0_4px_18px_0_rgba(59,113,202,0.2)]", 
        "focus:bg-red-600", 
        "focus:shadow-[0_8px_9px_-4px_rgba(59,113,202,0.3),0_4px_18px_0_rgba(59,113,202,0.2)]", 
        "focus:outline-none", 
        "focus:ring-0",
        "active:bg-red-700",
        "active:shadow-[0_8px_9px_-4px_rgba(59,113,202,0.3),0_4px_18px_0_rgba(59,113,202,0.2)]",
        "disabled:bg-red-400", 
        "disabled:opacity-75",
        "disabled:cursor-not-allowed",
        "disabled:hover:shadow-[0_4px_9px_-4px_#3b71ca]"
    ];

    html! {
        <button
            type="submit"
            disabled={props.is_loading}
            class={classes!(button_classes)}
            data-te-ripple-init="true"
            data-te-ripple-color="light">
            if props.is_loading {
            <div
            class="inline-block h-4 w-4 animate-spin rounded-full border-2 border-solid border-current border-r-transparent align-[-0.125em] motion-reduce:animate-[spin_1.5s_linear_infinite]"
            role="status">
            <span
                class="!absolute !-m-px !h-px !w-px !overflow-hidden !whitespace-nowrap !border-0 !p-0 ![clip:rect(0,0,0,0)]"
                >{"Loading..."}
            </span>
            </div>} 
            else {<>{props.children.clone()}</>}
        </button>
    }
}

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
    #[prop_or(AttrValue::from("999999"))]
    pub maxlength: AttrValue,
    #[prop_or_default]
    pub value: AttrValue,
    pub type_handler: AttrValue,
    #[prop_or_default]
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
            <input maxlength={&props.maxlength} id={&props.id} onkeypress={props.onkeypress.clone()} ref={props.input_ref.clone()} type={&props.type_handler} value={&props.value} class={classes!(input_class)} placeholder=" " />
            <label for={&props.id} class={classes!(label_class)}>
                {props.children.clone()}
            </label>
        </div>
    }
}