# dynamic-connectors

This simulates what a plugin for Tremor connectors would look like with raw
dynamic loading. The following plugins have been implemented for now:

* `plugin-metronome`: valid plugin, should work just fine
* `plugin-missing`: invalid plugin, will be handled gracefully
* `plugin-versionmismatch`: invalid plugin, will be handled gracefully
* `plugin-wrongtype`: invalid plugin, will crash
* `plugin-wrongaddress`: invalid plugin, will crash
* `plugin-panic`: invalid plugin, will crash
