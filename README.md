# `bounded-any`: A safe version of `Any` which works with non-'static types

Caveat: requires the implementation of `AsStatic`, providing a `'static` version
of your type. This is necessary so that we can get a `TypeId`, which is only
possible for `'static` types; even though we preserve the lifetime, we still
have to get the `TypeId` as if the type were `'static`.
