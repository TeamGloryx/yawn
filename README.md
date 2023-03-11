# Yawn — a Rust UI framework that just does things differently :)

It's inspired by [Jetpack Compose](https://github.com/JetBrains/compose-multiplatform-core/tree/jb-main/compose)
and is currently *just a rewrite* of it in Rust.

## Runtime
Yawn Runtime is like Jetpack Compose Runtime, a library that lets you manage a tree of `Node`s and use them to do
anything.

## UI
Yawn UI is planned to use Skia to render pixels to some screens, not excluding phones' screens.

## Credits
- **Jetpack Compose AKA Compose Multiplatform (JetBrains Compose)** ::
  [main (docs, integrations) repo](https://github.com/JetBrains/compose-multiplatform-core) -- [code repo](https://github.com/JetBrains/compose-multiplatform-core/blob/jb-main/compose/)
- **How to use Skia together with GLFW to render a window** :: [gist](https://gist.github.com/ad8e/dd150b775ae6aa4d5cf1a092e4713add)
- ***Safe (!)* Skia bindings for Rust** :: [repo](https://github.com/rust-skia/rust-skia) -- [docs](https://rust-skia.github.io/doc/skia_safe)
### Jetpack Compose
Huge credits to the creators of Compose, not only for the work they've done, \
but for inspiring a lot of people, for example, me, \
to create tools for it, or sometimes **full frameworks** (like this one! 🙂), inspired by it.