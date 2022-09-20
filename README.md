# url-shortener

A small daemon that acts as a URL shortener.

## Use Case

I worked for a company that had a similar internal tool where employees would
type in their browser `go/<shortcode>` (ex: `go/jira`) and be forwarded to the
appropriate URL. I always found this useful and [@bahamas10] wrote us a similar
[service] many years ago. I wanted to rewrite this in rust as an excuse to learn
more about [warp], and because I find it easier to maintain rust services on my
home infrastructure compared to nodejs.

[service]: https://github.com/bahamas10/node-url-shortener
[@bahamas10]: https://github.com/bahamas10
[warp]: https://docs.rs/warp/


## Example Config

```toml
[server]
host = "0.0.0.0"
port = 8080
# number of worker threads handling requests
threads = 1

[index]
title = "my website"
text_color = "white"
bg_color_top = "black"
bg_color_bottom = "#157a19"

[urls]
gh = "https://github.com"
yt = "https://youtube.com"
```

## Example Usage

The program looks for a toml config passed as the first argument or defaults to
`config.toml` in the current working directory.

### Server Side

```
$ url-shortener config.toml
Feb 28 18:55:05.536 INFO 127.0.0.1 GET "/" 200 curl/7.75.0 (248.727µs)
Feb 28 18:55:14.880 INFO 127.0.0.1 GET "/gh" 307 curl/7.75.0 (232.817µs)
Feb 28 18:55:21.498 INFO 127.0.0.1 GET "/yt" 307 curl/7.75.0 (139.424µs)
Feb 28 18:55:26.705 INFO 127.0.0.1 GET "/dne" 404 curl/7.75.0 (105.353µs)

```

### Client Side

```
$ curl -i localhost:8080/gh
HTTP/1.1 307 Temporary Redirect
location: https://github.com/
content-length: 0
date: Sun, 28 Feb 2021 18:55:14 GMT

$ curl -i localhost:8080/yt
HTTP/1.1 307 Temporary Redirect
location: https://youtube.com/
content-length: 0
date: Sun, 28 Feb 2021 18:55:21 GMT

$ curl -i localhost:8080/dne
HTTP/1.1 404 Not Found
content-length: 0
date: Sun, 28 Feb 2021 18:55:26 GMT
```

HTML is generated for the index page by default:

```
$ curl -s localhost:8080
<!doctype html>
<html>
	<head>
		<title>my website</title>
... snipped ...
```

This can be overridden by explicitly accepting `json`:

```
$ curl -s -H 'accept: application/json' localhost:8080/ | jq
{
  "gh": "https://github.com/",
  "yt": "https://youtube.com/"
}
```
