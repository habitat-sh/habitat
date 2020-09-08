To define OAuth 2 information for Chef Supermarket, create a Hash
similar to:

``` ruby
oc_id['applications'] ||= {}
oc_id['applications']['supermarket'] = {
  'redirect_uri' => 'https://supermarket.mycompany.com/auth/chef_oauth2/callback'
}
```