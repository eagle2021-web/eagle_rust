use clap::{App, Arg};
use trust_dns::rr::RecordType::A;
use url::Url;
use smoltcp::iface::EthernetInterface;
fn main() {
    let app = App::new("mget")
        .about("GET a webpage, manually")
        .arg(Arg::with_name("url")
            .short("u")
            .default_value("https://www.baidu.com"))
        .arg(Arg::with_name("tap-device")
            .short("t")
            .default_value("sssssssss"))
        .arg(Arg::with_name("dns-server")
            .short("d")
            .default_value("8.8.8.8"))
        .get_matches();

    let url_text = app.value_of("url").unwrap();
    println!("url = {:?}", url_text);
    let tap_device = app.value_of("tap-device").unwrap();
    println!("tap_device = {:?}", tap_device);
    let dns_server_text = app.value_of("dns-server").unwrap();
    println!("dns_server_text = {:?}", dns_server_text);

    let url = Url::parse(url_text)
        .expect("error: unable to parse <url> as a URL");
    if url.scheme() != "http" {
        eprintln!("eroor: only HTTP protocol supported");
        return;
    }


    let http::get();
}

#[cfg(test)]
mod test{
    use clap::{App, Arg};

    #[test]
    fn test_arg() {
        let m = App::new("prog")
            .arg(Arg::with_name("config")
                .short("c"))
            .get_matches_from(vec![
                "prog", "-c aaaaa"
            ]);
        println!("m = {:?}", m);
        assert!(m.is_present("config"));
    }
}