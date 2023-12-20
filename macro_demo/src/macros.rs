#[macro_export]
macro_rules! add {
    ($v1:expr,$v2:expr) => {
        $v1+$v2
    };
    ($v1:expr,$v2:expr,$typ:ty) =>{
        ($v1+$v2) as $typ
    };
    ($($element:expr),*)=>{
        0
        $(+ $element)*
    };
}
