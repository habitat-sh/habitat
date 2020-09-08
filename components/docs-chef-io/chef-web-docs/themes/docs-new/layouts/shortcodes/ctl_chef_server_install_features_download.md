The `install` subcommand downloads packages from
<https://packages.chef.io/> by default. For systems that are not behind
a firewall (and have connectivity to <https://packages.chef.io/>), these
packages can be installed as described below.

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 80%" />
</colgroup>
<thead>
<tr class="header">
<th>Feature</th>
<th>Command</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><p>Chef Manage</p></td>
<td><p>Use Chef management console to manage data bags, attributes, run-lists, roles, environments, and cookbooks from a web user interface.</p>
<p>On the Chef Infra Server, run:</p>
<div class="sourceCode" id="cb1"><pre class="sourceCode bash"><code class="sourceCode bash"><span id="cb1-1"><a href="#cb1-1"></a><span class="fu">sudo</span> chef-server-ctl install chef-manage</span></code></pre></div>
<p>then:</p>
<div class="sourceCode" id="cb2"><pre class="sourceCode bash"><code class="sourceCode bash"><span id="cb2-1"><a href="#cb2-1"></a><span class="fu">sudo</span> chef-server-ctl reconfigure</span></code></pre></div>
<p>and then:</p>
<div class="sourceCode" id="cb3"><pre class="sourceCode bash"><code class="sourceCode bash"><span id="cb3-1"><a href="#cb3-1"></a><span class="fu">sudo</span> chef-manage-ctl reconfigure</span></code></pre></div>
<p>To accept the <a href="/chef_license/">Chef MLSA</a>:</p>
<div class="sourceCode" id="cb4"><pre class="sourceCode bash"><code class="sourceCode bash"><span id="cb4-1"><a href="#cb4-1"></a><span class="fu">sudo</span> chef-manage-ctl reconfigure --accept-license</span></code></pre></div></td>
</tr>
</tbody>
</table>