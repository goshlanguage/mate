# mate

![build](https://github.com/goshlanguage/mate/actions/workflows/release.yaml/badge.svg)

`mate` is an experimental mini algorithmic trading engine. I am writing this to learn more about algorithmic trading, and quantitative finance.

Mate supports the following account types:

| Account Type  | Description                                                                                                                                                                |
| ------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| TD Ameritrade | See [TD Ameritrade developer portal](https://developer.tdameritrade.com/user) and [TDA SDK Crate source](https://github.com/rideron89/tda-sdk-rs/) in use for this project |
| Kraken        | See [Kraken API docs](https://docs.kraken.com/rest/#section/Authentication) and [TDA SDK Crate source](https://github.com/rideron89/tda-sdk-rs/) in use for this project   |

## Getting Started

You will need access to an API with live market data. Currently only supporting [`TD Ameritrade`](#TD-Ameritrade-Setup).

### Configuration

Configuration is passed in through environment only currently:

| Env Var                | Description                                                                                                |
| ---------------------- | ---------------------------------------------------------------------------------------------------------- |
| `TDA_CLIENT_ID`        | OAuth Client ID from your TD Ameritrade developer account, appending your oAuth domain eg: `@AMER.OAUTHAP` |
| `TDA_REFRESH_TOKEN`    | Oauth refresh token to renew oauth access token, see [`TD Ameritrade`](#TD-Ameritrade-Setup)               |
| `KRAKEN_CLIENT_KEY`    | API client key for your Kraken Account                                                                     |
| `KRAKEN_CLIENT_SECRET` | API client secret for your Kraken Account                                                                  |

## Getting your keys

### TD Ameritrade Setup

To register an account and get API access, you can either see [upstream documentation](https://developer.tdameritrade.com/content/getting-started#createAccount), or try your luck with the steps below:

- [Sign up](https://developer.tdameritrade.com/user/register) for a developer account, even if you have a brokerage account
- [Create your application](https://developer.tdameritrade.com/user/me/apps/add) (if you plan to run this locally, use `http://localhost` or `http://127.0.0.1` for the `redirect_url`)
- go to `https://auth.tdameritrade.com/auth?response_type=code&redirect_uri=<Your url encoded callback url>&client_id=<Your Client Key>%40AMER.OAUTHAP` in a browser, replacing `<Your url encoded callback url>` with the `redirect_url` you created when you created your TD Ameritrade app, and replacing `<Your Client Key>` with the client key from your TD Ameritrade app.
  - eg `https://auth.tdameritrade.com/auth?response_type=code&redirect_uri=http%3A%2F%2Flocalhost&client_id=MYKEY123%40AMER.OAUTHAP`
- This should redirect you to a log in for your TD ameritrade account (brokerage account), to perform the oauth grant to give your app access to your account
- if you used localhost, it will redirect and be broken, but the code to fetch your access and refresh token will be in the redirected url query params (eg: `https://localhost/?code=Your%2Code%2Here`)
- take that code, and go to [https://developer.tdameritrade.com/authentication/apis/post/token-0](https://developer.tdameritrade.com/authentication/apis/post/token-0)
  - enter `authorization_code` for `grant_type`
  - enter `offline` for `access_type`
  - urldecode the code you copied from the redirect earlier

    ```sh
    ipython -c "import urllib; urllib.parse.unquote(\"YOURCODEHERE\")"
    ```

  - enter the urldecoded string into `code`
  - enter your application's given `client_id` into `client_id` (should end in `@AMER.OAUTHAP`)
  - enter your application's given `redirect_uri`
  - click `send`
  - the JSON populated at the bottom of the page contains an `access_token` and a `refresh_token`, be sure to save the `refresh_token`, and set it in your environment (export it in your shell rc eg: `~/.bashrc` or `~/.zshrc`)

> note: if you end up with a failing `grant_invalid` error, you might want to make sure you're unquoting your authorization code, otherwise see the [Authentication FAQ](https://developer.tdameritrade.com/content/authentication-faq)

## Developing

### Data

Data is represented with tick data, or OHLC candles, then serialized into JSON.

_Equities_:

```json
[
  {
    'close': 39.435,
    'datetime': 1546236000000,
    'high': 39.84,
    'low': 39.12,
    'open': 39.6325,
    'volume': 140013864
  }
]
```

This is derived from the response of the [`pricehistory` endpoint](https://developer.tdameritrade.com/price-history/apis/get/marketdata/%7Bsymbol%7D/pricehistory) of the TD Ameritrade API

_Crypto_:

```json
{'1641237335': {'a': ['0.169609000', '221449', '221449.000'],
                'b': ['0.169608900', '268', '268.000'],
                'c': ['0.169608900', '67.14677906']}
}
```

JSON contains a list of epoch timestamps as strings, containing an object representing the ask, bid, and close of each tick.
This model is derived from the [`ticker` endpoint](https://docs.kraken.com/rest/#operation/getTickerInformation) response that comes from the Kraken API

### References

The following references may be helpful for the underlying technologies used in this project:

| Name     | Link                                                                                                                   |
| -------- | ---------------------------------------------------------------------------------------------------------------------- |
| clap     | [derive arg reference](https://github.com/clap-rs/clap/blob/v3.0.0-rc.11/examples/derive_ref/README.md#arg-attributes) |
| diesel | [getting started](https://diesel.rs/guides/getting-started) |
| | [type mappings](https://kotiri.com/2018/01/31/postgresql-diesel-rust-types.html) |
| krakenrs | [cargo docs](https://docs.rs/krakenrs/5.2.2/krakenrs/) |
|| [crates.io](https://crates.io/crates/krakenrs) |
