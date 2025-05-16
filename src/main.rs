wasmtime::component::bindgen!({
    path: "./wit",
    world: "kekw",
    async: true,
    with: {
        "grafbase:test/test": test,
    },
});

mod test {
    use wasmtime::component::{ComponentType, Lower};

    #[derive(Debug, ComponentType, Lower)]
    #[component(record)]
    pub struct MeowMeow<'a> {
        pub purr: &'a str,
        pub meow: &'a str,
        pub hiss: Vec<u8>,
    }

    pub trait Host: Send + ::core::marker::Send {}
    pub trait GetHost<T, D>:
        Fn(T) -> <Self as GetHost<T, D>>::Host + Send + Sync + Copy + 'static
    {
        type Host: Host + Send;
    }
    impl<F, T, D, O> GetHost<T, D> for F
    where
        F: Fn(T) -> O + Send + Sync + Copy + 'static,
        O: Host + Send,
    {
        type Host = O;
    }
    pub fn add_to_linker_get_host<T, G: for<'a> GetHost<&'a mut T, T, Host: Host + Send>>(
        _linker: &mut wasmtime::component::Linker<T>,
        _host_getter: G,
    ) -> wasmtime::Result<()>
    where
        T: Send,
    {
        Ok(())
    }
    pub fn add_to_linker<T, U>(
        linker: &mut wasmtime::component::Linker<T>,
        get: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
    ) -> wasmtime::Result<()>
    where
        U: Host + Send,
        T: Send,
    {
        add_to_linker_get_host(linker, get)
    }
    impl<_T: Host + ?Sized + Send> Host for &mut _T {}
}

fn main() {
    println!("Meow, meow!");
}
