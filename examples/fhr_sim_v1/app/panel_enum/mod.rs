
#[derive(serde::Deserialize, serde::Serialize,PartialEq,Clone, Debug)]
pub(crate) enum Panel {
    MainPage,
    ReactorPowerGraphs,
    PoisonGraphs,

}
