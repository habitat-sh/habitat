### Linux

We use [expect](https://core.tcl.tk/expect/index) in order to test the internal behavior of the Studio. We use `spawn` to create a studio, and then `send` to interact with it. With `send` we're unable to use return codes from the commands run, so all tests must emit `[PASS]` or `[FAIL]` before exiting. An automatic timeout is scheduled, such that a test will Fail if it doesn't emit either `[PASS]` or `[FAIL]`.


### Windows

We use the [Await module](https://www.powershellgallery.com/packages/Await/0.8) to test the internal behavior of the studio. Like expect, we're unable to use return codes from the tests and all tests must emit `[PASS]` or `[FAIL]`.  Unlike expect, we're unable to timeout a test, so we rely on the default timeout for CI in order to fail the test. 
