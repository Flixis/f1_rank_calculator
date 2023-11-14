use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="F1 Elo Calculator"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {

    view! {
        <NavBar/> //See Navbar component
        //Homepage stuff
        <div class="flex justify-center items-center h-screen bg-red-100">
        <DriverDisplayList/>
        </div>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}


#[component]
fn NavBar() -> impl IntoView {
    view! {
        <nav class="bg-stone-950">
            <div class="container mx-auto px-4">
                <div class="flex justify-between">
                    // <!-- Logo -->
                    <text class="text-white">"F1 Elo Calculator"</text>
                    <div>
                    <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" width="120px" height="30px" viewBox="0 0 120 30" version="1.1" class="injected-svg js-svg-inject"><script xmlns=""/>
                    <defs>
                        <path d="M101.086812,30 L101.711812,30 L101.711812,27.106875 L101.722437,27.106875 L102.761812,30 L103.302437,30 L104.341812,27.106875 L104.352437,27.106875 L104.352437,30 L104.977437,30 L104.977437,26.25125 L104.063687,26.25125 L103.055562,29.18625 L103.044937,29.18625 L102.011187,26.25125 L101.086812,26.25125 L101.086812,30 Z M97.6274375,26.818125 L98.8136875,26.818125 L98.8136875,30 L99.4699375,30 L99.4699375,26.818125 L100.661812,26.818125 L100.661812,26.25125 L97.6274375,26.25125 L97.6274375,26.818125 Z M89.9999375,30 L119.999937,0 L101.943687,0 L71.9443125,30 L89.9999375,30 Z M85.6986875,13.065 L49.3818125,13.065 C38.3136875,13.065 36.3768125,13.651875 31.6361875,18.3925 C27.2024375,22.82625 20.0005625,30 20.0005625,30 L35.7324375,30 L39.4855625,26.246875 C41.9530625,23.779375 43.2255625,23.52375 48.4068125,23.52375 L75.2405625,23.52375 L85.6986875,13.065 Z M31.1518125,16.253125 C27.8774375,19.3425 20.7530625,26.263125 16.9130625,30 L-6.25e-05,30 C-6.25e-05,30 13.5524375,16.486875 21.0849375,9.0725 C28.8455625,1.685 32.7143125,0 46.9486875,0 L98.7643125,0 L87.5449375,11.21875 L48.0011875,11.21875 C37.9993125,11.21875 35.7518125,11.911875 31.1518125,16.253125 Z" id="path-1"/>
                    </defs>
                    <g id="Page-1">
                        // <@_ id="Fill-1" fill="#EE0000" xlink:href="#path-1"/> //use keyword not in leptos yet.
                    </g>
                </svg>
                    </div>
                    // <!-- Navigation Links -->
                    <div class="hidden md:flex space-x-4">
                        <a href="#" class="text-white">"Home"</a>
                        <a href="#" class="text-white">"Schedule"</a>
                        <a href="#" class="text-white">"Teams"</a>
                        <a href="#" class="text-white">"Drivers"</a>
                        <a href="#" class="text-white">"Standings"</a>
                    </div>
    
                    // <!-- Mobile Menu Button -->
                    <div class="md:hidden">
                        <button class="text-white">"Menu"</button>
                    </div>
                </div>
            </div>
        </nav>
    }
}


#[component]
fn DriverDisplayList() -> impl IntoView {

    view! {
        <div class="text-center">
            <h1>Welcome to f1 elo calculator!</h1>
            <table class="mx-auto border-collapse border border-gray-300">
                <tr>
                    <th class="border border-gray-300">Driver</th>
                    <th class="border border-gray-300">Team</th>
                    <th class="border border-gray-300">Country</th>
                </tr>
                <tr>
                    <td class="border border-gray-300">Max Verstappen</td>
                    <td class="border border-gray-300">RedBull Racing</td>
                    <td class="border border-gray-300">Netherlands</td>
                </tr>
                <tr>
                    <td class="border border-gray-300">Fernando Alonso</td>
                    <td class="border border-gray-300">Aston Martin Racing</td>
                    <td class="border border-gray-300">Spain</td>
                </tr>
                <tr>
                    <td class="border border-gray-300">Yuki Tsunoda</td>
                    <td class="border border-gray-300">Alpha Tauri Racing</td>
                    <td class="border border-gray-300">Japan</td>
                </tr>
            </table>
        </div>
    }

}