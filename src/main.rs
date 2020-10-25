use cronjob::CronJob;
use battery::Manager;
use color_eyre::eyre;
use notify_rust::Notification;
use battery::units::ratio::percent;

fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    let mut cron = CronJob::new("Battery Notification", check_battery);

    // Once a minute
    cron.seconds("0");
    Ok(cron.start_job())
}

fn check_battery(_: &str) {
    (|| -> eyre::Result<()> {
        let primary_battery = Manager::new()?
            .batteries()?
            .next()
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
    })().unwrap()
}
