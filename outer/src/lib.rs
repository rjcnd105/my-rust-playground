// 네임스페이싱, pub
// pub 으로 공개하지 않으면 외부에서 사용 불가.
// super::로 상위 접근 가능
// super::* 로 상위의 모든 것을 가져옴

mod outermost {
    // 같은 파일에 선언된 것은
    // 이 내부 입장에서 상위 모듈이므로 super:: 사용
    use super::hi;

    pub fn middle_function() {}

    fn middle_secret_function() {}

    pub mod inside {
        pub fn inner_function() {}

        fn secret_function() {}
    }

    fn access_secret_function() {
        middle_secret_function()
    }

    fn say_hi(){
        hi("hoejun")
    }
}

fn try_me() {
    outermost::middle_function();
    outermost::middle_secret_function();
    outermost::inside::inner_function();
    outermost::inside::secret_function();
}

pub fn hi(name: &str) {
    print!("hi {}", name);
}