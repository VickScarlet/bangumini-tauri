pub mod auth;
pub mod index;
pub mod subject;

#[macro_export]
macro_rules! bangumi {
    () => { "https://bgm.tv".to_string() };
    (/) => {
        {
            let mut url = $crate::bangumi!();
            url.push('/');
            url
        }
    };
    ($last:tt) => ({
        {
            let mut url = $crate::bangumi!();
            url.push('/');
            url.push_str($last);
            url
        }
    });
    ($first:tt $(/ $tail:tt)*) => ({
        {
            let mut url = $crate::bangumi!($first);
            $(
                url.push('/');
                url.push_str(&$tail);
            )*
            url
        }
    });
}
