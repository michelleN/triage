export function getGitHubUrl(from) {
    const rootURl = 'https://github.com/login/oauth/authorize';
  
    const options = {
      //redirect_uri: import.meta.env.SPIN_CONFIG_REDIRECT_URI,
      client_id: 'Iv1.4119494d292e3225',
      redirect_uri: 'http://127.0.0.1:3000/api/sessions/oauth/github',
      scope: 'user:email',
      state: from,
    };
  
    const qs = new URLSearchParams(options);
  
    return `${rootURl}?${qs.toString()}`;
  }
  