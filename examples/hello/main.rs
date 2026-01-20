use hello::{HelloEndpointService, SayHelloInputEnvelope, SayHelloInputEnvelopeBody, mod_hel::HelloRequest};

mod hello;

#[tokio::main]
async fn main() {
    env_logger::init();

    // Use a custom - INSECURE - client to avoid certificate verification errors
    let insecure_client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();

    let h = HelloEndpointService::default().with_client(insecure_client);
    let request = SayHelloInputEnvelope {
        body: SayHelloInputEnvelopeBody {
            hello_request: HelloRequest {
                name: "John".to_string(),
            },
        },
    };
    let hi = h.say_hello(request).await.expect("can not greet");

    println!("{hi:?}");
}
