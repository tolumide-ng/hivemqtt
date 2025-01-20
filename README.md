### HiveMQTT
[MQTT5.0](https://docs.oasis-open.org/mqtt/mqtt/v5.0/mqtt-v5.0.html)

### Plans
- [ ] Protocol Support
    - [ ] MQTT 3.1.1
    - [ ] MQTT 5.0 (In Progress)
    - [ ] TLS/TCP
    - [ ] IPV6
- [ ] All MQTT Packet Support (In Progress)
- [ ] Implement `Display` for `Property`
- [ ] Tests
- [ ] no_std support
- [ ] Integrate WASM for easy compiling to Javascript/Typescript/Node.js environments
- [ ] Easy internal utility for converting -> to string and vice versal (from terminal tool?) - for debugging
- [ ] Samples for easy learning
- [ ] Move bytes length validation/parsing into the trait, and update the trait's secondary properties




### Notes to users:
AsyncReadExt + AsyncWriteExt works fine with async-std, and smol runtime users. However users of tokio, would need to add [tokio-util](crates.io/crates/tokio_util) to ensure compatibility

```tokio
tokio-util = { version = "0.7.13", features= ["compat"]}
let stream = TcpStream::connect("example.com:80").await.unwrap();
let stream = stream.compat();
-->>
```


### Credits:
This crate derives heavy inspiration from:
1. 
2. 