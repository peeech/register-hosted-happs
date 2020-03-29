# Register hosted happs

Simple service that reads array of id's from `toml` file and updates `HAPP2HOST` KV store on cloudflare by sending json formatted request to the endpoint `/update/happ2host/` in the format:
```json
{
    "host": "HostPubKey", 
    "hhaids": [
        "QmHHAid_1",
        "QmHHAid_2",
        ...
    ]
}
```
In addition a signature of request signed with matching Host's private key is send in the header `X-HPOS-Signature`.

### Usage

register-hosted-happs < hosted-happs-list.toml 