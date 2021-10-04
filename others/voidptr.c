#include <stdio.h>
#include <stdlib.h>

// The state of the plugin
typedef struct {
    int counter;
} plugin_state_t;

// Exported by the plugin, initializes the state
void* new() {
    plugin_state_t* plugin_state = malloc(sizeof(plugin_state_t));
    plugin_state->counter = 0;
    return (void*) plugin_state;
}

// Exported by the plugin, which takes a pointer to its state
void something(void* state) {
    // We know the runtime used `new` to initialize the state, so we can cast it
    // back to its original type.
    plugin_state_t* plugin_state = (plugin_state_t*) state;

    printf("Current state: { counter = %d }\n", plugin_state->counter);
    plugin_state->counter++;
    printf("Final state: { counter = %d }\n", plugin_state->counter);
}

int main() {
    // We initialize the plugin, which returns its state
    void* state = new();
    // When calling anything from the plugin we pass its state
    something(state);
    // Don't forget!
    free(state);
}
