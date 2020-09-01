extern crate ddc;

use ddc::Ddc;
use ddc_winapi::Monitor;

fn main() {
    let matches = clap::App::new("screenctl")
        .version("1.0")
        .author("Jon Valdés")
        .about("Changes screen brightness")
        .arg(clap::Arg::with_name("brightness").required(true))
        .get_matches();

    let brightness_string = matches.value_of("brightness").unwrap();
    let brightness = brightness_string.parse::<u16>().unwrap();

    for mut ddc in Monitor::enumerate().unwrap() {
        if let Ok(mccs_version) = ddc.get_vcp_feature(0xdf) {
            println!("MCCS version: {:04x}", mccs_version.maximum());
            ddc.set_vcp_feature(0x10, brightness).unwrap();
        }
    }
}
