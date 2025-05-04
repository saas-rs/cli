pub mod saas_rs {
    pub mod user {
        pub mod v1 {
            #![allow(clippy::all)]
            include!("saas_rs.user.v1.rs");
            include!("saas_rs.user.v1.serde.rs");
        }
    }
}
