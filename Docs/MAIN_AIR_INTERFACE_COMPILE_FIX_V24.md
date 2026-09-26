# Main Air-Interface Compile Fix v24

This revision keeps the v23 main-compatible MM/MLE/CMCE radio path and adapts
`mle_bs.rs` to the newer SWMI LTPD SAP interface.

Changes:

- populate `LtpdMleUnitdataInd.received_address_type` from the received TETRA address;
- convert the legacy TLA `Option<i32>` channel-change handle into the strongly typed
  `Option<ChannelChangeHandle>` without accepting negative values;
- leave MM, CMCE, call release, Simplex/Duplex and SIP routing unchanged from v23.
