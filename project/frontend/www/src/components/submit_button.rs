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