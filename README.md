IP Tool
=============
Tool for IPv4 and IPv6 related work.<br />
Utilizes the [IP Tool Core](https://github.com/timmonfette1/iptool_core) and has two
different interfaces for use:

  - A text based UI (IP Tool UI)<br />
  - A single run Script (IP Tool Script)

The UI is recommneded for users who like interfaces.<br />
The script is recommended for use in automation.

How to Install Rust
------------
IP Tool (and it's core) is written in the Rust programming language. If you plan on using this tool,
you'll need to have Rust and it's package manager Cargo installed on your system.<br />
A couple useful links to aid in this installation can be found at:

 - https://www.rust-lang.org/en-US/install.html
 - http://doc.crates.io/

In the future I might provide a Makefile task to install this automatically if not already installed.<br />
If you're looking for IP Tool in a language that isn't Rust, I'm working on an FFI for it and will
include libraries for various languages along with it. Once it's completed, those libraries can be used
with other languages to gain iptool-core functionality and you can wrap your own interface(s) around it. 

How to Build
------------
A Makefile is included to make compiling differnet versions simple and fast.<br />
The following describes each Makefile target:

`make`<br />
Default behavior is to run the "build-script" target to build the IP Tool Script.

`make build-script`<br />
Will build the IP Tool Script.<br />
This is done by moving the "iptool-script/" to "src/" and then running `cargo build --release`<br />
to build the project. It is then available for use at "./target/release/iptool".

`make build-ui`<br />
Will build the IP Tool UI.<br />
This is done by moving the "iptool-ui/" to "src/" and then running `cargo build --release`<br />
to build the project. It is then available for use at "./target/release/iptool".

`make clean`
This will restore the directory to it's inital state. This means:

 - Removing Cargo.lock
 - Removing the "target" directory
 - Moving "src/" back to "iptool-script" or "iptool-ui".

The Makefile will check before moving "src/" to make sure it restores the correct project.

MAKE SURE TO RUN `make clean` BEFORE BUILDING THE OTHER PROJECT TO AVOID OVER-WRITING "src/".<br />
The Makefile checks for "src/" before doing a build to make sure this doesn't happen,
but it never hurts to make sure you check for yourself before doing this.

I recommend updating your PATH variable in your ~/.profile to include the path
to the IP Tool executable for ease of access.

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
After building with the Makefile, simply run `$ ./target/release/iptool`<br />
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
libraries for various languages that can allow the tool to be used in a variety
of different programming languages.

When that is written, it will probably exist in it's own repository (iptool-ffi).<br />
However there will be a link to it here when it's completed for ease of access.
