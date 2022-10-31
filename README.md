# Simple backend

This is a simple backend to test Grafbase with Cloudflare Workers.

It is deployed on <https://testgrafbase.cowcowcow.uk/>

## Operations

### Create user

```curl
curl --request POST \
  --url 'https://testgrafbase.cowcowcow.uk/create_user?username=Viola&password=Viola1995' \
  --header 'Content-Type: text/plain' \
  --data 'Hello this is Viola' # Optionally include user's bio
```

### Update user bio

```curl
curl --request POST \
  --url 'https://testgrafbase.cowcowcow.uk/update_user_bio?username=Viola&password=Viola1995' \
  --header 'Content-Type: text/plain' \
  --data 'Drinking smoothie!'
```

### Get user bio

```curl
curl --request GET \
  --url 'https://testgrafbase.cowcowcow.uk/get_user_bio?username=Viola'
```
