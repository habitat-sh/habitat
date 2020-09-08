A validation parameter is used to add zero (or more) validation
parameters to a property.

<table>
<colgroup>
<col style="width: 25%" />
<col style="width: 75%" />
</colgroup>
<thead>
<tr class="header">
<th>Parameter</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><p><code>:callbacks</code></p></td>
<td><p>Use to define a collection of unique keys and values (a ruby hash) for which the key is the error message and the value is a lambda to validate the parameter. For example:</p>
<div class="sourceCode" id="cb1"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb1-1"><a href="#cb1-1"></a><span class="st">callbacks: </span>{</span>
<span id="cb1-2"><a href="#cb1-2"></a>             <span class="st">&#39;should be a valid non-system port&#39;</span> =&gt; lambda {</span>
<span id="cb1-3"><a href="#cb1-3"></a>               |p| p &gt; <span class="dv">1024</span> &amp;&amp; p &lt; <span class="dv">65535</span></span>
<span id="cb1-4"><a href="#cb1-4"></a>             }</span>
<span id="cb1-5"><a href="#cb1-5"></a>           }</span></code></pre></div></td>
</tr>
<tr class="even">
<td><p><code>:default</code></p></td>
<td><p>Use to specify the default value for a property. For example:</p>
<div class="sourceCode" id="cb2"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb2-1"><a href="#cb2-1"></a><span class="st">default: &#39;a_string_value&#39;</span></span></code></pre></div>
<div class="sourceCode" id="cb3"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb3-1"><a href="#cb3-1"></a><span class="st">default: </span><span class="dv">123456789</span></span></code></pre></div>
<div class="sourceCode" id="cb4"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb4-1"><a href="#cb4-1"></a><span class="st">default: </span>[]</span></code></pre></div>
<div class="sourceCode" id="cb5"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb5-1"><a href="#cb5-1"></a><span class="st">default: </span>()</span></code></pre></div>
<div class="sourceCode" id="cb6"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb6-1"><a href="#cb6-1"></a><span class="st">default: </span>{}</span></code></pre></div></td>
</tr>
<tr class="odd">
<td><p><code>:equal_to</code></p></td>
<td><p>Use to match a value with <code>==</code>. Use an array of values to match any of those values with <code>==</code>. For example:</p>
<div class="sourceCode" id="cb7"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb7-1"><a href="#cb7-1"></a><span class="st">equal_to: </span>[<span class="dv">true</span>, <span class="dv">false</span>]</span></code></pre></div>
<div class="sourceCode" id="cb8"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb8-1"><a href="#cb8-1"></a><span class="st">equal_to: </span>[<span class="st">&#39;php&#39;</span>, <span class="st">&#39;perl&#39;</span>]</span></code></pre></div></td>
</tr>
<tr class="even">
<td><p><code>:regex</code></p></td>
<td><p>Use to match a value to a regular expression. For example:</p>
<div class="sourceCode" id="cb9"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb9-1"><a href="#cb9-1"></a><span class="st">regex: </span>[ <span class="ot">/^([a-z]|[A-Z]|[0-9]|_|-)+$/</span>, <span class="ot">/^\d+$/</span> ]</span></code></pre></div></td>
</tr>
<tr class="odd">
<td><p><code>:required</code></p></td>
<td><p>Indicates that a property is required. For example:</p>
<div class="sourceCode" id="cb10"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb10-1"><a href="#cb10-1"></a><span class="st">required: </span><span class="dv">true</span></span></code></pre></div></td>
</tr>
<tr class="even">
<td><p><code>:respond_to</code></p></td>
<td><p>Use to ensure that a value has a given method. This can be a single method name or an array of method names. For example:</p>
<div class="sourceCode" id="cb11"><pre class="sourceCode ruby"><code class="sourceCode ruby"><span id="cb11-1"><a href="#cb11-1"></a><span class="st">respond_to: </span>valid_encoding?</span></code></pre></div></td>
</tr>
</tbody>
</table>

Some examples of combining validation parameters:

``` ruby
property :spool_name, String, regex: /$\w+/
```

``` ruby
property :enabled, equal_to: [true, false, 'true', 'false'], default: true
```