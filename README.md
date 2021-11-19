mate
===

## Mini Algorithmic Trading Engine

`mate` is an experimental trading engine project. Use at your own risk.

Mate supports the following brokerages:

|Broker|Description|
|-|-|
|TD Ameritrade|See [TD Ameritrade developer portal](https://developer.tdameritrade.com/user) and [TDA SDK Crate source](https://github.com/rideron89/tda-sdk-rs/) in use for this project|


## Getting Started

You will need access to an API with live market data. Currently only supporting [`TD Ameritrade`](#TD-Ameritrade-Setup).

### Configuration

Configuration is passed in through environment only currently:

|Env Var|Description|
|-|-|
|`TDA_CLIENT_ID`|OAuth Client ID from your TD Ameritrade developer account, appending your oAuth domain eg: `@AMER.OAUTHAP`|
|`TDA_REFRESH_TOKEN`|Oauth refresh token to renew oauth access token, see [`TD Ameritrade`](#TD-Ameritrade-Setup) for |


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

> note: if you end up with a failing `grant_invalid` error, you might want to make sure you're unquoting your authorization code, otherwise see:
>
> https://developer.tdameritrade.com/content/authentication-faq

## TODOs

- Get away from TODOs
- open the user's browser to grab the refresh token when they start the application
- create a listener on local host to intercept the refresh token -> use the query params to fetch the authorization code, then exit listener and request the refresh token with authorization code.
- Setup paper trading
- Setup a collector for future backtesting feature
- Setup API to feed into a web dashboard

