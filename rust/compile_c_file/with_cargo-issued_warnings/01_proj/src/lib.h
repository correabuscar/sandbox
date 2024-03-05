#ifndef LIB_H
#define LIB_H

// Prototype for the deprecated function
// Deprecated function
void deprecated_function() __attribute__((deprecated));
//void deprecated_function() __attribute__((visibility("default"), deprecated));
#warning "manual warning since the deprecated one won't trigger"
#pragma message "This is a custom warning message, too"

#endif

