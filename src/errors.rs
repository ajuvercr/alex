use rocket::response::{self, Responder};
use rocket::{Request};
use crate::template::Template;

use crate::util::Context;

error_chain! {
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    errors {
        TemplateError(context: Context, template: &'static str, reason: &'static str) {
            description("Error happend in a template")
            display("{}", reason)
        }

        AuthError {
            description("Username not found in database")
            display("Username not found")
        }
    }

    foreign_links {
        JsonError(serde_json::Error);
        IOError(std::io::Error);
        DatabaseError(diesel::result::Error);
    }
}


impl<'r> Responder<'r> for Error {
    fn respond_to(self, x: &Request) -> response::Result<'r> {
        let mut errors = Vec::new();
        let mut error_list = self.iter();

        if let Some(e) = error_list.next() {
            errors.push(format!("Error: {}", e));
        }

        for e in error_list {
            errors.push(format!("caused by: {}", e));
        }

        if let ErrorKind::TemplateError(c, t, _) = self.kind() {
            let c = c.clone().insert("errors", errors);
            Template::render(t.clone(), &c.inner()).respond_to(x)
        } else {
            let context = Context::new().insert("errors", errors);
            Template::render("error", &context.inner()).respond_to(x)
        }
    }
}