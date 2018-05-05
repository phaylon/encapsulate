
#[macro_use] extern crate encapsulate as test_encapsulate;

type TestResult = Result<bool, i32>;
type TestOption = Option<i32>;

macro_rules! typed {
    ($ty:ty: $expr:expr) => {{
        let value: $ty = $expr;
        value
    }}
}

macro_rules! gen_tests {
    ($macro_name:ident) => {
        mod $macro_name {

            #[test]
            fn plain() {
                assert_eq!($macro_name! { 23 }, 23);
                assert_eq!($macro_name! { 23; }, ());
                assert_eq!($macro_name! {}, ());
            }

            #[test]
            fn returns() {
                assert_eq!($macro_name! { return 23 }, 23);
                assert_eq!($macro_name! { return }, ());
            }

            #[test]
            fn results() {
                assert_eq!(typed!(::TestResult: $macro_name! { Ok(true) }), Ok(true));
                assert_eq!(typed!(::TestResult: $macro_name! { Ok(Err(23)?) }), Err(23));
            }

            #[test]
            fn options() {
                assert_eq!($macro_name! { Some(23) }, Some(23));
                assert_eq!(typed!(::TestOption: $macro_name! { Some(None?) }), None);
            }

            #[test]
            fn capture() {
                let value = "foo".to_string();
                assert_eq!($macro_name! { value.as_str() }, "foo");
                assert_eq!(value.as_str(), "foo");
            }

            #[test]
            fn capture_mut() {
                let mut items = Vec::new();
                assert_eq!($macro_name! { items.push(23); items.len() }, 1);
                assert_eq!(items, vec![23]);
            }
        }
    }
}

gen_tests!(encapsulate);
gen_tests!(encapsulate_fn);
gen_tests!(encapsulate_flexible);
