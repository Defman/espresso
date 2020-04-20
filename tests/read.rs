use anyhow::Result;
use espresso::raw::parse;

#[test]
fn fake_packet_test() -> Result<()> {
    let mut fake_packet = &include_bytes!("./classes/FakePacket.class")[..];
    let fake_packet = parse(&mut fake_packet)?;

    println!("{}", fake_packet_ser);
    assert_eq!(fake_packet, fake_packet_de);
    Ok(())
}