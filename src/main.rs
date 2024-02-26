use leptos::{*, ev};
use uuid::Uuid;

fn main() {
    mount_to_body(move|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    let top_in_class = "border-slate-400 outline-slate-800 border rounded p-1 w-[99%]";

    let (show_title, set_show_title) = create_signal(false);
    let (new_title, set_new_title) = create_signal("".to_string());
    let (new_item, set_new_item) = create_signal("".to_string());

    let (containers, set_containers) = create_signal(vec![]);

    view! {
        <div>
            <div class="p-0 m-0 md:w-1/4 w-1/2 space-y-2 relative top-4 text-center pt-2 flex flex-col md:translate-x-[150%] translate-x-1/2">
                {move || {
                    show_title()
                        .then_some(
                            view! {
                                <input
                                    class=top_in_class
                                    class=("animate-slide-in", show_title)
                                    type="text"
                                    placeholder="title"
                                    prop:value=new_title
                                    on:input=move |ev| set_new_title(event_target_value(&ev))
                                />
                            },
                        )
                }}
                <input
                    class=top_in_class
                    type="text"
                    placeholder="new item"
                    prop:value=new_item
                    on:input=move |ev| set_new_item(event_target_value(&ev))
                    on:focus=move |_| {
                        if !show_title() {
                            set_show_title(true)
                        }
                    }
                />
                <svg
                    class="self-end cursor-pointer"
                    xmlns="http://www.w3.org/2000/svg"
                    viewBox="0 0 24 24"
                    width="2em"
                    height="2em"
                    fill="currentColor"
                    on:click=move |_| {
                        set_containers
                            .update(|containers| {
                                if new_item().trim().is_empty() && new_title().trim().is_empty() {
                                    return;
                                }
                                containers.push((Uuid::new_v4(), new_title(), new_item()));
                                set_new_title("".to_string());
                                set_new_item("".to_string());
                            })
                    }
                >

                    <path d="M12 22C6.47715 22 2 17.5228 2 12C2 6.47715 6.47715 2 12 2C17.5228 2 22 6.47715 22 12C22 17.5228 17.5228 22 12 22ZM11 11H7V13H11V17H13V13H17V11H13V7H11V11Z"></path>
                </svg>
            </div>
            <div class="flex flex-wrap justify-evenly mt-10">
                <For
                    each=move || containers().into_iter()
                    key=move |container: &(Uuid, String, String)| container.0.clone()
                    children=move |new_container| {
                        view! {
                            <ListContainer
                                title=new_container.1
                                first_item=new_container.2
                                on_delete=move |_| {
                                    set_containers
                                        .update(|containers| {
                                            containers
                                                .retain(|container| !container.0.eq(&new_container.0))
                                        })
                                }
                            />
                        }
                    }
                />

            </div>
        </div>
    }
}

#[component]
fn ListContainer<F: FnMut(ev::MouseEvent) + 'static>(
    title: String,
    first_item: String,
    on_delete: F
) -> impl IntoView {
    let (show_new_in, set_show_new_in) = create_signal(false);
    let (items, set_items) = create_signal(vec![(
        Uuid::new_v4(), first_item
    )]);

    view! {
        <div class="bg-slate-800 text-slate-400 md:w-[30%] w-[40%] p-2 m-2 rounded flex flex-col">
            <h3 class="text-2xl font-bold">{title}</h3>
            <ul class="mt-2 space-y-2">
                <For
                    each=move || items().into_iter()
                    key=|item: &(Uuid, String)| item.0.clone()
                    children=move |new_item| {
                        view! {
                            <ListItem
                                content=new_item.1
                                on_delete=move |_| {
                                    set_items
                                        .update(|items| {
                                            items.retain(|item| !item.0.eq(&new_item.0))
                                        })
                                }
                            />
                        }
                    }
                />

            </ul>
            <div class="flex gap-2 w-[100%] place-content-end mt-2">

                {move || {
                    show_new_in()
                        .then_some(
                            view! {
                                <input
                                    class="flex-grow ps-2 text-slate-900 rounded"
                                    type="text"
                                    on:blur=move |ev| {
                                        let content: String = event_target_value(&ev);
                                        (!content.is_empty())
                                            .then(|| {
                                                set_items
                                                    .update(|items| items.push((Uuid::new_v4(), content)))
                                            });
                                        set_show_new_in(false);
                                    }
                                />
                            },
                        )
                }}
                <ListActionButtonContainer>
                    <svg
                        class="self-end cursor-pointer fill-red-600"
                        xmlns="http://www.w3.org/2000/svg"
                        viewBox="0 0 24 24"
                        width="1.5em"
                        height="1.5em"
                        on:click=on_delete
                    >
                        <path d="M4 8H20V21C20 21.5523 19.5523 22 19 22H5C4.44772 22 4 21.5523 4 21V8ZM7 5V3C7 2.44772 7.44772 2 8 2H16C16.5523 2 17 2.44772 17 3V5H22V7H2V5H7ZM9 4V5H15V4H9ZM9 12V18H11V12H9ZM13 12V18H15V12H13Z"></path>
                    </svg>
                </ListActionButtonContainer> <ListActionButtonContainer>
                    <svg
                        class="self-end cursor-pointer fill-slate-900"
                        xmlns="http://www.w3.org/2000/svg"
                        viewBox="0 0 24 24"
                        width="1.5em"
                        height="1.5em"
                        on:click=move |_| set_show_new_in(true)
                    >
                        <path d="M12 22C6.47715 22 2 17.5228 2 12C2 6.47715 6.47715 2 12 2C17.5228 2 22 6.47715 22 12C22 17.5228 17.5228 22 12 22ZM11 11H7V13H11V17H13V13H17V11H13V7H11V11Z"></path>
                    </svg>
                </ListActionButtonContainer>
            </div>
        </div>
    }
}

#[component]
fn ListItem<F: FnMut(ev::MouseEvent) + 'static>(
    content: String,
    on_delete: F
) -> impl IntoView {
    view! {
        <li class="flex justify-between bg-slate-200 p-1 rounded">
            <p class="text-slate-900 w-[80%]">{content}</p>
            <svg
                class="fill-red-600 cursor-pointer"
                xmlns="http://www.w3.org/2000/svg"
                viewBox="0 0 24 24"
                width="2em"
                height="2em"
                on:click=on_delete
            >
                <path d="M12 22C6.47715 22 2 17.5228 2 12C2 6.47715 6.47715 2 12 2C17.5228 2 22 6.47715 22 12C22 17.5228 17.5228 22 12 22ZM12 10.5858L9.17157 7.75736L7.75736 9.17157L10.5858 12L7.75736 14.8284L9.17157 16.2426L12 13.4142L14.8284 16.2426L16.2426 14.8284L13.4142 12L16.2426 9.17157L14.8284 7.75736L12 10.5858Z"></path>
            </svg>
        </li>
    }
}

#[component]
fn ListActionButtonContainer(
    #[prop(default="bg-slate-200 rounded-full p-1")]
    bg_color: &'static str,
    children: Children
) -> impl IntoView {
    view! { <span class=bg_color>{children()}</span> }
}