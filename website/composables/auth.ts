import { generateCodeVerifier, OAuth2Client, OAuth2Fetch, type OAuth2Token } from "@badgateway/oauth2-client";
import type { ClientSettings } from "@badgateway/oauth2-client/dist/client";
import { onMounted, ref, type Ref } from "vue";

export type ProviderSettings = ClientSettings & {
  /**
   * the name of the provider - this becomes a prefix for values stored in browser storage
   */
  providerName: string;
  /**
   * a relative path to the redirect uri
   * @example '/auth'
   */
  callbackPath: string;
};

/**
 * a clientside only OAuth2 provider
 */
export class OAuth2Provider {
  private _client: OAuth2Client;
  private _token?: OAuth2Token | null;
  private _codeVerifier?: string;
  private _settings: ProviderSettings;
  private _state?: string;
  accessToken: Ref<string | null> = ref(null);

  constructor(settings: ProviderSettings) {
    const self = this;
    const redirectUri = window.location.origin + settings.callbackPath;
    const client = new OAuth2Client(settings);
    this._settings = settings;
    this._client = client;

    const fetcher = new OAuth2Fetch({
      client: this._client,
      storeToken: (token) => {
        window.localStorage.setItem(`${settings.providerName}_token`, JSON.stringify(token));
      },
      getStoredToken() {
        const tokenString = window.localStorage.getItem(`${settings.providerName}_token`);
        if (tokenString) {
          return JSON.parse(tokenString);
        }
        return null;
      },
      async getNewToken() {
        return await client.authorizationCode.getToken({
          redirectUri,
          codeVerifier: await self.getCodeVerifier(),
          code: new URL(window.location.href).searchParams.get('code') || '',
        });
      },
      onError(err: Error) {
      },
    })
  }

  async tokenCallback(navigator?: (url: string) => void) {
    const callbackPath = this._settings.callbackPath;
    const token = await this._client.authorizationCode.getTokenFromCodeRedirect(
      window.location.href,
      {
        codeVerifier: await this.getCodeVerifier(),
        redirectUri: window.location.origin + callbackPath,
        state: this.state,
      }
    )

    this.token = token;
    this.state = null;
    const continueTo = this.continueTo;
    window.history.replaceState(null, '', continueTo);
    if (navigator) {
      navigator(continueTo);
    } else {
      window.history.go();
    }
  }

  async login(scopes: string[]) {
    // is this safe? i feel like it would be, but I don't know. womp.
    this.state = await generateCodeVerifier();
    window.location.href = await this._client.authorizationCode.getAuthorizeUri({
      codeVerifier: await this.getCodeVerifier(),
      redirectUri: window.location.origin + this._settings.callbackPath,
      scope: scopes,
      state: this.state
    })
  }

  async getCodeVerifier(): Promise<string> {
    if (!window?.sessionStorage) {
      throw new Error('`window.sessionStorage` is required, is this being called inside of client context?');
    }
    const name = this._settings.providerName;
    if (!this._codeVerifier) {
      this._codeVerifier = window.sessionStorage.getItem(`${name}_code_verifier`) || await generateCodeVerifier();
      window.sessionStorage.setItem(`${name}_code_verifier`, this._codeVerifier);
    }
    return this._codeVerifier;
  }

  /**
   * a relative path to an area to continue to after login, for if you wish to navigate back to a page.
   * @example '/dashboard'
   */
  get continueTo(): string {
    const name = this._settings.providerName;
    return window.sessionStorage.getItem(`${name}_continue_to`) ?? '/';
  }

  set continueTo(path: string | null) {
    const name = this._settings.providerName;
    if (!path) {
      window.sessionStorage.removeItem(`${name}_continue_to`);
    } else {
      window.sessionStorage.setItem(`${name}_continue_to`, path);
    }
  }

  private set token(token: OAuth2Token) {
    const name = this._settings.providerName;
    this._token = token;
    window.localStorage.setItem(`${name}_token`, JSON.stringify(token));
  }

  get token(): OAuth2Token | null {
    const name = this._settings.providerName;
    let token = this._token;
    if (!token) {
      const tokenString = window.localStorage.getItem(`${name}_token`);
      if (tokenString) {
        token = JSON.parse(tokenString);
      }
    }

    if (token && (token.expiresAt ?? 0) < Date.now()) {
      window.localStorage.removeItem(`${name}_token`);
      token = null;
    }

    this._token = token;

    return token || null;
  }

  private set state(state: string | null) {
    const name = this._settings.providerName;
    if (state === null) {
      delete this._state;
      window.sessionStorage.removeItem(`${name}_state`);
      return;
    }

    this._state = state;
    window.sessionStorage.setItem(`${name}_state`, state);
  }

  get state(): string | undefined {
    const name = this._settings.providerName;
    if (!this._state) {
      this._state = window.sessionStorage.getItem(`${name}_state`) || undefined;
    }
    return this._state;
  }
}

export const useOAuth2Provider = (settings: ProviderSettings) => {
  const provider = new OAuth2Provider(settings);

  return provider;
}