use ibc_proto::{google::protobuf::Any, Protobuf};

use crate::prelude::*;

use core::fmt::Display;

/// Types that implement this trait are able to be converted to
/// a raw Protobuf `Any` type.
pub trait ToProto<P>: Protobuf<P>
where
    P: From<Self> + prost::Message + prost::Name + Default,
    <Self as TryFrom<P>>::Error: Display,
{
    fn type_url() -> String {
        P::type_url()
    }

    fn to_any(self) -> Any {
        Any {
            type_url: P::type_url(),
            value: self.encode_vec(),
        }
    }
}

impl<T, P> ToProto<P> for T
where
    T: Protobuf<P>,
    P: From<Self> + prost::Message + prost::Name + Default,
    <Self as TryFrom<P>>::Error: Display,
{
}
