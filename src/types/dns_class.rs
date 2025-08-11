// There are multiple class types but basically all cases we will be dealing with IN.
pub enum DnsClass {
    // CLASSes
    In, // Internet
    Cs, // CSNET (Obsolete - only used as an example in obsolete RFCs)
    Ch, // CHAOS
    Hs, // Hesiod (Dyer 87)

    // QCLASSes
    Any = 255, // Any Class
}
