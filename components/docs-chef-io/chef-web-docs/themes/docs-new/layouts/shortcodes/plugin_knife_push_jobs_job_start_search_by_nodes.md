To search for a specific set of nodes (named `chico`, `harpo`,
`groucho`, `gummo`, `zeppo`), and where 90% of those nodes must be
available, run the following command:

``` bash
knife job start --quorum 90% 'chef-client' chico harpo groucho gummo zeppo
```

to return something similar to:

``` bash
Started. Job ID: GUID12345abc
  quorum_failed
  Command: chef-client
  Created_at: date
  unavailable: zeppo
  was_ready:
    gummo
    groucho
    chico
    harpo
  On_timeout: 3600
  Status: quorum_failed
```

<div class="admonition-note">

<p class="admonition-note-title">Note</p>

<div class="admonition-note-text">

If quorum had been set at 80% (`--quorum 80%`), then quorum would have
passed with the previous example.



</div>

</div>