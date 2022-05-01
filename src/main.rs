use base64::encode;
fn main() {
  let mut data_origin = String::from("3458rfdnjhu24929842389rgufdhuir8u2947reghfudfiu27843738t");
  data_origin.push(data_origin.len() as u8 as char);
  for i in encode(&data_origin).as_bytes() {
    data_origin.push(*i as char)
  }
  let bytes = data_origin.as_bytes();
  let mut data = Vec::<u8>::new();
  for (i, v) in bytes.iter().enumerate() {
    data.push((*v / (((i as u8 + 1) % 4) + 1)) >> ((data_origin.len() / (i + 1)) % 2))
  }
  data.reverse();
  for i in data.iter() {
    print!("{:02X}", i)
  }
}
