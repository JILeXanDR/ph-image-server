use user_agent_parser::{Device, Product, OS};

pub fn get_device_from_parsed_user_agent<'a>(device: Device<'a>) -> i8 {
    // TODO:
    match device.name {
        None => 0,
        Some(_) => 0,
    }
}

pub fn get_os_from_parsed_user_agent<'a>(os: OS<'a>) -> i8 {
    // TODO:
    match os.name {
        None => 0,
        Some(_) => 0,
    }
}

pub fn get_browser_from_parsed_user_agent<'a>(product: Product<'a>) -> i8 {
    // TODO:
    match product.name {
        None => 0,
        Some(_) => 0,
    }
}
