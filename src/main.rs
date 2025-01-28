use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum RootRoute {
    #[at("/dev-tool/:s")]
    Route,
}

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/about")]
    About,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn root_route(routes: RootRoute) -> Html {
    match routes {
        RootRoute::Route => html! {
            <Switch<Route> render={switch} />
        },
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::About => html! { <p>{ "About" }</p> },
        Route::NotFound => html! { <p>{ "Not Found" }</p> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        // ********************************************************
        // **    basename is not supported on yew 0.19.0 yet.    **
        // <BrowserRouter basename="/yew-template-for-github-io/">
        //     <Switch<Route> render={Switch::render(switch)} />
        // </BrowserRouter>
        // ********************************************************
        <BrowserRouter>
            <Switch<RootRoute> render={root_route} />
        </BrowserRouter>
    }
}

/// entry point
fn main() {
    yew::Renderer::<App>::new().render();
}
