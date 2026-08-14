# [UNOFFICIAL] jellyfin sdk for rust

generate using the python cli of [openapi-generator-cli](https://pypi.org/project/openapi-generator-cli/)

to regenerate run:

```bash
uv run openapi-generator-cli generate -i jellyfin-openapi-<version>.json -g rust -o generated-client --config openapi-config.yaml
```

## Usage

simply use the sdk with:

```rs
use jellyfin_sdk_rs::<option>
```

the sdk reexports the `apis` & `models` found in the [generated-client](generated-client) and adds simple helper function as:

```rs
use jellyfin_sdk_rs::configure;

fn config() {
    let config = configure(
            base_url,
            client_info,
            device_info,
            access_token,
            basic_auth,
            oauth_access_token,
            bearer_access_token,
            api_key,
        );
}
```

which can then be used to authenticate with the server:

```rs
use jellyfin_sdk_rs::apis::authentication_api::authenticate_user_by_name;

fn auth() {
    authenticate_user_by_name(config, authenticate_user_by_name)
}
```

After you authenticate with the server & retrieve your `auth_token` from the [reqwest response](https://docs.rs/reqwest/latest/reqwest/struct.Response.html) you will have to create a new configuration as a [reqwest client](https://docs.rs/reqwest/latest/reqwest/struct.Client.html) is immutable after creation & headers cannot be added to the default client configuration after the fact.
