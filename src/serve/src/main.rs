use std::{
    io::prelude::*,
    net::TcpListener
};

use marketplace::Marketplace;

fn main() -> std::io::Result<()> {
    let instant = std::time::Instant::now();
    let mut marketplace = Marketplace::new();
    
    
    // for id in 0..1000000 {
    //     // marketplace.query(b"create\0name\0#'{\"header\":{\"default\":\"<This is the name of the list>\"},\"footer\":{\"default\":\"<Subtext of the list>\"},\"body\":{\"default\":\"<This is text*>\"},\"sections\":[{\"title\":{\"default\":\"<List Category Item>\"},\"rows\":[{\"id\":\"<Item ID>1\",\"title\":{\"default\":\"<Item Title>\"},\"description\":{\"default\":\"<Item Description>\"}},{\"id\":\"<Item ID>2\",\"title\":{\"default\":\"<Item Title>\"},\"description\":{\"default\":\"<Item Description>\"}},{\"id\":\"<Item ID>3\",\"title\":{\"default\":\"<Item Title>\"},\"description\":{\"default\":\"<Item Description>\"}}]}]}{\"header\":{\"default\":\"<This is the name of the list>\"},\"footer\":{\"default\":\"<Subtext of the list>\"},\"body\":{\"default\":\"<This is text*>\"},\"sections\":[{\"title\":{\"default\":\"<List Category Item>\"},\"rows\":[{\"id\":\"<Item ID>1\",\"title\":{\"default\":\"<Item Title>\"},\"description\":{\"default\":\"<Item Description>\"}},{\"id\":\"<Item ID>2\",\"title\":{\"default\":\"<Item Title>\"},\"description\":{\"default\":\"<Item Description>\"}},{\"id\":\"<Item ID>3\",\"title\":{\"default\":\"<Item Title>\"},\"description\":{\"default\":\"<Item Description>\"}}]}]}'\0");
    //     let q1 = "match\0_id\0=\0&";
    //     let mut query = q1.to_owned() + id.to_string().as_str();
    //     query = query + "\0return\0";
    //     marketplace.query(query.as_bytes());
    // }

    println!("{:?}", instant.elapsed());
    let listener = TcpListener::bind("127.0.0.1:41221").unwrap();

    for stream_result in listener.incoming() {
        match stream_result {
            Ok(mut stream) => {
                let mut read: [u8; 10000] = [0; 10000];

                match stream.read(&mut read) {
                    Ok(n) => {
                        if n == 0 {
                            break;
                        }

                        let result = marketplace.query(&read[0..n]);
                        stream.write(result.as_bytes()).unwrap();
                        
                    },
                    Err(err) => panic!("{:?}", err)
                }
                
            },
            Err(err) => panic!("{:?}", err)
        }
    }

    Ok(())
}
