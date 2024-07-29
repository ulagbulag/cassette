use patternfly_yew::prelude::*;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub msg: AttrValue,
}

#[function_component(Error)]
pub fn error(props: &Props) -> Html {
    let Props { msg } = props;

    html! {
        <Content>
            <Alert inline=true title="Error" r#type={ AlertType::Danger }>
                { msg.clone() }
            </Alert>
        </Content>
    }
}
