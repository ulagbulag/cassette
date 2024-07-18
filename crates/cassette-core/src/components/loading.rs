use patternfly_yew::prelude::*;
use yew::prelude::*;

#[function_component(Loading)]
pub fn loading() -> Html {
    html! {
        <Flex >
            <FlexItem>
                <Spinner size={ SpinnerSize::Lg } />
            </FlexItem>
            <FlexItem modifiers={[ FlexModifier::Align(Alignment::Center) ]}>
                <Content>
                    <p>{ "Loading..." }</p>
                </Content>
            </FlexItem>
        </Flex>
    }
}
