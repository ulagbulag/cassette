use std::rc::Rc;

use patternfly_yew::prelude::*;
use yew::prelude::*;

use crate::data::table::DataTable;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub uri: String,
    pub values: Rc<DataTable>,

    #[prop_or_default]
    pub delete: Option<bool>,

    #[prop_or_default]
    pub update: Option<bool>,
}

#[function_component(BaseActor)]
pub fn base_actor(props: &Props) -> Html {
    let Props { .. } = props;

    html! {
        <Content>
            <Alert inline=true title="Error" r#type={ AlertType::Danger }>
            </Alert>
        </Content>
    }
}
