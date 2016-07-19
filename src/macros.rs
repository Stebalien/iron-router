/// The router macro. See the crate-level documentation.
#[macro_export]
macro_rules! router {
    (for $typ:ty; $($tree:tt)*) => {
        impl $crate::iron::Handler for $typ {
            fn handle(&self, req: &mut $crate::iron::Request) -> $crate::iron::IronResult<$crate::iron::Response> {
                // TODO: Avoid cloning this.
                let url = req.url.clone();
                let mut components = url.path().into_iter().filter(|c| !c.is_empty());
                router!(@routes self, req, components, (), {$($tree)*})
            }
        }
    };

    (@method DELETE) => {$crate::iron::method::Method::Delete};
    (@method OPTIONS) => {$crate::iron::method::Method::Options};
    (@method GET) => {$crate::iron::method::Method::Get};
    (@method POST) => {$crate::iron::method::Method::Post};
    (@method PUT) => {$crate::iron::method::Method::Put};
    (@method DELETE) => {$crate::iron::method::Method::Delete};
    (@method HEAD) => {$crate::iron::method::Method::Head};
    (@method TRACE) => {$crate::iron::method::Method::Trace};
    (@method CONNECT) => {$crate::iron::method::Method::Connect};
    (@method PATCH) => {$crate::iron::method::Method::Patch};

    (@call $s:ident, $req:ident, $func:ident, ($($vars:ident)*)) => {
        $s.$func($req, $($vars),*)
    };

    (@append_var $s:ident, $req:ident, $components:ident, $vars:tt, $var_ty:ty, $var:ident, {}) => {
        Err($crate::RouterError::new($crate::iron::status::NotFound, format!("unexpected route component '{}'", $var)).into())
    };
    (@append_var $s:ident, $req:ident, $components:ident, ($($vars:ident)*), $var_ty:ty, $var:ident, $route_sub:tt) => {{
        match $var.parse::<$var_ty>() {
            Ok(v) => router!(@routes $s, $req, $components, ($($vars)* v), $route_sub),
            Err(e) => Err($crate::RouterError::new($crate::iron::status::NotFound, e.to_string()).into()),
        }
    }};

    (@routes $s:ident, $req:ident, $components:ident, $vars:tt, {
        $(@$method:tt $func:ident;)*
        $(/$route:ident $route_sub:tt)*
        :$var_ty:ty {$($var_sub:tt)*}
    }) => {
        match $components.next() {
            None => match $req.method {
                $(router!(@method $method) => router!(@call $s, $req, $func, $vars),)*
                _ => Err($crate::RouterError::new($crate::iron::status::MethodNotAllowed, format!("no route defined for {}", $req.method)).into()),
            },
            $(Some(stringify!($route)) => router!(@routes $s, $req, $components, $vars, $route_sub),)*
            Some(var) => router!(@append_var $s, $req, $components, $vars, $var_ty, var, {$($var_sub)*}),
        }
    };
    (@routes $s:ident, $req:ident, $components:ident, $vars:tt, {
        $(@$method:ident $func:ident;)*
        $(/$route:ident $route_sub:tt)*
    }) => {
        router!(@routes $s, $req, $components, $vars, {
            $(@$method $func;)*
            $(/$route $route_sub)*
            :() {}
        })
    };
}
