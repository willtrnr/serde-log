use serde::de;
use std::{fmt, ops};

#[inline]
pub fn deserialize<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: de::Deserializer<'de>,
    T: de::Deserialize<'de>,
{
    T::deserialize(Deserializer::new(deserializer))
}

pub struct Deserializer;

impl Deserializer {
    #[inline]
    pub fn new<'de, D>(deserializer: D) -> Wrapper<D>
    where
        D: de::Deserializer<'de>,
    {
        Wrapper::new(deserializer)
    }
}

pub struct Wrapper<A> {
    inner: A,
    depth: u32,
}

impl<A> Wrapper<A> {
    #[inline]
    pub(crate) fn new(inner: A) -> Self {
        Self { inner, depth: 0 }
    }

    #[inline]
    fn sub_wrap<B>(&self, inner: B) -> Wrapper<B> {
        Wrapper {
            inner,
            depth: self.depth + 1,
        }
    }

    #[inline]
    fn unwrap(self) -> A {
        self.inner
    }
}

impl<T> Default for Wrapper<T>
where
    T: Default,
{
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl<T> Clone for Wrapper<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            depth: self.depth,
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.inner = source.inner.clone();
        self.depth = source.depth;
    }
}

impl<T> fmt::Debug for Wrapper<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Wrapper")
            .field("inner", &self.inner)
            .field("depth", &self.depth)
            .finish()
    }
}

impl<T> ops::Deref for Wrapper<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> ops::DerefMut for Wrapper<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T> From<T> for Wrapper<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<'de, T> de::Deserialize<'de> for Wrapper<T>
where
    T: de::Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        log::trace!("Deserialize: deserialize");
        T::deserialize(Wrapper::new(deserializer)).map(Wrapper::new)
    }
}

impl<'de, D> de::Deserializer<'de> for Wrapper<D>
where
    D: de::Deserializer<'de>,
{
    type Error = D::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        log::trace!("Deserializer: deserialize_any");
        let visitor = self.sub_wrap(visitor);
        self.inner.deserialize_any(visitor)
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        log::trace!("Deserializer: deserialize_bool");
        let visitor = self.sub_wrap(visitor);
        self.inner.deserialize_bool(visitor)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        log::trace!("Deserializer: deserialize_i8");
        let visitor = self.sub_wrap(visitor);
        self.inner.deserialize_i8(visitor)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        log::trace!("Deserializer: deserialize_i16");
        let visitor = self.sub_wrap(visitor);
        self.inner.deserialize_i16(visitor)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        log::trace!("Deserializer: deserialize_i32");
        let visitor = self.sub_wrap(visitor);
        self.inner.deserialize_i32(visitor)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        log::trace!("Deserializer: deserialize_i64");
        let visitor = self.sub_wrap(visitor);
        self.inner.deserialize_i64(visitor)
    }

    serde::serde_if_integer128! {
        fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'de>,
        {
            log::trace!("Deserializer: deserialize_i128");
            let visitor = self.sub_wrap(visitor);
            self.inner.deserialize_i128(visitor)
        }
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        log::trace!("Deserializer: deserialize_u8");
        let visitor = self.sub_wrap(visitor);
        self.inner.deserialize_u8(visitor)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        log::trace!("Deserializer: deserialize_u16");
        let visitor = self.sub_wrap(visitor);
        self.inner.deserialize_u16(visitor)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        log::trace!("Deserializer: deserialize_u32");
        let visitor = self.sub_wrap(visitor);
        self.inner.deserialize_u32(visitor)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        log::trace!("Deserializer: deserialize_u64");
        let visitor = self.sub_wrap(visitor);
        self.inner.deserialize_u64(visitor)
    }

    serde::serde_if_integer128! {
        fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'de>,
        {
            log::trace!("Deserializer: deserialize_u128");
            let visitor = self.sub_wrap(visitor);
            self.inner.deserialize_u128(visitor)
        }
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        log::trace!("Deserializer: deserialize_f32");
        let visitor = self.sub_wrap(visitor);
        self.inner.deserialize_f32(visitor)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        log::trace!("Deserializer: deserialize_f64");
        let visitor = self.sub_wrap(visitor);
        self.inner.deserialize_f64(visitor)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        log::trace!("Deserializer: deserialize_char");
        let visitor = self.sub_wrap(visitor);
        self.inner.deserialize_char(visitor)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        log::trace!("Deserializer: deserialize_str");
        let visitor = self.sub_wrap(visitor);
        self.inner.deserialize_str(visitor)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        log::trace!("Deserializer: deserialize_string");
        let visitor = self.sub_wrap(visitor);
        self.inner.deserialize_string(visitor)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        log::trace!("Deserializer: deserialize_bytes");
        let visitor = self.sub_wrap(visitor);
        self.inner.deserialize_bytes(visitor)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        log::trace!("Deserializer: deserialize_byte_buf");
        let visitor = self.sub_wrap(visitor);
        self.inner.deserialize_byte_buf(visitor)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        log::trace!("Deserializer: deserialize_option");
        let visitor = self.sub_wrap(visitor);
        self.inner.deserialize_option(visitor)
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        log::trace!("Deserializer: deserialize_unit");
        let visitor = self.sub_wrap(visitor);
        self.inner.deserialize_unit(visitor)
    }

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        log::trace!("Deserializer: deserialize_unit_struct");
        let visitor = self.sub_wrap(visitor);
        self.inner.deserialize_unit_struct(name, visitor)
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        log::trace!("Deserializer: deserialize_newtype_struct");
        let visitor = self.sub_wrap(visitor);
        self.inner.deserialize_newtype_struct(name, visitor)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        log::trace!("Deserializer: deserialize_seq");
        let visitor = self.sub_wrap(visitor);
        self.inner.deserialize_seq(visitor)
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        log::trace!("Deserializer: deserialize_tuple");
        let visitor = self.sub_wrap(visitor);
        self.inner.deserialize_tuple(len, visitor)
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        log::trace!("Deserializer: deserialize_tuple_struct");
        let visitor = self.sub_wrap(visitor);
        self.inner.deserialize_tuple_struct(name, len, visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        log::trace!("Deserializer: deserialize_map");
        let visitor = self.sub_wrap(visitor);
        self.inner.deserialize_map(visitor)
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        log::trace!("Deserializer: deserialize_struct");
        let visitor = self.sub_wrap(visitor);
        self.inner.deserialize_struct(name, fields, visitor)
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        log::trace!("Deserializer: deserialize_enum");
        let visitor = self.sub_wrap(visitor);
        self.inner.deserialize_enum(name, variants, visitor)
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        log::trace!("Deserializer: deserialize_identifier");
        let visitor = self.sub_wrap(visitor);
        self.inner.deserialize_identifier(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        log::trace!("Deserializer: deserialize_ignored_any");
        let visitor = self.sub_wrap(visitor);
        self.inner.deserialize_ignored_any(visitor)
    }
}

impl<'de, V> de::Visitor<'de> for Wrapper<V>
where
    V: de::Visitor<'de>,
{
    type Value = V::Value;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.inner.expecting(formatter)
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        log::trace!("Visitor: visit_bool({:?})", v);
        self.inner.visit_bool(v)
    }

    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        log::trace!("Visitor: visit_i8({:?})", v);
        self.inner.visit_i8(v)
    }

    fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        log::trace!("Visitor: visit_i16({:?})", v);
        self.inner.visit_i16(v)
    }

    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        log::trace!("Visitor: visit_i32({:?})", v);
        self.inner.visit_i32(v)
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        log::trace!("Visitor: visit_i64({:?})", v);
        self.inner.visit_i64(v)
    }

    serde::serde_if_integer128! {
        fn visit_i128<E>(self, v: i128) -> Result<Self::Value,E>
        where
            E: de::Error,
        {
            log::trace!("Visitor: visit_i128({:?})", v);
            self.inner.visit_i128(v)
        }
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        log::trace!("Visitor: visit_u8({:?})", v);
        self.inner.visit_u8(v)
    }

    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        log::trace!("Visitor: visit_u16({:?})", v);
        self.inner.visit_u16(v)
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        log::trace!("Visitor: visit_u32({:?})", v);
        self.inner.visit_u32(v)
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        log::trace!("Visitor: visit_u64({:?})", v);
        self.inner.visit_u64(v)
    }

    serde::serde_if_integer128! {
        fn visit_u128<E>(self, v: u128) -> Result<Self::Value,E>
        where
            E: de::Error,
        {
            log::trace!("Visitor: visit_u128({:?})", v);
            self.inner.visit_u128(v)
        }
    }

    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        log::trace!("Visitor: visit_f32({:?})", v);
        self.inner.visit_f32(v)
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        log::trace!("Visitor: visit_f64({:?})", v);
        self.inner.visit_f64(v)
    }

    fn visit_char<E>(self, v: char) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        log::trace!("Visitor: visit_char({:?})", v);
        self.inner.visit_char(v)
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        log::trace!("Visitor: visit_str({:?})", v);
        self.inner.visit_str(v)
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        log::trace!("Visitor: visit_borrowed_str({:?})", v);
        self.inner.visit_borrowed_str(v)
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        log::trace!("Visitor: visit_string({:?})", v);
        self.inner.visit_string(v)
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        log::trace!("Visitor: visit_bytes({:?})", v);
        self.inner.visit_bytes(v)
    }

    fn visit_borrowed_bytes<E>(self, v: &'de [u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        log::trace!("Visitor: visit_borrowed_bytes({:?})", v);
        self.inner.visit_borrowed_bytes(v)
    }

    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        log::trace!("Visitor: visit_byte_buf({:?})", v);
        self.inner.visit_byte_buf(v)
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        log::trace!("Visitor: visit_none()");
        self.inner.visit_none()
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        log::trace!("Visitor: visit_some(...)");
        let deserializer = self.sub_wrap(deserializer);
        self.inner.visit_some(deserializer)
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        log::trace!("Visitor: visit_unit()");
        self.inner.visit_unit()
    }

    fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        log::trace!("Visitor: visit_newtype_struct(...)");
        let deserializer = self.sub_wrap(deserializer);
        self.inner.visit_newtype_struct(deserializer)
    }

    fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
    where
        A: de::SeqAccess<'de>,
    {
        log::trace!("Visitor: visit_seq(...)");
        let seq = self.sub_wrap(seq);
        self.inner.visit_seq(seq)
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        log::trace!("Visitor: visit_map(...)");
        let map = self.sub_wrap(map);
        self.inner.visit_map(map)
    }

    fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
    where
        A: de::EnumAccess<'de>,
    {
        log::trace!("Visitor: visit_enum(...)");
        let data = self.sub_wrap(data);
        self.inner.visit_enum(data)
    }
}

impl<'de, S> de::DeserializeSeed<'de> for Wrapper<S>
where
    S: de::DeserializeSeed<'de>,
{
    type Value = S::Value;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        log::trace!("DeserializeSeed: deserialize");
        let deserializer = self.sub_wrap(deserializer);
        self.inner.deserialize(deserializer)
    }
}

impl<'de, A> de::SeqAccess<'de> for Wrapper<A>
where
    A: de::SeqAccess<'de>,
{
    type Error = A::Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        log::trace!("SeqAccess: next_element_seed");
        let seed = self.sub_wrap(seed);
        self.inner.next_element_seed(seed)
    }

    fn next_element<T>(&mut self) -> Result<Option<T>, Self::Error>
    where
        T: de::Deserialize<'de>,
    {
        log::trace!("SeqAccess: next_element");
        self.inner
            .next_element::<Wrapper<T>>()
            .map(|v| v.map(Wrapper::unwrap))
    }

    fn size_hint(&self) -> Option<usize> {
        self.inner.size_hint()
    }
}

impl<'de, A> de::MapAccess<'de> for Wrapper<A>
where
    A: de::MapAccess<'de>,
{
    type Error = A::Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: de::DeserializeSeed<'de>,
    {
        log::trace!("MapAccess: next_key_seed");
        let seed = self.sub_wrap(seed);
        self.inner.next_key_seed(seed)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        log::trace!("MapAccess: next_value_seed");
        let seed = self.sub_wrap(seed);
        self.inner.next_value_seed(seed)
    }

    fn next_entry_seed<K, V>(
        &mut self,
        kseed: K,
        vseed: V,
    ) -> Result<Option<(K::Value, V::Value)>, Self::Error>
    where
        K: de::DeserializeSeed<'de>,
        V: de::DeserializeSeed<'de>,
    {
        log::trace!("MapAccess: next_entry_seed");
        let kseed = self.sub_wrap(kseed);
        let vseed = self.sub_wrap(vseed);
        self.inner.next_entry_seed(kseed, vseed)
    }

    fn next_key<K>(&mut self) -> Result<Option<K>, Self::Error>
    where
        K: de::Deserialize<'de>,
    {
        log::trace!("MapAccess: next_key");
        self.inner
            .next_key::<Wrapper<K>>()
            .map(|v| v.map(Wrapper::unwrap))
    }

    fn next_value<V>(&mut self) -> Result<V, Self::Error>
    where
        V: de::Deserialize<'de>,
    {
        log::trace!("MapAccess: next_value");
        self.inner.next_value::<Wrapper<V>>().map(Wrapper::unwrap)
    }

    fn next_entry<K, V>(&mut self) -> Result<Option<(K, V)>, Self::Error>
    where
        K: de::Deserialize<'de>,
        V: de::Deserialize<'de>,
    {
        log::trace!("MapAccess: next_entry");
        self.inner
            .next_entry::<Wrapper<K>, Wrapper<V>>()
            .map(|r| r.map(|(k, v)| (k.unwrap(), v.unwrap())))
    }

    fn size_hint(&self) -> Option<usize> {
        self.inner.size_hint()
    }
}

impl<'de, A> de::EnumAccess<'de> for Wrapper<A>
where
    A: de::EnumAccess<'de>,
{
    type Error = A::Error;
    type Variant = A::Variant;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        log::trace!("EnumAccess: variant_seed");
        let seed = self.sub_wrap(seed);
        self.inner.variant_seed(seed)
    }

    fn variant<V>(self) -> Result<(V, Self::Variant), Self::Error>
    where
        V: de::Deserialize<'de>,
    {
        log::trace!("EnumAccess: variant");
        self.inner
            .variant::<Wrapper<V>>()
            .map(|(k, v)| (k.unwrap(), v))
    }
}
