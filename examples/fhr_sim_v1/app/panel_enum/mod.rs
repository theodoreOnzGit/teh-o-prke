
#[derive(serde::Deserialize, serde::Serialize,PartialEq,Clone)]
pub(crate) enum Panel {
    MainPage,
    ReactorPowerGraphs,
    PoisonGraphs,

}
