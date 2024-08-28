use hivemqtt_macros::Length;

fn main() {
    
}

#[cfg(test)]
mod tests {
    use super::*;
    #[derive(Debug, Length)]
    struct MQTTProp {
        #[byte = "2"]
        name: String,
        age: Option<u8>
    }

    #[test]
    fn testing_bambi() {
        let xx = MQTTProp {name: String::from("tolumide"), age: Some(19u8)};
        println!("the xx {:#?}", xx);
        assert!(false);
    }



}