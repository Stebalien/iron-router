A simple router macro for iron that implements `iron::Handler` given a
description of the desired routes.

## Usage

```rust
#[macro_use]
extern crate iron_router;
extern crate iron;

use iron::{Request, Response, IronResult};

struct MyServer;

// Define a set of route handlers (views).
impl MyServer {
    fn index(&self, req: &mut Request) -> IronResult<Response> {
        unimplemented!()
    }
    fn contacts(&self, req: &mut Request) -> IronResult<Response> {
        unimplemented!()
    }
    fn search_contacts(&self, req: &mut Request) -> IronResult<Response> {
        unimplemented!()
    }
    fn add_contact(&self, req: &mut Request) -> IronResult<Response> {
        unimplemented!()
    }
    fn get_contact(&self, req: &mut Request, id: u64) -> IronResult<Response> {
        unimplemented!()
    }
    fn replace_contact(&self, req: &mut Request, id: u64) -> IronResult<Response> {
        unimplemented!()
    }
    fn update_contact(&self, req: &mut Request, id: u64) -> IronResult<Response> {
        unimplemented!()
    }
    fn delete_contact(&self, req: &mut Request, id: u64) -> IronResult<Response> {
        unimplemented!()
    }
}

router! {
    for MyServer;

    @GET index;

    /contacts {
        @GET contacts;
        @POST add_contact;

        /* tries matching this first */
        /search {
            @GET search_contacts;
        }

        /* One variable branch allowed per level. Matches: `/contacts/:id` */
        :u64 /* id */ {
            @GET get_contact;
            @PUT replace_contact;
            @PATCH update_contact;
            @DELETE delete_contact;
        }
    }
}
```

To give you a picture of how this works under the covers, here's the generated
`Handler` implementation (slightly reformatted for readability):

```rust
impl ::iron::Handler for MyServer {
    fn handle(&self, req: &mut ::iron::Request) -> ::iron::IronResult<::iron::Response> {
        let url = req.url.clone();
        let mut components = url.path().into_iter().filter(|c| !c.is_empty());
        match components.next() {
            None => match req.method {
                ::iron::method::Method::Get => self.index(req),
                _ => Ok(::iron::Response::with(::iron::status::MethodNotAllowed)),
            },
            Some("contacts") => match components.next() {
                None => match req.method {
                    ::iron::method::Method::Get => self.contacts(req),
                    ::iron::method::Method::Post => self.add_contact(req),
                    _ => Ok(::iron::Response::with(::iron::status::MethodNotAllowed)),
                },
                Some("search") => match components.next() {
                    None => match req.method {
                        ::iron::method::Method::Get => self.search_contacts(req),
                        _ => Ok(::iron::Response::with(::iron::status::MethodNotAllowed)),
                    },
                    Some(var) => {
                        drop(var);
                        Ok(Response::with(::iron::status::Status::NotFound))
                    }
                },
                Some(var) => match var.parse::<u64>() {
                    Ok(v) => match components.next() {
                        None => match req.method {
                            ::iron::method::Method::Get => self.get_contact(req, v),
                            ::iron::method::Method::Put => self.replace_contact(req, v),
                            ::iron::method::Method::Patch => self.update_contact(req, v),
                            ::iron::method::Method::Delete => self.delete_contact(req, v),
                            _ => Ok(::iron::Response::with(::iron::status::MethodNotAllowed)),
                        },
                        Some(var) => {
                            drop(var);
                            Ok(Response::with(::iron::status::Status::NotFound))
                        }
                    },
                    Err(e) => Ok(Response::with((::iron::status::NotFound, e.to_string()))),
                },
            },
            Some(var) => {
                drop(var);
                Ok(Response::with(::iron::status::Status::NotFound))
            }
        }
    }
}
```
