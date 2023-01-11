use sendmail::email;
use lazy_static::lazy_static;

pub struct Email {
    pub body: String,
}

lazy_static!{
    static ref FROM_ADDRESS: &'static str = "wangtao20100517@gmail.com";
    static ref TO_ADDRESS: Vec<&'static str> = vec!["576217702@qq.com"];
}

pub trait HTML {
    fn to_email_body(&self) -> String;
}

impl Email {
    pub fn new<C: ?Sized>(components: Vec<&C>) -> Self
        where C: HTML,
    {
        let mut body = String::from("");
        for c in components {
            body.push_str(&c.to_email_body());
        }

        Self {
            body,
        }
    }

    pub fn send(&self) -> Result<(), std::io::Error> {
        email::send(
            // From Address
            &FROM_ADDRESS,
            // TO Address
            <Vec<&str> as AsRef<[&str]>>::as_ref(&TO_ADDRESS.to_vec()),
            // Subject
            &format!("Personal Finance Newsletter"),
            // Body
            &self.body,
        )
    }
}