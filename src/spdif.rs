#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    spdif_cfgr: SpdifCfgr,
    spdif_sdblr: SpdifSdblr,
    spdif_dmacr: SpdifDmacr,
    spdif_intcr: SpdifIntcr,
    spdif_intsr: SpdifIntsr,
    _reserved5: [u8; 0x04],
    spdif_xfer: SpdifXfer,
    _reserved6: [u8; 0x04],
    spdif_smpdr: SpdifSmpdr,
    _reserved7: [u8; 0x3c],
    spdif_vldfrn: SpdifVldfrn,
    _reserved8: [u8; 0x2c],
    spdif_usrdrn: SpdifUsrdrn,
    _reserved9: [u8; 0x2c],
    spdif_chnsrn: SpdifChnsrn,
    _reserved10: [u8; 0x3c],
    spdif_burtsinfo: SpdifBurtsinfo,
    spdif_repettion: SpdifRepettion,
    spdif_burtsinfo_shd: SpdifBurtsinfoShd,
    spdif_repettion_shd: SpdifRepettionShd,
    _reserved14: [u8; 0x80],
    spdif_usrdr_shdn: SpdifUsrdrShdn,
}
impl RegisterBlock {
    #[doc = "0x00 - Transfer Configuration Register"]
    #[inline(always)]
    pub const fn spdif_cfgr(&self) -> &SpdifCfgr {
        &self.spdif_cfgr
    }
    #[doc = "0x04 - Sample Date Buffer Level Register"]
    #[inline(always)]
    pub const fn spdif_sdblr(&self) -> &SpdifSdblr {
        &self.spdif_sdblr
    }
    #[doc = "0x08 - DMA Control Register"]
    #[inline(always)]
    pub const fn spdif_dmacr(&self) -> &SpdifDmacr {
        &self.spdif_dmacr
    }
    #[doc = "0x0c - Interrupt Control Register"]
    #[inline(always)]
    pub const fn spdif_intcr(&self) -> &SpdifIntcr {
        &self.spdif_intcr
    }
    #[doc = "0x10 - Interrupt Status Register"]
    #[inline(always)]
    pub const fn spdif_intsr(&self) -> &SpdifIntsr {
        &self.spdif_intsr
    }
    #[doc = "0x18 - Transfer Start Register"]
    #[inline(always)]
    pub const fn spdif_xfer(&self) -> &SpdifXfer {
        &self.spdif_xfer
    }
    #[doc = "0x20 - Sample Data Register"]
    #[inline(always)]
    pub const fn spdif_smpdr(&self) -> &SpdifSmpdr {
        &self.spdif_smpdr
    }
    #[doc = "0x60 - Validity Flag Register n"]
    #[inline(always)]
    pub const fn spdif_vldfrn(&self) -> &SpdifVldfrn {
        &self.spdif_vldfrn
    }
    #[doc = "0x90 - User Data Register n"]
    #[inline(always)]
    pub const fn spdif_usrdrn(&self) -> &SpdifUsrdrn {
        &self.spdif_usrdrn
    }
    #[doc = "0xc0 - Channel Status Register n"]
    #[inline(always)]
    pub const fn spdif_chnsrn(&self) -> &SpdifChnsrn {
        &self.spdif_chnsrn
    }
    #[doc = "0x100 - Channel Burst Info Register"]
    #[inline(always)]
    pub const fn spdif_burtsinfo(&self) -> &SpdifBurtsinfo {
        &self.spdif_burtsinfo
    }
    #[doc = "0x104 - Channel Repetition Register"]
    #[inline(always)]
    pub const fn spdif_repettion(&self) -> &SpdifRepettion {
        &self.spdif_repettion
    }
    #[doc = "0x108 - Shadow Channel Burst Info Register"]
    #[inline(always)]
    pub const fn spdif_burtsinfo_shd(&self) -> &SpdifBurtsinfoShd {
        &self.spdif_burtsinfo_shd
    }
    #[doc = "0x10c - Shadow Channel Repetition Register"]
    #[inline(always)]
    pub const fn spdif_repettion_shd(&self) -> &SpdifRepettionShd {
        &self.spdif_repettion_shd
    }
    #[doc = "0x190 - Shadow User Data Register n"]
    #[inline(always)]
    pub const fn spdif_usrdr_shdn(&self) -> &SpdifUsrdrShdn {
        &self.spdif_usrdr_shdn
    }
}
#[doc = "SPDIF_CFGR (rw) register accessor: Transfer Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spdif_cfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spdif_cfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spdif_cfgr`]
module"]
#[doc(alias = "SPDIF_CFGR")]
pub type SpdifCfgr = crate::Reg<spdif_cfgr::SpdifCfgrSpec>;
#[doc = "Transfer Configuration Register"]
pub mod spdif_cfgr;
#[doc = "SPDIF_SDBLR (rw) register accessor: Sample Date Buffer Level Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spdif_sdblr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spdif_sdblr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spdif_sdblr`]
module"]
#[doc(alias = "SPDIF_SDBLR")]
pub type SpdifSdblr = crate::Reg<spdif_sdblr::SpdifSdblrSpec>;
#[doc = "Sample Date Buffer Level Register"]
pub mod spdif_sdblr;
#[doc = "SPDIF_DMACR (rw) register accessor: DMA Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spdif_dmacr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spdif_dmacr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spdif_dmacr`]
module"]
#[doc(alias = "SPDIF_DMACR")]
pub type SpdifDmacr = crate::Reg<spdif_dmacr::SpdifDmacrSpec>;
#[doc = "DMA Control Register"]
pub mod spdif_dmacr;
#[doc = "SPDIF_INTCR (rw) register accessor: Interrupt Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spdif_intcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spdif_intcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spdif_intcr`]
module"]
#[doc(alias = "SPDIF_INTCR")]
pub type SpdifIntcr = crate::Reg<spdif_intcr::SpdifIntcrSpec>;
#[doc = "Interrupt Control Register"]
pub mod spdif_intcr;
#[doc = "SPDIF_INTSR (rw) register accessor: Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spdif_intsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spdif_intsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spdif_intsr`]
module"]
#[doc(alias = "SPDIF_INTSR")]
pub type SpdifIntsr = crate::Reg<spdif_intsr::SpdifIntsrSpec>;
#[doc = "Interrupt Status Register"]
pub mod spdif_intsr;
#[doc = "SPDIF_XFER (rw) register accessor: Transfer Start Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spdif_xfer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spdif_xfer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spdif_xfer`]
module"]
#[doc(alias = "SPDIF_XFER")]
pub type SpdifXfer = crate::Reg<spdif_xfer::SpdifXferSpec>;
#[doc = "Transfer Start Register"]
pub mod spdif_xfer;
#[doc = "SPDIF_SMPDR (rw) register accessor: Sample Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spdif_smpdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spdif_smpdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spdif_smpdr`]
module"]
#[doc(alias = "SPDIF_SMPDR")]
pub type SpdifSmpdr = crate::Reg<spdif_smpdr::SpdifSmpdrSpec>;
#[doc = "Sample Data Register"]
pub mod spdif_smpdr;
#[doc = "SPDIF_VLDFRn (rw) register accessor: Validity Flag Register n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spdif_vldfrn::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spdif_vldfrn::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spdif_vldfrn`]
module"]
#[doc(alias = "SPDIF_VLDFRn")]
pub type SpdifVldfrn = crate::Reg<spdif_vldfrn::SpdifVldfrnSpec>;
#[doc = "Validity Flag Register n"]
pub mod spdif_vldfrn;
#[doc = "SPDIF_USRDRn (rw) register accessor: User Data Register n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spdif_usrdrn::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spdif_usrdrn::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spdif_usrdrn`]
module"]
#[doc(alias = "SPDIF_USRDRn")]
pub type SpdifUsrdrn = crate::Reg<spdif_usrdrn::SpdifUsrdrnSpec>;
#[doc = "User Data Register n"]
pub mod spdif_usrdrn;
#[doc = "SPDIF_CHNSRn (rw) register accessor: Channel Status Register n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spdif_chnsrn::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spdif_chnsrn::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spdif_chnsrn`]
module"]
#[doc(alias = "SPDIF_CHNSRn")]
pub type SpdifChnsrn = crate::Reg<spdif_chnsrn::SpdifChnsrnSpec>;
#[doc = "Channel Status Register n"]
pub mod spdif_chnsrn;
#[doc = "SPDIF_BURTSINFO (rw) register accessor: Channel Burst Info Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spdif_burtsinfo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spdif_burtsinfo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spdif_burtsinfo`]
module"]
#[doc(alias = "SPDIF_BURTSINFO")]
pub type SpdifBurtsinfo = crate::Reg<spdif_burtsinfo::SpdifBurtsinfoSpec>;
#[doc = "Channel Burst Info Register"]
pub mod spdif_burtsinfo;
#[doc = "SPDIF_REPETTION (rw) register accessor: Channel Repetition Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spdif_repettion::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spdif_repettion::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spdif_repettion`]
module"]
#[doc(alias = "SPDIF_REPETTION")]
pub type SpdifRepettion = crate::Reg<spdif_repettion::SpdifRepettionSpec>;
#[doc = "Channel Repetition Register"]
pub mod spdif_repettion;
#[doc = "SPDIF_BURTSINFO_SHD (r) register accessor: Shadow Channel Burst Info Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spdif_burtsinfo_shd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spdif_burtsinfo_shd`]
module"]
#[doc(alias = "SPDIF_BURTSINFO_SHD")]
pub type SpdifBurtsinfoShd = crate::Reg<spdif_burtsinfo_shd::SpdifBurtsinfoShdSpec>;
#[doc = "Shadow Channel Burst Info Register"]
pub mod spdif_burtsinfo_shd;
#[doc = "SPDIF_REPETTION_SHD (r) register accessor: Shadow Channel Repetition Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spdif_repettion_shd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spdif_repettion_shd`]
module"]
#[doc(alias = "SPDIF_REPETTION_SHD")]
pub type SpdifRepettionShd = crate::Reg<spdif_repettion_shd::SpdifRepettionShdSpec>;
#[doc = "Shadow Channel Repetition Register"]
pub mod spdif_repettion_shd;
#[doc = "SPDIF_USRDR_SHDn (r) register accessor: Shadow User Data Register n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spdif_usrdr_shdn::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spdif_usrdr_shdn`]
module"]
#[doc(alias = "SPDIF_USRDR_SHDn")]
pub type SpdifUsrdrShdn = crate::Reg<spdif_usrdr_shdn::SpdifUsrdrShdnSpec>;
#[doc = "Shadow User Data Register n"]
pub mod spdif_usrdr_shdn;
