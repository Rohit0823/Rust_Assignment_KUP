#[derive(PartialEq, Eq, Debug)]
///IpAddress enum which used to encapsulate the class of ipAddress
///
/// #field
///
/// ClassA,B,C,D:-Classes is field of enum IpAddress and associated with String type

pub enum IpAddress {
    ClassA(String),
    ClassB(String),
    ClassC(String),
    ClassD(String),
    ClassE(String),
}

/// check_ip_address is used to check the given ip_Address
///
/// #Arguments
///
///ipconfig: It is a tuple object whose Store the Dotted decimal Notation.
///
/// #Return
///
/// Returns Result enum which used give the Class Of Ip and handle Error
pub fn check_ip_address(ipconfig: (u128, u128, u128, u128)) -> Result<IpAddress, String> {
    match ipconfig {
        (1..=126, 0..=255, 0..=255, 1..=254) => Ok(IpAddress::ClassA(format!(
            "{}.{}.{}.{}",
            ipconfig.0, ipconfig.1, ipconfig.2, ipconfig.3
        ))),
        (128..=191, 0..=255, 0..=255, 1..=254) => Ok(IpAddress::ClassB(format!(
            "{}.{}.{}.{}",
            ipconfig.0, ipconfig.1, ipconfig.2, ipconfig.3
        ))),
        (192..=223, 0..=255, 1..=254, 1..=254) => Ok(IpAddress::ClassC(format!(
            "{}.{}.{}.{}",
            ipconfig.0, ipconfig.1, ipconfig.2, ipconfig.3
        ))),
        (224..=239, 0..=255, 0..=255, 0..=255) => Ok(IpAddress::ClassD(format!(
            "{}.{}.{}.{}",
            ipconfig.0, ipconfig.1, ipconfig.2, ipconfig.3
        ))),
        (240..=254, 0..=255, 0..=255, 0..=254) => Ok(IpAddress::ClassE(format!(
            "{}.{}.{}.{}",
            ipconfig.0, ipconfig.1, ipconfig.2, ipconfig.3
        ))),
        _ => Err("Unwanted Input".to_string()),
    }
}
fn main() {
    println!("{:?}", check_ip_address((192, 0, 1, 1)));
}