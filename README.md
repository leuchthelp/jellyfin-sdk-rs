# [UNOFFICIAL] jellyfin sdk for rust

generate using the python cli of [openapi-generator-cli](https://pypi.org/project/openapi-generator-cli/)

to regenerate run:

```bash
uv run openapi-generator-cli generate -i jellyfin-openapi-<version>.json -g rust -o generated-client
```

this closely mirrors the official [jellyfin-sdk-typescript](https://github.com/jellyfin/jellyfin-sdk-typescript) and therefore might include "unrustlike" concepts due to my inexperience with rust