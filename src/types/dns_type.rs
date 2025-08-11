pub enum DnsType {
    // TYPEs
    A,     // IPv4 Address
    Ns,    // Authoritative Name Server
    Md,    // Obsolete - use MX
    Mf,    // ^
    Cname, // Alias
    Soa,   // Start of a Zone of Authority
    Mb,    // Mailbox Domain Name - Experimental
    Mg,    // Mail Group Member - Experimental
    Mr,    // Mail Rename Domain Name - Experimental
    Null,  // Empty (NULL - duh)
    Wks,   // Well Known Service Description
    Ptr,   // Domain Name Pointer
    Hinfo, // Host Information
    Minfo, // Mailbox/Mail List Information
    Mx,    // Mail Exchange
    Txt,   // Text Strings

    // QTYPEs
    Axfr = 252,  // Zone Transfer Request
    Mailb = 253, // Mailbox-related Records
    Maila = 254, // Mail Agent RRs (obsolete - see MX)
    All = 255,   // All Records
}
