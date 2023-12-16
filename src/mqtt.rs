extern crate mosquitto_client as mosq;
use mosq::Mosquitto;
use std::error::Error;

pub fn publish(data: String) -> Result<(), Box<dyn Error>> {
    let m = Mosquitto::new("test");
    m.connect("localhost", 1883)?;

    // publish and get a message id
    let our_mid = m.publish("pc", data.as_bytes(), 2, false)?;

    // and wait for confirmation for that message id
    let mut mc = m.callbacks(());
    mc.on_publish(|_, mid| {
        if mid == our_mid {
            m.disconnect().unwrap();
        }
    });

    // wait for 1 sec (use -1 for forever) until explicit disconnect
    m.loop_until_disconnect(1)?;
    Ok(())
}
