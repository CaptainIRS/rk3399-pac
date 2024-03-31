#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cfgr: Cfgr,
    sdblr: Sdblr,
    dmacr: Dmacr,
    intcr: Intcr,
    intsr: Intsr,
    _reserved5: [u8; 0x04],
    xfer: Xfer,
    _reserved6: [u8; 0x04],
    smpdr: Smpdr,
    _reserved7: [u8; 0x3c],
    vldfrn: Vldfrn,
    _reserved8: [u8; 0x2c],
    usrdrn: Usrdrn,
    _reserved9: [u8; 0x2c],
    chnsrn: Chnsrn,
    _reserved10: [u8; 0x3c],
    burtsinfo: Burtsinfo,
    repettion: Repettion,
    burtsinfo_shd: BurtsinfoShd,
    repettion_shd: RepettionShd,
    _reserved14: [u8; 0x80],
    usrdr_shdn: UsrdrShdn,
}
impl RegisterBlock {
    #[doc = "0x00 - Transfer Configuration Register"]
    #[inline(always)]
    pub const fn cfgr(&self) -> &Cfgr {
        &self.cfgr
    }
    #[doc = "0x04 - Sample Date Buffer Level Register"]
    #[inline(always)]
    pub const fn sdblr(&self) -> &Sdblr {
        &self.sdblr
    }
    #[doc = "0x08 - DMA Control Register"]
    #[inline(always)]
    pub const fn dmacr(&self) -> &Dmacr {
        &self.dmacr
    }
    #[doc = "0x0c - Interrupt Control Register"]
    #[inline(always)]
    pub const fn intcr(&self) -> &Intcr {
        &self.intcr
    }
    #[doc = "0x10 - Interrupt Status Register"]
    #[inline(always)]
    pub const fn intsr(&self) -> &Intsr {
        &self.intsr
    }
    #[doc = "0x18 - Transfer Start Register"]
    #[inline(always)]
    pub const fn xfer(&self) -> &Xfer {
        &self.xfer
    }
    #[doc = "0x20 - Sample Data Register"]
    #[inline(always)]
    pub const fn smpdr(&self) -> &Smpdr {
        &self.smpdr
    }
    #[doc = "0x60 - Validity Flag Register n"]
    #[inline(always)]
    pub const fn vldfrn(&self) -> &Vldfrn {
        &self.vldfrn
    }
    #[doc = "0x90 - User Data Register n"]
    #[inline(always)]
    pub const fn usrdrn(&self) -> &Usrdrn {
        &self.usrdrn
    }
    #[doc = "0xc0 - Channel Status Register n"]
    #[inline(always)]
    pub const fn chnsrn(&self) -> &Chnsrn {
        &self.chnsrn
    }
    #[doc = "0x100 - Channel Burst Info Register"]
    #[inline(always)]
    pub const fn burtsinfo(&self) -> &Burtsinfo {
        &self.burtsinfo
    }
    #[doc = "0x104 - Channel Repetition Register"]
    #[inline(always)]
    pub const fn repettion(&self) -> &Repettion {
        &self.repettion
    }
    #[doc = "0x108 - Shadow Channel Burst Info Register"]
    #[inline(always)]
    pub const fn burtsinfo_shd(&self) -> &BurtsinfoShd {
        &self.burtsinfo_shd
    }
    #[doc = "0x10c - Shadow Channel Repetition Register"]
    #[inline(always)]
    pub const fn repettion_shd(&self) -> &RepettionShd {
        &self.repettion_shd
    }
    #[doc = "0x190 - Shadow User Data Register n"]
    #[inline(always)]
    pub const fn usrdr_shdn(&self) -> &UsrdrShdn {
        &self.usrdr_shdn
    }
}
#[doc = "CFGR (rw) register accessor: Transfer Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr`]
module"]
#[doc(alias = "CFGR")]
pub type Cfgr = crate::Reg<cfgr::CfgrSpec>;
#[doc = "Transfer Configuration Register"]
pub mod cfgr;
#[doc = "SDBLR (rw) register accessor: Sample Date Buffer Level Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdblr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdblr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdblr`]
module"]
#[doc(alias = "SDBLR")]
pub type Sdblr = crate::Reg<sdblr::SdblrSpec>;
#[doc = "Sample Date Buffer Level Register"]
pub mod sdblr;
#[doc = "DMACR (rw) register accessor: DMA Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmacr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmacr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacr`]
module"]
#[doc(alias = "DMACR")]
pub type Dmacr = crate::Reg<dmacr::DmacrSpec>;
#[doc = "DMA Control Register"]
pub mod dmacr;
#[doc = "INTCR (rw) register accessor: Interrupt Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intcr`]
module"]
#[doc(alias = "INTCR")]
pub type Intcr = crate::Reg<intcr::IntcrSpec>;
#[doc = "Interrupt Control Register"]
pub mod intcr;
#[doc = "INTSR (rw) register accessor: Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intsr`]
module"]
#[doc(alias = "INTSR")]
pub type Intsr = crate::Reg<intsr::IntsrSpec>;
#[doc = "Interrupt Status Register"]
pub mod intsr;
#[doc = "XFER (rw) register accessor: Transfer Start Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xfer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xfer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xfer`]
module"]
#[doc(alias = "XFER")]
pub type Xfer = crate::Reg<xfer::XferSpec>;
#[doc = "Transfer Start Register"]
pub mod xfer;
#[doc = "SMPDR (rw) register accessor: Sample Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smpdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smpdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smpdr`]
module"]
#[doc(alias = "SMPDR")]
pub type Smpdr = crate::Reg<smpdr::SmpdrSpec>;
#[doc = "Sample Data Register"]
pub mod smpdr;
#[doc = "VLDFRn (rw) register accessor: Validity Flag Register n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vldfrn::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vldfrn::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vldfrn`]
module"]
#[doc(alias = "VLDFRn")]
pub type Vldfrn = crate::Reg<vldfrn::VldfrnSpec>;
#[doc = "Validity Flag Register n"]
pub mod vldfrn;
#[doc = "USRDRn (rw) register accessor: User Data Register n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usrdrn::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usrdrn::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usrdrn`]
module"]
#[doc(alias = "USRDRn")]
pub type Usrdrn = crate::Reg<usrdrn::UsrdrnSpec>;
#[doc = "User Data Register n"]
pub mod usrdrn;
#[doc = "CHNSRn (rw) register accessor: Channel Status Register n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chnsrn::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chnsrn::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chnsrn`]
module"]
#[doc(alias = "CHNSRn")]
pub type Chnsrn = crate::Reg<chnsrn::ChnsrnSpec>;
#[doc = "Channel Status Register n"]
pub mod chnsrn;
#[doc = "BURTSINFO (rw) register accessor: Channel Burst Info Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`burtsinfo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`burtsinfo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@burtsinfo`]
module"]
#[doc(alias = "BURTSINFO")]
pub type Burtsinfo = crate::Reg<burtsinfo::BurtsinfoSpec>;
#[doc = "Channel Burst Info Register"]
pub mod burtsinfo;
#[doc = "REPETTION (rw) register accessor: Channel Repetition Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`repettion::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`repettion::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@repettion`]
module"]
#[doc(alias = "REPETTION")]
pub type Repettion = crate::Reg<repettion::RepettionSpec>;
#[doc = "Channel Repetition Register"]
pub mod repettion;
#[doc = "BURTSINFO_SHD (r) register accessor: Shadow Channel Burst Info Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`burtsinfo_shd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@burtsinfo_shd`]
module"]
#[doc(alias = "BURTSINFO_SHD")]
pub type BurtsinfoShd = crate::Reg<burtsinfo_shd::BurtsinfoShdSpec>;
#[doc = "Shadow Channel Burst Info Register"]
pub mod burtsinfo_shd;
#[doc = "REPETTION_SHD (r) register accessor: Shadow Channel Repetition Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`repettion_shd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@repettion_shd`]
module"]
#[doc(alias = "REPETTION_SHD")]
pub type RepettionShd = crate::Reg<repettion_shd::RepettionShdSpec>;
#[doc = "Shadow Channel Repetition Register"]
pub mod repettion_shd;
#[doc = "USRDR_SHDn (r) register accessor: Shadow User Data Register n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usrdr_shdn::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usrdr_shdn`]
module"]
#[doc(alias = "USRDR_SHDn")]
pub type UsrdrShdn = crate::Reg<usrdr_shdn::UsrdrShdnSpec>;
#[doc = "Shadow User Data Register n"]
pub mod usrdr_shdn;
