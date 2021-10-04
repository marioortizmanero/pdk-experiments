#include <stdio.h>
#include <stdlib.h>

// The base plugin type
typedef struct base_state_t {
    void (*print)(struct base_state_t *);
} base_state_t;

// The state of the plugin, child of the above type
typedef struct {
    base_state_t base;
    int counter;
} plugin_state_t;

// The implementation of `print` for the `plugin_state_t` child
void print(base_state_t* state) {
    plugin_state_t* plugin_state = (plugin_state_t*) state;
    printf("Current state: { counter = %d }\n", plugin_state->counter);
}

// Exported by the plugin, initializes the state
base_state_t* new() {
    base_state_t base = {print};

    plugin_state_t* plugin_state = malloc(sizeof(plugin_state_t));
    plugin_state->base = base;
    plugin_state->counter = 0;
    return (base_state_t*) plugin_state;
}

// Exported by the plugin, which takes a pointer to its state
void something(void* state) {
    // We know the runtime used `new` to initialize the state, so we can cast it
    // back to its original type.
    plugin_state_t* plugin_state = (plugin_state_t*) state;
    plugin_state->counter++;
}

int main() {
    // We initialize the plugin, which returns its state
    base_state_t* state = new();
    // When calling anything from the plugin we pass its state
    state->print(state);
    something((void*) state);
    state->print(state);
    // Don't forget!
    free(state);
}
