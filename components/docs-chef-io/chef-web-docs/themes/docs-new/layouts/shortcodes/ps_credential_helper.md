Use the `ps_credential` helper to embed a `PSCredential` object--- [a
set of security credentials, such as a user name or
password](https://technet.microsoft.com/en-us/magazine/ff714574.aspx)
---within a script, which allows that script to be run using security
credentials.

For example, assuming the `CertificateID` is configured in the local
configuration manager, the `SeaPower1@3` object is created and embedded
within the `seapower-user` script:

``` ruby
dsc_script 'seapower-user' do
  code <<-EOH
    User AlbertAtom
    {
      UserName = 'AlbertAtom'
      Password = #{ps_credential('SeaPower1@3')}
    }
  EOH
  configuration_data <<-EOH
    @{
      AllNodes = @(
        @{
          NodeName = "localhost";
          CertificateID = 'A8D1234559F349F7EF19104678908F701D4167'
        }
      )
    }
  EOH
end
```