---
title: Analytics in Chef Habitat
---

# Analytics in Chef Habitat

## Command-Line Analytics

The `hab` command-line utility is instrumented to _optionally_ send anonymous usage data to us. This is an opt-in activity and no tracking occurs unless you respond affirmatively to the question during `hab cli setup`. If you do not use `hab cli setup`, no data will ever be sent.

We collect this data to help improve Chef Habitat's user experience: for example, to know what tasks users are performing, and which ones they are having trouble with.

By anonymous we mean that all identifying information about you is removed before we send the data. This includes the removal of any information about what packages you are building, or what origins you are using. For example, if you were building the package `yourname/yourapp`, and you typed `hab pkg build -k yourkey yourname/yourapp`, the fact that you were performing the `pkg build` operation would be transmitted. Neither the name of the specific package you are building, nor the fact that you are using the `yourkey` key to sign that package would be transmitted.

We also suppress the transmittal of your IP address.

Please do not hesitate to contact us at support@habitat.sh should you have questions or concerns about the use of analytics in the Chef Habitat product.

<hr>
<ul class="main-content--link-nav">
  <li>Continue to the next topic</li>
  <li><a href="/docs/install-habitat/">Download and install Chef Habitat</a></li>
</ul>
