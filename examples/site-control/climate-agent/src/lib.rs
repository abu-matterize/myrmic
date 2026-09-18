#![no_std]

use myrmic_sdk::db::state::State;
use myrmic_sdk::{Metadata, Result, Sri, publish, send};

const CONTROLLER: &str = "controller";

const HEATING: State<bool> = State::new_const("heating");

#[derive(serde::Serialize, serde::Deserialize, myrmic_sdk::Message)]
struct SiteState {
    temperature: f32,
    heating: bool,
    target_low: f32,
    target_high: f32,
}

#[myrmic_sdk::evt]
fn site_state(_md: Metadata, site: SiteState) -> Result<()> {
    let heating = HEATING.load()?.unwrap_or_default();

    if !heating && site.temperature < site.target_low {
        controller(true)?;
        HEATING.save(&true)?;
        publish("heating_requested", &true)?;
    } else if heating && site.temperature > site.target_high {
        controller(false)?;
        HEATING.save(&false)?;
        publish("heating_requested", &false)?;
    }

    Ok(())
}

fn controller(on: bool) -> Result<()> {
    let controller = Sri::of_path(CONTROLLER).map_err(|_| "invalid controller srn")?;

    send(controller, "set_heating", &on)
}
