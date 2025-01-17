use yew::prelude::*;

pub struct Home;
impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
        <div class="">
          <div class="">
            <div class="">
              <h1 class="">{ "Welcome..." }</h1>
              <h2 class="">{ "...to the best yew content" }</h2>
            </div>
          </div>

          <div class="">
            <figure class="">
              <img
                alt="A random image for the input term 'yew'."
                src="https://source.unsplash.com/random/1200x400/?yew"
              />
            </figure>
          </div>

          <div class="">{ self.view_info_tiles() }</div>
        </div>
        }
    }
}

impl Home {
    fn view_info_tiles(&self) -> Html {
        html! {
            <>
                <div class="bg-red-500">
                    <div class="tile is-child box">
                        <p class="title">{ "What are yews?" }</p>
                        <p class="subtitle">{ "Everything you need to know!" }</p>

                        <div class="content">
                            {r#"
                            A yew is a small to medium-sized evergreen tree, growing 10 to 20 metres tall, with a trunk up to 2 metres in diameter.
                            The bark is thin, scaly brown, coming off in small flakes aligned with the stem.
                            The leaves are flat, dark green, 1 to 4 centimetres long and 2 to 3 millimetres broad, arranged spirally on the stem,
                            but with the leaf bases twisted to align the leaves in two flat rows either side of the stem,
                            except on erect leading shoots where the spiral arrangement is more obvious.
                            The leaves are poisonous.
                            "#}
                        </div>
                    </div>
                </div>

                <div class="tile is-parent">
                    <div class="tile is-child box">
                        <p class="title">{ "Who are we?" }</p>

                        <div class="content">
                            { "We're a small team of just 2" }
                            <sup>{ 64 }</sup>
                            { " members working tirelessly to bring you the low-effort yew content we all desperately crave." }
                            <br />
                            {r#"
                                We put a ton of effort into fact-checking our posts.
                                Some say they read like a Wikipedia article - what a compliment!
                            "#}
                        </div>
                    </div>
                </div>
            </>
        }
    }
}
