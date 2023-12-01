<p align="center">
  <a href="https://nexis.network">
    <img alt="Nexis" src="https://i.imgur.com/IKyzQ6T.png" width="250" />
  </a>
</p>

# Nexis Geyser Plugin Interface

This crate enables an plugin to be added into the Nexis Validator runtime to
take actions at the time of account updates or block and transaction processing;
for example, saving the account state to an external database. The plugin must
implement the `GeyserPlugin` trait. Please see the detail of the
`geyser_plugin_interface.rs` for the interface definition.

The plugin should produce a `cdylib` dynamic library, which must expose a `C`
function `_create_plugin()` that instantiates the implementation of the
interface.

The https://github.com/nexis-labs/nexis-accountsdb-plugin-postgres repository
provides an example of how to create a plugin which saves the accounts data into
an external PostgreSQL databases.

More information about Nexis is available in the [Nexis documentation](https://docs.nexis.network/).

Still have questions?  Ask us on [Discord](https://discordapp.com/invite/pquxPsq)
