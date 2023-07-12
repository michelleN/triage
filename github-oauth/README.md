# GitHub Oauth Spin Component

This component implements the redirect url needed to get an auth token from GitHub for a user.

The rough dance:

1. Create a GitHub app. The `callback url` should be the url that points to this github component. If running locally: `http://127.0.0.1:3000/api/sessions/oauth.github`. Select `Request user authorization (OAuth) during installation`.

2. Run this app locally using spin up. You'll need to set some config variables using the info from the GitHub app: `SPIN_CONFIG_ID` = client_id, `SPIN_CONFIG_SECRET` = client_secret

3. Implement a [button](#implementing-the-frontend) that directs the user to github.com/login/oauth/authorize using the client id from the github app you just created.

4. After a user logs and gives permission, github will redirect to the callback_url and will add a query parameter called "code". This component will read that code and exchange it for an auth token. To do the exchange, the component will need the client_id, client_secret, redirect_url, and code. The component then uses that token to request the github handle for that user.

## Implementing the frontend

Use this code snippet to implement a login button on the frontend or direct the browser to the url `https://github.com/login/oauth/authorize` and add query params for client_id, redirect_url, scope, and state.

```
export function getGitHubUrl(from) {
const rootURl = 'https://github.com/login/oauth/authorize';

    const options = {
      client_id: 'Iv1.4119494d292e3225',
      redirect_uri: 'http://127.0.0.1:3000/api/sessions/oauth/github',
      scope: 'user:email',
      state: from,
    };

    const qs = new URLSearchParams(options);

    return `${rootURl}?${qs.toString()}`;

}
```
