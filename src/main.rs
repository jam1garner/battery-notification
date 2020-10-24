use color_eyre::eyre;

use battery::Manager;
use battery::units::ratio::percent;

use notify_rust::Notification;

use cronjob::CronJob;

fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    let mut cron = CronJob::new("Battery Notificatoin", check_battery_job);

    cron.seconds("0");

    cron.start_job();

    Ok(())
}

fn check_battery_job(_: &str) {
    check_battery().unwrap();
}

fn check_battery() -> eyre::Result<()> {
    let primary_battery = Manager::new()?
        .batteries()?
        .nth(0)
        .ok_or_else(|| eyre::eyre!("No battery found"))??;

    let primary_percent = primary_battery.state_of_charge().get::<percent>();

    if primary_percent < 10.0 && primary_battery.state() != battery::State::Charging {
        Notification::new()
            .summary("Low battery")
            .icon("low-battery")
            .body("The primary battery has dropped below 10%")
            .show()?;
    }
    
    Ok(())
}
