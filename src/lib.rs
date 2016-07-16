//! Iron router macro.
//! 
//! # Example
//!
//! ```rust
//! #[macro_use]
//! extern crate iron_router;
//! extern crate iron;
//!
//! use iron::{Request, Response, IronResult};
//!
//! struct MyServer;
//!
//! // Define a set of route handlers (views).
//! impl MyServer {
//!     fn index(&self, req: &mut Request) -> IronResult<Response> {
//!         unimplemented!()
//!     }
//!     fn contacts(&self, req: &mut Request) -> IronResult<Response> {
//!         unimplemented!()
//!     }
//!     fn search_contacts(&self, req: &mut Request) -> IronResult<Response> {
//!         unimplemented!()
//!     }
//!     fn add_contact(&self, req: &mut Request) -> IronResult<Response> {
//!         unimplemented!()
//!     }
//!     fn get_contact(&self, req: &mut Request, id: u64) -> IronResult<Response> {
//!         unimplemented!()
//!     }
//!     fn replace_contact(&self, req: &mut Request, id: u64) -> IronResult<Response> {
//!         unimplemented!()
//!     }
//!     fn update_contact(&self, req: &mut Request, id: u64) -> IronResult<Response> {
//!         unimplemented!()
//!     }
//!     fn delete_contact(&self, req: &mut Request, id: u64) -> IronResult<Response> {
//!         unimplemented!()
//!     }
//! }
//!
//! router! {
//!     for MyServer;
//!
//!     @GET index;
//!
//!     /contacts {
//!         @GET contacts;
//!         @POST add_contact;
//!
//!         /* tries matching this first */
//!         /search {
//!             @GET search_contacts;
//!         }
//!
//!         /* One variable block allowed. Matches: `/contacts/:id` */
//!         :u64 /* id */ {
//!             @GET get_contact;
//!             @PUT replace_contact;
//!             @PATCH update_contact;
//!             @DELETE delete_contact;
//!         }
//!     }
//! }
//! # fn main() {}
//! ```

#[macro_use]
mod macros;
