#[macro_export]
macro_rules! build_message {
    (  $x:expr  ) => {{
        let iter = $x.into_iter();

        let mut great_greet = "".to_string();
        for greet in iter {
            let g = greet.hi();
            great_greet.push_str("\n");
            great_greet.push_str(&g);
        }

        great_greet.to_string()
    }};
}
