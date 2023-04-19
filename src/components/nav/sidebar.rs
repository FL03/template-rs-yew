/*
    Appellation: sidebar <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/

use yew::prelude::*;
use yew_router::prelude::*;

pub enum SidebarMessage {
    Open,
    Close,
}

pub struct Sidebar {
    pub open: bool,
}

impl Component for Sidebar {
    type Message = SidebarMessage;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self { open: false }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            SidebarMessage::Close => {
                self.open = false;
            }
            SidebarMessage::Open => {
                self.open = true;
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {

            <section class="flex flex-auto grow nowrap scrollable m-0 p-0 min-h-full max-h-screen min-w-full max-w-screen">

            </section>

        }
    }
}
