use crate::route::router::Route;
use yew::prelude::*;
use yew_router::{
    prelude::{use_location, use_navigator, Navigator},
    Routable,
};

const ACTIVE_CLASS: &'static str="block py-2 pl-3 pr-4 text-white bg-blue-700 rounded md:bg-transparent md:text-blue-700 md:p-0 dark:text-white";
const INACTIVE_CLASS: &'static str="block py-2 pl-3 pr-4 text-gray-700 rounded hover:bg-gray-100 md:hover:bg-transparent md:border-0 md:hover:text-blue-700 md:p-0 dark:text-gray-400 md:dark:hover:text-white dark:hover:bg-gray-700 dark:hover:text-white md:dark:hover:bg-transparent";

#[function_component(NavBar)]
pub fn nav_bar() -> Html {
    let navigator = use_navigator().unwrap();
    let location = use_location().unwrap();

    let redirect =
        |nav: Navigator, route: Route| Callback::from(move |_: MouseEvent| nav.push(&route));

    let getclass = |route: Route| {
        if location.path() == route.to_path() {
            ACTIVE_CLASS
        } else {
            INACTIVE_CLASS
        }
    };

    html! {
        <nav class="bg-white border-gray-200 px-2 sm:px-4 py-2.5 dark:bg-gray-900">
        <div class="container flex flex-wrap items-center justify-between mx-auto">
          <div class="hidden w-full md:block md:w-auto" id="navbar-default">
            <ul class="flex flex-col p-4 mt-4 border border-gray-100 rounded-lg bg-gray-50 md:flex-row md:space-x-8 md:mt-0 md:text-sm md:font-medium md:border-0 md:bg-white dark:bg-gray-800 md:dark:bg-gray-900 dark:border-gray-700">
              <li>
                <span onclick={redirect(navigator.clone(), Route::HomePage)} class={getclass(Route::HomePage)}>{"Home"}</span>
              </li>
              <li>
                <span onclick={redirect(navigator.clone(), Route::NonogramPage)} class={getclass(Route::NonogramPage)}>{"NonnoGrampa"}</span>
              </li>
              <li>
                <span onclick={redirect(navigator.clone(), Route::SnakePage)} class={getclass(Route::SnakePage)}>{"Snaken't"}</span>
              </li>
              <li>
                <span onclick={redirect(navigator.clone(), Route::SukoduPage)} class={getclass(Route::SukoduPage)}>{"Sukodu"}</span>
              </li>
            </ul>
          </div>
        </div>
      </nav>
    }
}
