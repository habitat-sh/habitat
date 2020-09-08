A `Policyfile.lock.json` file is similar to:

``` javascript
{
  "revision_id": "288ed244f8db8bff3caf58147e840bbe079f76e0",
  "name": "jenkins",
  "run_list": [
    "recipe[java::default]",
    "recipe[jenkins::master]",
    "recipe[policyfile_demo::default]"
  ],
  "cookbook_locks": {
    "policyfile_demo": {
      "version": "0.1.0",
      "identifier": "f04cc40faf628253fe7d9566d66a1733fb1afbe9",
      "dotted_decimal_identifier": "67638399371010690.23642238397896298.25512023620585",
      "source": "cookbooks/policyfile_demo",
      "cache_key": null,
      "scm_info": null,
      "source_options": {
        "path": "cookbooks/policyfile_demo"
      }
    },
  "java": {
    "version": "1.24.0",
    "identifier": "4c24ae46a6633e424925c24e683e0f43786236a3",
    "dotted_decimal_identifier": "21432429158228798.18657774985439294.16782456927907",
    "cache_key": "java-1.24.0-supermarket.chef.io",
    "origin": "https://supermarket.chef.io/api/v1/cookbooks/java/versions/1.24.0/download",
    "source_options": {
      "artifactserver": "https://supermarket.chef.io/api/v1/cookbooks/java/versions/1.24.0/download",
      "version": "1.24.0"
    }
```