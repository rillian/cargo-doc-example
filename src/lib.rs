use log::info;

pub fn check() {
    info!("Checking...");
}

#[cfg(test)]
mod tests {

    #[test]
    fn check() {
        super::check();
    }
}
