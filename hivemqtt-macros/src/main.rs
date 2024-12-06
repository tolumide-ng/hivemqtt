use hivemqtt_macros::Length;

fn main() {
    
}

#[cfg(test)]
mod tests {
    use super::*;        
        
    #[derive(Length)]
    #[allow(dead_code)]
    struct TestOne {
        title: String, // length + 2 + 1
        port: u16, // std::mem::size_of::<u16>() + 1
        properties: Vec<(String, String)>,// ....iter().map(|(k, v)| k.len() + 2 + v.len() + 2 + 1).sum::<usizee>()
        topics: Vec<String>, // ...iter().map(|k| k.len() + 2 + 1).sum::<usize>()
        active: bool, // std::mem::size_of::<bool>() + 1
        size: Vec<usize>, // variable_len(value) + 1
        ops: Option<usize>, // variable_len(value) + 1
        s: usize, // variable_len(value) + 1
    }

    #[derive(Length)]
    struct TestOptionalFields {
        title: String,
        topics: Option<Vec<String>>,
        auth: Option<Vec<(String, String)>>,
        client_id: Option<String>,
        abc: Option<bool>,
    }


    #[allow(dead_code)]
    #[derive(Length)]
    struct IgnoreField {
        title: String, // length + 2 + 1
        port: u16, // std::mem::size_of::<u16>() + 1
        #[bytes(ignore)]
        properties: Vec<(String, String)>,// ....iter().map(|(k, v)| k.len() + 2 + v.len() + 2 + 1).sum::<usizee>()
        topics: Vec<String>, // ...iter().map(|k| k.len() + 2 + 1).sum::<usize>()
        #[bytes(ignore)]
        active: bool, // std::mem::size_of::<bool>() + 1
        #[bytes(ignore)]
        client_id: Option<String>,
    }

    #[test]
    fn should_return_the_length_if_all_fields_are_populated() {
        let subject = TestOne { 
            title: "A".to_string(),  port: 440, properties: vec![(String::from("abc"), String::from("cde"))], topics: vec![String::from("topic_A")], 
            active: true, size: vec![10], ops: Some(10), s: 10 };
        assert_eq!(subject.len(), 36);
    }

    #[test]
    fn should_ignore_an_empty_vector() {
        let subject = TestOne { title: "A".to_string(),  port: 440, properties: vec![], topics: vec![], active: true, 
            size: vec![], s: 0, ops: None };
        assert_eq!(subject.len(), 11);
    }

    #[test]
    fn should_return_the_accurate_len() {
        let title_only  = TestOptionalFields{title: "testingHuman".to_string(), topics: None, auth: None, client_id: None, abc: None, };
        assert_eq!(title_only.len(), 15);
        
        let with_auth  = TestOptionalFields{title: "testingHuman".to_string(), topics: None, auth: Some(vec![("username".to_string(), "xxxx88yj".to_string())]), client_id: None, abc: Some(false), };
        assert_eq!(with_auth.len(), 38)
    }

    #[test]
    fn should_ignore_fields_with_the_ignore_attribute() {
        let subject = IgnoreField { title: "A".to_string(),  port: 440, properties: vec![(String::from("abc"), String::from("cde"))], topics: vec![String::from("topic_A")], active: true, client_id: None };
        assert_eq!(subject.len(), 17)
    }
}