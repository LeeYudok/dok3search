# Module axum::extract::rejection

## Overview

Rejection response types.

## Items

- `FailedToDeserializeForm` — Rejection type used if the Form extractor is unable to deserialize the form into the target type.
- `FailedToDeserializeFormBody` — Rejection type used if the Form extractor is unable to deserialize the form body into the target type.
- `FailedToDeserializeQueryString` — Rejection type used if the Query extractor is unable to deserialize the query string into the target type.
- `InvalidFormContentType` — Rejection type for Form or RawForm used if the Content-Type header is missing or its value is not application/x-www-form-urlencoded.
- `InvalidUtf8` — Rejection type used when buffering the request into a String if the body doesn’t contain valid UTF-8.
- `JsonDataError` — Rejection type for Json.
- `JsonSyntaxError` — Rejection type for Json.
- `LengthLimitError` — Encountered some other error when buffering the body.
- `MatchedPathMissing` — Rejection if no matched path could be found.
- `MissingExtension` — Rejection type for Extension if an expected request extension was not found.
- `MissingJsonContentType` — Rejection type for Json used if the Content-Type header is missing.
- `MissingPathParams` — Rejection type used if axum’s internal representation of path parameters is missing. This is commonly caused by extracting Request<_>. Path must be extracted first.
- `NestedPathRejection` — Rejection type for NestedPath.
- `UnknownBodyError` — Encountered an unknown error when buffering the body.
- `BytesRejection` — Rejection used for Bytes.
- `ExtensionRejection` — Rejection used for Extension.
- `FailedToBufferBody` — Rejection type for extractors that buffer the request body. Used if the request body cannot be buffered due to an error.
- `FormRejection` — Rejection used for Form.
- `JsonRejection` — Rejection used for Json.
- `MatchedPathRejection` — Rejection used for MatchedPath.
- `PathRejection` — Rejection used for Path.
- `QueryRejection` — Rejection used for Query.
- `RawFormRejection` — Rejection used for RawForm.
- `RawPathParamsRejection` — Rejection used for RawPathParams.
- `StringRejection` — Rejection used for String.

### enum axum::extract::rejection::BytesRejection

```rust
#[non_exhaustive]pub enum BytesRejection {
    FailedToBufferBody(FailedToBufferBody),
}
```

Rejection used for `Bytes`.

Contains one variant for each way the `Bytes` extractor
can fail.

**Methods**

- `pub fn body_text(&self) -> String`
- `pub fn status(&self) -> StatusCode`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>`
- `fn source(&self) -> Option<&(dyn Error + 'static)>`
- `1.0.0 · fn description(&self) -> &str`
- `1.0.0 · fn cause(&self) -> Option<&dyn Error>`
- `fn provide<'a>(&'a self, request: &mut Request<'a>)`
- `fn from(inner: BytesRejection) -> Self`
- `fn from(inner: BytesRejection) -> Self`
- `fn from(inner: BytesRejection) -> Self`
- `fn from(inner: FailedToBufferBody) -> BytesRejection`
- `fn into_response(self) -> Response<Body>`

### enum axum::extract::rejection::ExtensionRejection

```rust
#[non_exhaustive]pub enum ExtensionRejection {
    MissingExtension(MissingExtension),
}
```

Rejection used for `Extension`.

Contains one variant for each way the `Extension` extractor
can fail.

**Methods**

- `pub fn body_text(&self) -> String`
- `pub fn status(&self) -> StatusCode`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn source(&self) -> Option<&(dyn Error + 'static)>`
- `1.0.0 · fn description(&self) -> &str`
- `1.0.0 · fn cause(&self) -> Option<&dyn Error>`
- `fn provide<'a>(&'a self, request: &mut Request<'a>)`
- `fn from(inner: MissingExtension) -> Self`
- `fn into_response(self) -> Response`

### enum axum::extract::rejection::FailedToBufferBody

```rust
#[non_exhaustive]pub enum FailedToBufferBody {
    LengthLimitError(LengthLimitError),
    UnknownBodyError(UnknownBodyError),
}
```

Rejection type for extractors that buffer the request body. Used if the
request body cannot be buffered due to an error.

**Methods**

- `pub fn body_text(&self) -> String`
- `pub fn status(&self) -> StatusCode`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>`
- `fn source(&self) -> Option<&(dyn Error + 'static)>`
- `1.0.0 · fn description(&self) -> &str`
- `1.0.0 · fn cause(&self) -> Option<&dyn Error>`
- `fn provide<'a>(&'a self, request: &mut Request<'a>)`
- `fn from(inner: FailedToBufferBody) -> BytesRejection`
- `fn from(inner: FailedToBufferBody) -> StringRejection`
- `fn from(inner: LengthLimitError) -> FailedToBufferBody`
- `fn from(inner: UnknownBodyError) -> FailedToBufferBody`
- `fn into_response(self) -> Response<Body>`

### enum axum::extract::rejection::FormRejection

```rust
#[non_exhaustive]pub enum FormRejection {
    InvalidFormContentType(InvalidFormContentType),
    FailedToDeserializeForm(FailedToDeserializeForm),
    FailedToDeserializeFormBody(FailedToDeserializeFormBody),
    BytesRejection(BytesRejection),
}
```

Rejection used for `Form`.

Contains one variant for each way the `Form` extractor
can fail.

**Methods**

- `pub fn body_text(&self) -> String`
- `pub fn status(&self) -> StatusCode`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn source(&self) -> Option<&(dyn Error + 'static)>`
- `1.0.0 · fn description(&self) -> &str`
- `1.0.0 · fn cause(&self) -> Option<&dyn Error>`
- `fn provide<'a>(&'a self, request: &mut Request<'a>)`
- `fn from(inner: BytesRejection) -> Self`
- `fn from(inner: FailedToDeserializeForm) -> Self`
- `fn from(inner: FailedToDeserializeFormBody) -> Self`
- `fn from(inner: InvalidFormContentType) -> Self`
- `fn into_response(self) -> Response`

### enum axum::extract::rejection::JsonRejection

```rust
#[non_exhaustive]pub enum JsonRejection {
    JsonDataError(JsonDataError),
    JsonSyntaxError(JsonSyntaxError),
    MissingJsonContentType(MissingJsonContentType),
    BytesRejection(BytesRejection),
}
```

Rejection used for `Json`.

Contains one variant for each way the `Json` extractor
can fail.

**Methods**

- `pub fn body_text(&self) -> String`
- `pub fn status(&self) -> StatusCode`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn source(&self) -> Option<&(dyn Error + 'static)>`
- `1.0.0 · fn description(&self) -> &str`
- `1.0.0 · fn cause(&self) -> Option<&dyn Error>`
- `fn provide<'a>(&'a self, request: &mut Request<'a>)`
- `fn from(inner: BytesRejection) -> Self`
- `fn from(inner: JsonDataError) -> Self`
- `fn from(inner: JsonSyntaxError) -> Self`
- `fn from(inner: MissingJsonContentType) -> Self`
- `fn into_response(self) -> Response`

### enum axum::extract::rejection::MatchedPathRejection

```rust
#[non_exhaustive]pub enum MatchedPathRejection {
    MatchedPathMissing(MatchedPathMissing),
}
```

Rejection used for `MatchedPath`.

**Methods**

- `pub fn body_text(&self) -> String`
- `pub fn status(&self) -> StatusCode`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn source(&self) -> Option<&(dyn Error + 'static)>`
- `1.0.0 · fn description(&self) -> &str`
- `1.0.0 · fn cause(&self) -> Option<&dyn Error>`
- `fn provide<'a>(&'a self, request: &mut Request<'a>)`
- `fn from(inner: MatchedPathMissing) -> Self`
- `fn into_response(self) -> Response`

### enum axum::extract::rejection::PathRejection

```rust
#[non_exhaustive]pub enum PathRejection {
    FailedToDeserializePathParams(FailedToDeserializePathParams),
    MissingPathParams(MissingPathParams),
}
```

Rejection used for `Path`.

Contains one variant for each way the `Path` extractor
can fail.

**Methods**

- `pub fn body_text(&self) -> String`
- `pub fn status(&self) -> StatusCode`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn source(&self) -> Option<&(dyn Error + 'static)>`
- `1.0.0 · fn description(&self) -> &str`
- `1.0.0 · fn cause(&self) -> Option<&dyn Error>`
- `fn provide<'a>(&'a self, request: &mut Request<'a>)`
- `fn from(inner: FailedToDeserializePathParams) -> Self`
- `fn from(inner: MissingPathParams) -> Self`
- `fn into_response(self) -> Response`

### enum axum::extract::rejection::QueryRejection

```rust
#[non_exhaustive]pub enum QueryRejection {
    FailedToDeserializeQueryString(FailedToDeserializeQueryString),
}
```

Rejection used for `Query`.

Contains one variant for each way the `Query` extractor
can fail.

**Methods**

- `pub fn body_text(&self) -> String`
- `pub fn status(&self) -> StatusCode`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn source(&self) -> Option<&(dyn Error + 'static)>`
- `1.0.0 · fn description(&self) -> &str`
- `1.0.0 · fn cause(&self) -> Option<&dyn Error>`
- `fn provide<'a>(&'a self, request: &mut Request<'a>)`
- `fn from(inner: FailedToDeserializeQueryString) -> Self`
- `fn into_response(self) -> Response`

### enum axum::extract::rejection::RawFormRejection

```rust
#[non_exhaustive]pub enum RawFormRejection {
    InvalidFormContentType(InvalidFormContentType),
    BytesRejection(BytesRejection),
}
```

Rejection used for `RawForm`.

Contains one variant for each way the `RawForm` extractor
can fail.

**Methods**

- `pub fn body_text(&self) -> String`
- `pub fn status(&self) -> StatusCode`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn source(&self) -> Option<&(dyn Error + 'static)>`
- `1.0.0 · fn description(&self) -> &str`
- `1.0.0 · fn cause(&self) -> Option<&dyn Error>`
- `fn provide<'a>(&'a self, request: &mut Request<'a>)`
- `fn from(inner: BytesRejection) -> Self`
- `fn from(inner: InvalidFormContentType) -> Self`
- `fn into_response(self) -> Response`

### enum axum::extract::rejection::RawPathParamsRejection

```rust
#[non_exhaustive]pub enum RawPathParamsRejection {
    InvalidUtf8InPathParam(InvalidUtf8InPathParam),
    MissingPathParams(MissingPathParams),
}
```

Rejection used for `RawPathParams`.

Contains one variant for each way the `RawPathParams` extractor
can fail.

**Methods**

- `pub fn body_text(&self) -> String`
- `pub fn status(&self) -> StatusCode`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn source(&self) -> Option<&(dyn Error + 'static)>`
- `1.0.0 · fn description(&self) -> &str`
- `1.0.0 · fn cause(&self) -> Option<&dyn Error>`
- `fn provide<'a>(&'a self, request: &mut Request<'a>)`
- `fn from(inner: InvalidUtf8InPathParam) -> Self`
- `fn from(inner: MissingPathParams) -> Self`
- `fn into_response(self) -> Response`

### enum axum::extract::rejection::StringRejection

```rust
#[non_exhaustive]pub enum StringRejection {
    FailedToBufferBody(FailedToBufferBody),
    InvalidUtf8(InvalidUtf8),
}
```

Rejection used for `String`.

Contains one variant for each way the `String` extractor can fail.

**Methods**

- `pub fn body_text(&self) -> String`
- `pub fn status(&self) -> StatusCode`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>`
- `fn source(&self) -> Option<&(dyn Error + 'static)>`
- `1.0.0 · fn description(&self) -> &str`
- `1.0.0 · fn cause(&self) -> Option<&dyn Error>`
- `fn provide<'a>(&'a self, request: &mut Request<'a>)`
- `fn from(inner: FailedToBufferBody) -> StringRejection`
- `fn from(inner: InvalidUtf8) -> StringRejection`
- `fn into_response(self) -> Response<Body>`

### struct axum::extract::rejection::FailedToDeserializeForm

```rust
pub struct FailedToDeserializeForm(/* private fields */);
```

Rejection type used if the `Form` extractor is unable to
deserialize the form into the target type.

**Methods**

- `pub fn body_text(&self) -> String`
- `pub fn status(&self) -> StatusCode`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn source(&self) -> Option<&(dyn Error + 'static)>`
- `1.0.0 · fn description(&self) -> &str`
- `1.0.0 · fn cause(&self) -> Option<&dyn Error>`
- `fn provide<'a>(&'a self, request: &mut Request<'a>)`
- `fn from(inner: FailedToDeserializeForm) -> Self`
- `fn into_response(self) -> Response`

### struct axum::extract::rejection::FailedToDeserializeFormBody

```rust
pub struct FailedToDeserializeFormBody(/* private fields */);
```

Rejection type used if the `Form` extractor is unable to
deserialize the form body into the target type.

**Methods**

- `pub fn body_text(&self) -> String`
- `pub fn status(&self) -> StatusCode`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn source(&self) -> Option<&(dyn Error + 'static)>`
- `1.0.0 · fn description(&self) -> &str`
- `1.0.0 · fn cause(&self) -> Option<&dyn Error>`
- `fn provide<'a>(&'a self, request: &mut Request<'a>)`
- `fn from(inner: FailedToDeserializeFormBody) -> Self`
- `fn into_response(self) -> Response`

### struct axum::extract::rejection::FailedToDeserializeQueryString

```rust
pub struct FailedToDeserializeQueryString(/* private fields */);
```

Rejection type used if the `Query` extractor is unable to
deserialize the query string into the target type.

**Methods**

- `pub fn body_text(&self) -> String`
- `pub fn status(&self) -> StatusCode`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn source(&self) -> Option<&(dyn Error + 'static)>`
- `1.0.0 · fn description(&self) -> &str`
- `1.0.0 · fn cause(&self) -> Option<&dyn Error>`
- `fn provide<'a>(&'a self, request: &mut Request<'a>)`
- `fn from(inner: FailedToDeserializeQueryString) -> Self`
- `fn into_response(self) -> Response`

### struct axum::extract::rejection::InvalidFormContentType

```rust
#[non_exhaustive]pub struct InvalidFormContentType;
```

Rejection type for `Form` or `RawForm`
used if the `Content-Type` header is missing
or its value is not `application/x-www-form-urlencoded`.

**Methods**

- `pub fn body_text(&self) -> String`
- `pub fn status(&self) -> StatusCode`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn default() -> Self`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `1.30.0 · fn source(&self) -> Option<&(dyn Error + 'static)>`
- `1.0.0 · fn description(&self) -> &str`
- `1.0.0 · fn cause(&self) -> Option<&dyn Error>`
- `fn provide<'a>(&'a self, request: &mut Request<'a>)`
- `fn from(inner: InvalidFormContentType) -> Self`
- `fn from(inner: InvalidFormContentType) -> Self`
- `fn into_response(self) -> Response`

### struct axum::extract::rejection::InvalidUtf8

```rust
pub struct InvalidUtf8(/* private fields */);
```

Rejection type used when buffering the request into a `String` if the
body doesn’t contain valid UTF-8.

**Methods**

- `pub fn body_text(&self) -> String`
- `pub fn status(&self) -> StatusCode`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>`
- `fn source(&self) -> Option<&(dyn Error + 'static)>`
- `1.0.0 · fn description(&self) -> &str`
- `1.0.0 · fn cause(&self) -> Option<&dyn Error>`
- `fn provide<'a>(&'a self, request: &mut Request<'a>)`
- `fn from(inner: InvalidUtf8) -> StringRejection`
- `fn into_response(self) -> Response<Body>`

### struct axum::extract::rejection::JsonDataError

```rust
pub struct JsonDataError(/* private fields */);
```

Rejection type for `Json`.

This rejection is used if the request body is syntactically valid JSON but couldn’t be
deserialized into the target type.

**Methods**

- `pub fn body_text(&self) -> String`
- `pub fn status(&self) -> StatusCode`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn source(&self) -> Option<&(dyn Error + 'static)>`
- `1.0.0 · fn description(&self) -> &str`
- `1.0.0 · fn cause(&self) -> Option<&dyn Error>`
- `fn provide<'a>(&'a self, request: &mut Request<'a>)`
- `fn from(inner: JsonDataError) -> Self`
- `fn into_response(self) -> Response`

### struct axum::extract::rejection::JsonSyntaxError

```rust
pub struct JsonSyntaxError(/* private fields */);
```

Rejection type for `Json`.

This rejection is used if the request body didn’t contain syntactically valid JSON.

**Methods**

- `pub fn body_text(&self) -> String`
- `pub fn status(&self) -> StatusCode`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn source(&self) -> Option<&(dyn Error + 'static)>`
- `1.0.0 · fn description(&self) -> &str`
- `1.0.0 · fn cause(&self) -> Option<&dyn Error>`
- `fn provide<'a>(&'a self, request: &mut Request<'a>)`
- `fn from(inner: JsonSyntaxError) -> Self`
- `fn into_response(self) -> Response`

### struct axum::extract::rejection::LengthLimitError

```rust
pub struct LengthLimitError(/* private fields */);
```

Encountered some other error when buffering the body.

This can only happen when you’re using [`tower_http::limit::RequestBodyLimitLayer`] or
otherwise wrapping request bodies in `http_body_util::Limited`.

**Methods**

- `pub fn body_text(&self) -> String`
- `pub fn status(&self) -> StatusCode`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>`
- `fn source(&self) -> Option<&(dyn Error + 'static)>`
- `1.0.0 · fn description(&self) -> &str`
- `1.0.0 · fn cause(&self) -> Option<&dyn Error>`
- `fn provide<'a>(&'a self, request: &mut Request<'a>)`
- `fn from(inner: LengthLimitError) -> FailedToBufferBody`
- `fn into_response(self) -> Response<Body>`

### struct axum::extract::rejection::MatchedPathMissing

```rust
#[non_exhaustive]pub struct MatchedPathMissing;
```

Rejection if no matched path could be found.

See `MatchedPath` for more details.

**Methods**

- `pub fn body_text(&self) -> String`
- `pub fn status(&self) -> StatusCode`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn default() -> Self`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `1.30.0 · fn source(&self) -> Option<&(dyn Error + 'static)>`
- `1.0.0 · fn description(&self) -> &str`
- `1.0.0 · fn cause(&self) -> Option<&dyn Error>`
- `fn provide<'a>(&'a self, request: &mut Request<'a>)`
- `fn from(inner: MatchedPathMissing) -> Self`
- `fn into_response(self) -> Response`

### struct axum::extract::rejection::MissingExtension

```rust
pub struct MissingExtension(/* private fields */);
```

Rejection type for `Extension` if an expected
request extension was not found.

**Methods**

- `pub fn body_text(&self) -> String`
- `pub fn status(&self) -> StatusCode`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn source(&self) -> Option<&(dyn Error + 'static)>`
- `1.0.0 · fn description(&self) -> &str`
- `1.0.0 · fn cause(&self) -> Option<&dyn Error>`
- `fn provide<'a>(&'a self, request: &mut Request<'a>)`
- `fn from(inner: MissingExtension) -> Self`
- `fn into_response(self) -> Response`

### struct axum::extract::rejection::MissingJsonContentType

```rust
#[non_exhaustive]pub struct MissingJsonContentType;
```

Rejection type for `Json` used if the `Content-Type`
header is missing.

**Methods**

- `pub fn body_text(&self) -> String`
- `pub fn status(&self) -> StatusCode`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn default() -> Self`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `1.30.0 · fn source(&self) -> Option<&(dyn Error + 'static)>`
- `1.0.0 · fn description(&self) -> &str`
- `1.0.0 · fn cause(&self) -> Option<&dyn Error>`
- `fn provide<'a>(&'a self, request: &mut Request<'a>)`
- `fn from(inner: MissingJsonContentType) -> Self`
- `fn into_response(self) -> Response`

### struct axum::extract::rejection::MissingPathParams

```rust
#[non_exhaustive]pub struct MissingPathParams;
```

Rejection type used if axum’s internal representation of path parameters
is missing. This is commonly caused by extracting `Request<_>`. `Path`
must be extracted first.

**Methods**

- `pub fn body_text(&self) -> String`
- `pub fn status(&self) -> StatusCode`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn default() -> Self`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `1.30.0 · fn source(&self) -> Option<&(dyn Error + 'static)>`
- `1.0.0 · fn description(&self) -> &str`
- `1.0.0 · fn cause(&self) -> Option<&dyn Error>`
- `fn provide<'a>(&'a self, request: &mut Request<'a>)`
- `fn from(inner: MissingPathParams) -> Self`
- `fn from(inner: MissingPathParams) -> Self`
- `fn into_response(self) -> Response`

### struct axum::extract::rejection::NestedPathRejection

```rust
#[non_exhaustive]pub struct NestedPathRejection;
```

Rejection type for `NestedPath`.

This rejection is used if the matched route wasn’t nested.

**Methods**

- `pub fn body_text(&self) -> String`
- `pub fn status(&self) -> StatusCode`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `fn default() -> Self`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result`
- `1.30.0 · fn source(&self) -> Option<&(dyn Error + 'static)>`
- `1.0.0 · fn description(&self) -> &str`
- `1.0.0 · fn cause(&self) -> Option<&dyn Error>`
- `fn provide<'a>(&'a self, request: &mut Request<'a>)`
- `fn into_response(self) -> Response`

### struct axum::extract::rejection::UnknownBodyError

```rust
pub struct UnknownBodyError(/* private fields */);
```

Encountered an unknown error when buffering the body.

**Methods**

- `pub fn body_text(&self) -> String`
- `pub fn status(&self) -> StatusCode`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>`
- `fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>`
- `fn source(&self) -> Option<&(dyn Error + 'static)>`
- `1.0.0 · fn description(&self) -> &str`
- `1.0.0 · fn cause(&self) -> Option<&dyn Error>`
- `fn provide<'a>(&'a self, request: &mut Request<'a>)`
- `fn from(inner: UnknownBodyError) -> FailedToBufferBody`
- `fn into_response(self) -> Response<Body>`
