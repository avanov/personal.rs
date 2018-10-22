Dev Environment
---------------


.. code-block::

    $ cargo install cargo-make --force


Release Builds
--------------

The release build is compiled in `rust-musl-builder <https://hub.docker.com/r/ekidd/rust-musl-builder/>`_.


.. code-block::

    $ cargo make release