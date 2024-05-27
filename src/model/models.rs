use serde::{Serialize, Deserialize};



#[derive(Serialize, Deserialize)]
pub struct Body<T>{
    pub value: T
}

#[derive(Serialize, Deserialize)]
pub struct Response<T>{
   pub protocol: String,
   pub version: String,
   pub status: String,
   pub headers: Vec<Header>,
   pub body: Body<T>
}


impl<T> Response<T> where T:Serialize{

    pub fn ToString(self: Self) -> String{
        let mut result: String = String::new();
        let mut fitstLine = self.protocol;
        let mut secondLine = String::new();
        fitstLine.push_str("/");
        fitstLine.push_str(&self.version);
        fitstLine.push_str(" ");
        fitstLine.push_str(&self.status);
        result.push_str(&fitstLine);
        result.push_str("\r\n");
        for header in self.headers{
            secondLine.push_str(&header.to_string())
        }
        secondLine.push_str("\r\n");
        result.push_str(&secondLine);
        let json = serde_json::to_string(&self.body).unwrap();
        result.push_str(&json);

        return result;
    }

}


#[derive(Serialize, Deserialize)]
pub struct Header {
    pub key: String,
    pub value: String
}

impl Header {
    pub fn new(key: String, value: String) -> Self { 
        let mut nvalue : String = value ;
        nvalue.push_str("\r\n");
        let mut nkey : String = key;
        nkey.push_str(":");
        return Self { 
            key: nkey,
            value: nvalue,
        };
    }

    pub fn to_string(self: Self) -> String{
        let mut header = self.key;
        header.push_str(&self.value);
        return header;
    }
}