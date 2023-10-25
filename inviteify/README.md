# Inviteify

A rust library that can be used to manage discord invite links.

## Config

The authentication details can be generated via the [discord application portal](https://discord.com/developers/applications?new_application=true).

See [the discord docs](https://discord.com/developers/docs/getting-started) for more info.

`inviteify.toml` (see `.inviteify.toml.sample`):

```toml
[auth]
application_id = ""
public_key = ""
client_id = ""
```

`.env` (see `.env.sample`)

```env
INVITEIFY_CLIENT_SECRET=some_secret_here
```
