# dynamic-codec

This simulates what a plugin for Tremor connectors would look like with dynamic
loading, as a way of trying to get a more complex example running.

The following plugins have been implemented for now:

* `plugin-metronome`: an implementation of the `metronome` connector as a plugin
* `plugin-missing`: invalid plugin, will be handled gracefully
* `plugin-versionmismatch`: invalid plugin, will be handled gracefully
* `plugin-wrongtype`: invalid plugin, will crash
