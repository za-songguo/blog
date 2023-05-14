use crate::components::{
    article::{delete::DeleteArticle, edit::EditArticle, new::NewArticle, view::ArticleViewer},
    comment::delete::DeleteComment,
    container::Container,
    home::Home,
    not_found::NotFound,
    user::{login::Login, oauth::OAuth},
};

use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

/// 主路由
#[derive(Debug, Clone, PartialEq, Eq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/article/:article_id")]
    ArticleViewer { article_id: u32 },
    #[at("/article/new")]
    NewArticle,
    #[at("/article/edit/:article_id")]
    EditArticle { article_id: u32 },
    #[at("/article/delete/:article_id")]
    DeleteArticle { article_id: u32 },
    #[at("/comment/delete/:comment_id")]
    DeleteComment { comment_id: u32 },
    #[at("/user/login")]
    Login,
    #[at("/user/login/oauth")]
    OAuth,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(route: Route) -> Html {
    html! {
        <Container>
        {
            match route {
                Route::Home => html! { <Home/> },
                Route::ArticleViewer { article_id } => html! { <ArticleViewer {article_id}/> },
                Route::NewArticle => html! { <NewArticle/> },
                Route::EditArticle { article_id } => html! { <EditArticle {article_id}/> },
                Route::DeleteArticle { article_id } => html! { <DeleteArticle {article_id}/> },
                Route::DeleteComment { comment_id } => html! { <DeleteComment {comment_id}/> },
                Route::Login => html! { <Login/> },
                Route::OAuth => html! { <OAuth/> },
                Route::NotFound => html! { <NotFound/> },
            }
        }
        </Container>
    }
}
