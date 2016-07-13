========================================
tenrus - Travis Encryption in Rust
========================================

Travis CI `says <https://docs.travis-ci.com/user/environment-variables/#Encrypting-Variables-Using-a-Public-Key>`_
, "you can use `travis <https://github.com/travis-ci/travis.rb>`_
to encrypt your secret and put it at ``.travis.yml`` ".

Here is a alternative to ``travis encrypt`` .

(This project's ``.travis.yml`` has a token encrypted by itself :P)


.. contents:: Table of Contents


Installation
========================================

From `crate.io <https://crates.io/>`_

.. code-block:: sh

    $ cargo install tenrus


From GitHub

.. code-block:: sh

    $ cargo install --git https://github.com/wdv4758h/tenrus/


Download Prebuilt Binary

.. code-block:: sh

    # by curl
    $ curl -O -J -L https://github.com/wdv4758h/tenrus/releases/download/v0.1.0/tenrus-v0.1.0-x86_64-unknown-linux-gnu.tar.gz

    # by wget
    $ wget https://github.com/wdv4758h/tenrus/releases/download/v0.1.0/tenrus-v0.1.0-x86_64-unknown-linux-gnu.tar.gz



Usage
========================================

.. code-block:: sh

    $ tenrus
    tenrus 0.1.0
    Chiu-Hsiang Hsu <wdv4758h@gmail.com>
    Travis Encryption in Rust

    USAGE:
        tenrus [ARGS]

    FLAGS:
        -h, --help       Prints help information
        -V, --version    Prints version information

    ARGS:
        <repo>    repository slug (USERNAME/PROJECT) e.g. "wdv4758h/tenrus"
        <data>    data to encrypt


.. code-block:: sh

    $ tenrus wdv4758h/tenrus MYSECRET
    secure: "p9BUpUjJiet+MX84SiAPd0xt4Gczmma67N2RPiCuyGHdz+7Blmtgra85SF1TZocy6RyXykMp354fHcbfu29mwArEaaPwALvybmCTIcE66ZvFWSp2k5LJHDJiIu15OUrTrZGvSAd1//rmmeFklRIVGBaE4rP5vuKpNx4VXwaz+X8RAz+OnHCEWJkpZkb2gBsQMiApsG2dF5KKdvogRHv22HMi2XIWzTRojhigNoS7AaUsZEwbgm2psyZRoKSEqdCMSaf/qYDzhIZUIlb0GqOAnkT+97SRGFDpYpl2bQQ7qyGdQ1SnZZo9WaU1+DdNPFtA63BVyEajOQXSlaJ98ulGk9v/jrqSzYFHp4Mp/trrlC7BBRXxn9uuBTKZMOBWukKi3Ve3XQfTcUMY7S/EPjtyeL26fTCs2dCTPD/IUKlgyW8cczYbbwV6as6sdU6ErtA6WpWRms23x13IZSaiJ5Ne4TiaNfxJtC/k35Q1gRtC9HWcusvKZFVGPtFSAxGbzLAmJHUS2yQOD2lY/Np8xFNG8WgvMaf9BguZct99H07bDTaIgRtGvt9IazDlbTKHGdGMPi/islr8NpXvZNyThqObj58K/s1e9JTwyKksg4nCQioTnvpjXsb3C6g9NKOisXlZmbso8MkhQlevuESpC11eIOvkO4prrJZ5Bjts4aiBtLw="



Information About Binary
========================================

Size
------------------------------

x86_64, Linux (build on Arch Linux)

+----------+---------+------------+--------------+-----------+
| Filename | Version | Stripped ? | Size (Bytes) | Size (MB) |
+----------+---------+------------+--------------+-----------+
| tenrus   | v0.1.0  | No         | 2843768      | 2.8M      |
+----------+---------+------------+--------------+-----------+
| tenrus   | v0.1.0  | Yes        | 2159288      | 2.1M      |
+----------+---------+------------+--------------+-----------+


Shared Library Dependency
------------------------------

x86_64, Linux (build on Arch Linux)

.. code-block:: sh

    $ ldd ./target/release/tenrus
            linux-vdso.so.1 (0x00007ffef6ce5000)
            libssl.so.1.0.0 => /usr/lib/libssl.so.1.0.0 (0x00007f3426482000)
            libcrypto.so.1.0.0 => /usr/lib/libcrypto.so.1.0.0 (0x00007f342600b000)
            libdl.so.2 => /usr/lib/libdl.so.2 (0x00007f3425e07000)
            libpthread.so.0 => /usr/lib/libpthread.so.0 (0x00007f3425bea000)
            libgcc_s.so.1 => /usr/lib/libgcc_s.so.1 (0x00007f34259d4000)
            libc.so.6 => /usr/lib/libc.so.6 (0x00007f3425633000)
            /lib64/ld-linux-x86-64.so.2 (0x00007f34266f3000)
            libm.so.6 => /usr/lib/libm.so.6 (0x00007f342532f000)



Changelog
========================================

Not Implemented Yet (Plan)
------------------------------

* cache for Travis CI's public key
* can use local key file
* can use custom URL
* more encryption options (not just OpenSSL)



Notice
========================================

I've only tested on my x86_64 Linux.
Other platforms are built by CI.
If they don't work properly, please tell me.


Developement
========================================

Making Release
------------------------------

1. update version in ``src/arguments.yml``
2. update version in ``Cargo.toml``
3. update version in ``Cargo.lock``
4. add git tag



Special Thanks
========================================

* `rust-everywhere <https://github.com/japaric/rust-everywhere/>`_ for CI integration
* `clap-rs <https://github.com/kbknapp/clap-rs>`_ for arguments parsing
* `hyper <https://github.com/hyperium/hyper>`_ for HTTPS client
* `Serde <https://github.com/serde-rs/serde>`_ for serialization
* `rustc-serialize <https://github.com/rust-lang-nursery/rustc-serialize>`_ for serialization
* `OpenSSL <https://www.openssl.org/>`_ for encryption
* `Rust Team <https://www.rust-lang.org/team.html>`_
* and every project I've used



License
========================================

tenrus is licensed under the AGPL License - see the ``LICENSE`` file for details
