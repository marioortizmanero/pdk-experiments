# dynamic-codec

This simulates what a plugin for Tremor connectors would look like with dynamic
loading, as a way of trying to get a more complex example running.

The following plugins have been implemented for now:

* `plugin-metronome`: an implementation of the `metronome` connector as a plugin
* `plugin-invalid`: a plugin with invalid metadata, which should be caught by
  the runtime instead of aborting
