use crate::typed_api::booleans::parameters::BooleanParameterSet;
use crate::typed_api::booleans::client_key::GenericBoolClientKey;

use serde::{Deserialize, Serialize};

#[cfg_attr(doc, cfg(feature = "boolean"))]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GenericBoolPublicKey<P>
    where
        P: BooleanParameterSet,
{
    pub(in crate::typed_api::booleans) key: crate::boolean::public_key::PublicKey,
    _marker: std::marker::PhantomData<P>,
}

impl<P> GenericBoolPublicKey<P> 
where P: BooleanParameterSet {
    pub fn new(client_key: &GenericBoolClientKey<P>) -> Self {
        let key = crate::boolean::public_key::PublicKey::new(&client_key.key);
        Self {
            key,
            _marker: Default::default(),
        }
    }
}
