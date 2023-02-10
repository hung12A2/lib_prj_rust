/// Construct a `serde_json::Value` from a JSON literal.
///
/// ```
/// # use serde_json::json;
/// #
/// let value = json!({
///     "code": 200,
///     "success": true,
///     "payload": {
///         "features": [
///             "serde",
///             "json"
///         ]
///     }
/// });
/// ```
///
/// Biến hoặc biểu thức có thể được nhúng vào JSON. 
/// Bất kỳ kiểu dữ liệu nào được nhúng vào một phần tử mảng hoặc 
/// giá trị đối tượng phải thực hiện trait Serialize của Serde, 
/// trong khi bất kỳ kiểu dữ liệu nào được nhúng vào một khóa đối tượng phải thực hiện Into<String>. 
/// Nếu thực hiện Serialize của kiểu dữ liệu được nhúng không thành công, 
/// hoặc nếu kiểu dữ liệu được nhúng chứa 1 key với key không phải là chuỗi, 
/// macro json! sẽ panic 
/// ```
/// # use serde_json::json;
/// #
/// let code = 200;
/// let features = vec!["serde", "json"];
///
/// let value = json!({
///     "code": code,
///     "success": code == 200,
///     "payload": {
///         features[0]: features[1]
///     }
/// });
/// ```
///
/// ```
/// # use serde_json::json;
/// #
/// let value = json!([
///     "notice",
///     "the",
///     "trailing",
///     "comma -->",
/// ]);
/// ```
// json_internal macro dưới đây không thể sử dụng trực tiếp vector macro ngay lập tức
// lý do là vì nó tạo macro con bên trong nó
// Thay vì gọi vec! ở bên trong json_internal trực tiếp, ta sẽ gọi lồng thông qua
// 3 macro con dưới đây
// json_internal_vec đưa mỗi content vào trong 1 vector và trả về
#[macro_export]
#[doc(hidden)]
macro_rules! json_internal_vec {
    ($($content:tt)*) => {
        vec![$($content)*]
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! json_unexpected {
    () => {};
}

#[macro_export]
#[doc(hidden)]
macro_rules! json_expect_expr_comma {
    ($e:expr , $($tt:tt)*) => {};
}
///
#[macro_export(local_inner_macros)]
macro_rules! json {
    
    ($($json:tt)+) => {
        json_internal!($($json)+)
    };
}
///
#[macro_export(local_inner_macros)]
#[doc(hidden)]
macro_rules! json_internal {

    // mọi kiểu dữ liệu được mã hóa thành json như number, string, ...
    // đều được bao phủ bởi các macro sau đây 

    (null) => {
        $crate::Value::Null
    };

    (true) => {
        $crate::Value::Bool(true)
    };

    (false) => {
        $crate::Value::Bool(false)
    };

    ([]) => {
        $crate::Value::Array(json_internal_vec![])
    };

    ([ $($tt:tt)+ ]) => {
        $crate::Value::Array(json_internal!(@array [] $($tt)+))
    };

    ({}) => {
        $crate::Value::Object($crate::Map::new())
    };

    ({ $($tt:tt)+ }) => {
        $crate::Value::Object({
            let mut object = $crate::Map::new();
            json_internal!(@object object () ($($tt)+) ($($tt)+));
            object
        })
    };

    
    ($other:expr) => {
        $crate::to_value(&$other).unwrap()
    };
    //////////////////////////////////////////////////////////////////////////
    //  dùng để phân tích bên trong một mảng đưa vào 
    // Tạo ra một vector hợp lệ của các phần tử 
    // 
    //////////////////////////////////////////////////////////////////////////

    // Kết thúc array với dấu phẩy
    (@array [$($elems:expr,)*]) => {
        json_internal_vec![$($elems,)*]
    };

    // Kết thúc với không dấu phẩy nào
    (@array [$($elems:expr),*]) => {
        json_internal_vec![$($elems),*]
    };

    // Phần tử tiếp theo là NULL
    // đưa phần đầu vào arr 
    (@array [$($elems:expr,)*] null $($rest:tt)*) => {
        json_internal!(@array [$($elems,)* json_internal!(null)] $($rest)*)
    };

    // Phần tử tiếp theo là true
    (@array [$($elems:expr,)*] true $($rest:tt)*) => {
        json_internal!(@array [$($elems,)* json_internal!(true)] $($rest)*)
    };

    // Phần tử tiếp theo là false 
    (@array [$($elems:expr,)*] false $($rest:tt)*) => {
        json_internal!(@array [$($elems,)* json_internal!(false)] $($rest)*)
    };

    // Phần tử tiếp theo là 1 array nữa
    (@array [$($elems:expr,)*] [$($array:tt)*] $($rest:tt)*) => {
        json_internal!(@array [$($elems,)* json_internal!([$($array)*])] $($rest)*)
    };

    // Phần tử tiếp theo là map 
    (@array [$($elems:expr,)*] {$($map:tt)*} $($rest:tt)*) => {
        json_internal!(@array [$($elems,)* json_internal!({$($map)*})] $($rest)*)
    };

    // Phần tử tiếp theo là 1 biểu thức, theo sau dấu phẩy
    (@array [$($elems:expr,)*] $next:expr, $($rest:tt)*) => {
        json_internal!(@array [$($elems,)* json_internal!($next),] $($rest)*)
    };

    // Phần tử cuối là 1 biểu thức, không có dấu phẩy ở cuối 
    (@array [$($elems:expr,)*] $last:expr) => {
        json_internal!(@array [$($elems,)* json_internal!($last)])
    };

    (@array [$($elems:expr),*] , $($rest:tt)*) => {
        json_internal!(@array [$($elems,)*] $($rest)*)
    };

    // Mã thông báo không mong đợi, nằm sau phần tử gần đây nhất 
    (@array [$($elems:expr),*] $unexpected:tt $($rest:tt)*) => {
        json_unexpected!($unexpected)
    };

    //////////////////////////////////////////////////////////////////////////
    // TT muncher for parsing the inside of an object {...}. Each entry is
    // inserted into the given map variable.
    //
    // Must be invoked as: json_internal!(@object $map () ($($tt)*) ($($tt)*))
    //
    // We require two copies of the input tokens so that we can match on one
    // copy and trigger errors on the other copy.
    //////////////////////////////////////////////////////////////////////////

    // Done.
    (@object $object:ident () () ()) => {};

    // Chèn mục hiện tại theo sau bởi dấu phẩy cuối
    (@object $object:ident [$($key:tt)+] ($value:expr) , $($rest:tt)*) => {
        let _ = $object.insert(($($key)+).into(), $value);
        json_internal!(@object $object () ($($rest)*) ($($rest)*));
    };

    // Mục hiện tại theo sau bởi mã k mong đợi
    // Không chèn gì hết 
    (@object $object:ident [$($key:tt)+] ($value:expr) $unexpected:tt $($rest:tt)*) => {
        json_unexpected!($unexpected);
    };

    // Chèn phần tử cuối mà không có dấu phẩy cuối 
    (@object $object:ident [$($key:tt)+] ($value:expr)) => {
        let _ = $object.insert(($($key)+).into(), $value);
    };

    // Value của phần tử tiếp theo là null
    (@object $object:ident ($($key:tt)+) (: null $($rest:tt)*) $copy:tt) => {
        json_internal!(@object $object [$($key)+] (json_internal!(null)) $($rest)*);
    };

    // Value của phần tử tiếp theo là true 
    (@object $object:ident ($($key:tt)+) (: true $($rest:tt)*) $copy:tt) => {
        json_internal!(@object $object [$($key)+] (json_internal!(true)) $($rest)*);
    };

    // Value của phần tử tiếp theo là false 
    (@object $object:ident ($($key:tt)+) (: false $($rest:tt)*) $copy:tt) => {
        json_internal!(@object $object [$($key)+] (json_internal!(false)) $($rest)*);
    };

    // Value của phần tử tiếp theo là 1 array 
    (@object $object:ident ($($key:tt)+) (: [$($array:tt)*] $($rest:tt)*) $copy:tt) => {
        json_internal!(@object $object [$($key)+] (json_internal!([$($array)*])) $($rest)*);
    };

    // Value của phần tử tiếp theo là map 
    (@object $object:ident ($($key:tt)+) (: {$($map:tt)*} $($rest:tt)*) $copy:tt) => {
        json_internal!(@object $object [$($key)+] (json_internal!({$($map)*})) $($rest)*);
    };

    // Giá trị tiếp theo là một biểu thức theo sau bởi dấu phẩy.
    (@object $object:ident ($($key:tt)+) (: $value:expr , $($rest:tt)*) $copy:tt) => {
        json_internal!(@object $object [$($key)+] (json_internal!($value)) , $($rest)*);
    };

    // Last value is an expression with no trailing comma.
    (@object $object:ident ($($key:tt)+) (: $value:expr) $copy:tt) => {
        json_internal!(@object $object [$($key)+] (json_internal!($value)));
    };

    // Thiếu giá trị cho mục cuối cùng
    (@object $object:ident ($($key:tt)+) (:) $copy:tt) => {
        // "unexpected end of macro invocation"
        json_internal!();
    };

    // Thiếu dấu hai chấm giữa key:value
    // Thiếu giá trị của value, báo lỗi 
    (@object $object:ident ($($key:tt)+) () $copy:tt) => {
        // "unexpected end of macro invocation"
        json_internal!();
    };

    // Đặt nhầm dấu :
    // Báo lỗi 
    (@object $object:ident () (: $($rest:tt)*) ($colon:tt $($copy:tt)*)) => {
        // Takes no arguments so "no rules expected the token `:`".
        json_unexpected!($colon);
    };

    // Tìm thấy dấu phẩy đặt trong mã, báo lỗi
    // key,value ? 
    (@object $object:ident ($($key:tt)*) (, $($rest:tt)*) ($comma:tt $($copy:tt)*)) => {
        // Takes no arguments so "no rules expected the token `,`".
        json_unexpected!($comma);
    };

    // Khóa được đặt trong ngoặc đơn hoàn toàn 
    (@object $object:ident () (($key:expr) : $($rest:tt)*) $copy:tt) => {
        json_internal!(@object $object ($key) (: $($rest)*) (: $($rest)*));
    };

    // Từ chối việc push dấu : vào biểu thức 
    (@object $object:ident ($($key:tt)*) (: $($unexpected:tt)+) $copy:tt) => {
        json_expect_expr_comma!($($unexpected)+);
    };

    // Munch a token into the current key.
    (@object $object:ident ($($key:tt)*) ($tt:tt $($rest:tt)*) $copy:tt) => {
        json_internal!(@object $object ($($key)* $tt) ($($rest)*) ($($rest)*));
    };

    //////////////////////////////////////////////////////////////////////////
    // The main implementation.
    //
    // Must be invoked as: json_internal!($($json)+)
    //////////////////////////////////////////////////////////////////////////

    
}

