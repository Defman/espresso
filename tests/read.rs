use anyhow::Result;
use espresso::parse_class;

#[test]
fn read() -> Result<()> {
    let mut fake_packet = &include_bytes!("./classes/FakePacket.class")[..];
    let fake_packet = parse_class(&mut fake_packet)?;
    println!("{:#?}", fake_packet);
    Ok(())
}

#[test]
fn ron() -> Result<()> {
    let mut fake_packet = &include_bytes!("./classes/FakePacket.class")[..];
    let fake_packet = parse_class(&mut fake_packet)?;
    let fake_packet_ser = ron::ser::to_string_pretty(&fake_packet, Default::default())?;
    let fake_packet_de = ron::de::from_str(&fake_packet_ser)?;

    println!("{}", fake_packet_ser);
    assert_eq!(fake_packet, fake_packet_de);
    Ok(())
}