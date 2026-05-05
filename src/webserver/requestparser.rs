//use std::io::error;
#[derive(Debug)]
pub struct Request<'a>{
pub method:RequestMethod,
pub url:& 'a str,
}
#[derive(Debug)]
pub enum RequestMethod{
Get,
Post
}
#[derive(Debug)]
pub struct RequestParser{
pub request:String}
impl RequestParser{

pub fn parse(&self)->Request<'_>{
let mut request = self.request
.split_whitespace(); 
//let url = request.next().next().unwrap()
match request.next().unwrap(){
"GET"=> Request{
method: RequestMethod::Get,
url: request.next().unwrap()
.split("?").next().unwrap()},
_=> Request{
method: RequestMethod::Post,
url: request.next().unwrap()}
}
}

}
