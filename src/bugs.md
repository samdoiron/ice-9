# Bugs

Constants don't work correctly with functions. The ops for constants are defined using
the local StatementChunk index but constants are not scoped at run time. We will probably
need to pass along a global / shared constant registry. This will also give us a chance to
remove duplicate constants.
