import * as cookies from 'js-cookie';

export class Browser {

  static get cookieDomain() {
    let delim = '.';
    let hostname = this.currentHostname;
    let tld = hostname.split(delim).pop();

    if (isNaN(Number(tld))) {
      let domain = hostname.split(delim);
      domain.shift();
      return domain.join(delim) || hostname;
    } else {
      return hostname;
    }
  }

  static get currentHostname() {
    return location.hostname;
  };

  static getCookie(key) {
    return cookies.get(key);
  }

  static redirect(url) {
    window.location.href = url;
  }

  static removeCookie(key) {
    cookies.remove(key, { domain: this.cookieDomain });
  }

  static setCookie(key, value) {
    return cookies.set(key, value, {
      domain: this.cookieDomain,
      secure: window.location.protocol === 'https'
    });
  }
}
