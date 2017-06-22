IP Tool
=============
Tool for IPv4 and IPv6 related work.<br />
Utilizes the [IP Tool Core](https://github.com/timmonfette1/iptool_core) and has two
different interfaces for use:

  -- A text based UI (IP Tool UI)
  -- A single run Script (IP Tool Script)

The UI is recommneded for users who like interfaces.<br />
The script is recommended for use in automation.

How to Build
------------
TODO

Supported Functions
------------
The tool supports the following IP functions:

  - Validates an IPv4 address.
  - Converts an IPv4 address to IPv6.
  - Converts an IPv4 address to it's binary representation.
  - Validates an IPv6 address.
  - Converts an IPv6 address to IPv4;
  - Converts an IPv6 address to it's binary representation.

IP Tool UI Usage
------------
IP Tool UI is very self explanatory.<br />
After building with the Makefile, simply run `$ iptool`<br />
For more information refer to the above "How to Build" section.

IP Tool Script Usage
------------
Refer to the following guidelines for running the tool:<br />
  -- Validate IPv4: `./target/release/iptool valid_ipv4 <address>`<br />
  -- IPv4 to IPv6: `./target/release/iptool ipv4_ipv6 <address>`<br />
  -- IPv4 to binary: `./target/release/iptool ipv4_binary <address>`<br />
  -- Validate IPv6: `./target/release/iptool valid_ipv6 <address>`<br />
  -- IPv6 to IPv4: `./target/release/iptool ipv6_ipv4 <address>`<br />
  -- IPv6 to binary: `./target/release/iptool ipv6_binary <address>`<br />

Any other usage will result in a printing of the above.

NOTE: For IPv6 to IPv4 translastion, a valid format is needed.<br />
Valid formats for that are as follows:

  - 2002::xxxx:xxxx or
  - 2002:xxxx:xxxx::

Future Enhancements
-------------
I have written an FFI interface around IP Tool Core and I would like to create
librarys for various languages that can allow the tool to be used in a variety
of different programming languages.

When that is written, it will probably exist in it's own repository (iptool-ffi).<br />
However there will be a link to it here when it's completed for ease of access.
