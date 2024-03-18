#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dmac_dsr: DmacDsr,
    dmac_dpc: DmacDpc,
    _reserved2: [u8; 0x18],
    dmac_inten: DmacInten,
    dmac_event_ris: DmacEventRis,
    dmac_intmis: DmacIntmis,
    dmac_intclr: DmacIntclr,
    dmac_fsrd: DmacFsrd,
    dmac_fsrc: DmacFsrc,
    dmac_ftrd: DmacFtrd,
    _reserved9: [u8; 0x04],
    dmac_ftr0: DmacFtr0,
    _reserved10: [u8; 0xbc],
    _reserved_10_dmac: [u8; 0x24],
    _reserved11: [u8; 0x02dc],
    _reserved_11_dmac: [u8; 0x30],
    _reserved12: [u8; 0x08d0],
    dmac_dbgstatus: DmacDbgstatus,
    dmac_dbgcmd: DmacDbgcmd,
    dmac_dbginst0: DmacDbginst0,
    dmac_dbginst1: DmacDbginst1,
    _reserved16: [u8; 0xf0],
    dmac_cr0: DmacCr0,
    dmac_cr1: DmacCr1,
    dmac_cr2: DmacCr2,
    dmac_cr3: DmacCr3,
    dmac_cr4: DmacCr4,
    dmac_crdn: DmacCrdn,
    _reserved22: [u8; 0x68],
    dmac_wd: DmacWd,
}
impl RegisterBlock {
    #[doc = "0x00 - DMA Manager Status Register"]
    #[inline(always)]
    pub const fn dmac_dsr(&self) -> &DmacDsr {
        &self.dmac_dsr
    }
    #[doc = "0x04 - DMA Program Counter Register"]
    #[inline(always)]
    pub const fn dmac_dpc(&self) -> &DmacDpc {
        &self.dmac_dpc
    }
    #[doc = "0x20 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn dmac_inten(&self) -> &DmacInten {
        &self.dmac_inten
    }
    #[doc = "0x24 - Event-Interrupt Raw Status Register"]
    #[inline(always)]
    pub const fn dmac_event_ris(&self) -> &DmacEventRis {
        &self.dmac_event_ris
    }
    #[doc = "0x28 - Interrupt Status Register"]
    #[inline(always)]
    pub const fn dmac_intmis(&self) -> &DmacIntmis {
        &self.dmac_intmis
    }
    #[doc = "0x2c - Interrupt Clear Register"]
    #[inline(always)]
    pub const fn dmac_intclr(&self) -> &DmacIntclr {
        &self.dmac_intclr
    }
    #[doc = "0x30 - Fault Status DMA Manager Register"]
    #[inline(always)]
    pub const fn dmac_fsrd(&self) -> &DmacFsrd {
        &self.dmac_fsrd
    }
    #[doc = "0x34 - Fault Status DMA Channel Register"]
    #[inline(always)]
    pub const fn dmac_fsrc(&self) -> &DmacFsrc {
        &self.dmac_fsrc
    }
    #[doc = "0x38 - Fault Type DMA Manager Register"]
    #[inline(always)]
    pub const fn dmac_ftrd(&self) -> &DmacFtrd {
        &self.dmac_ftrd
    }
    #[doc = "0x40 - Fault Type DMA Channel Register"]
    #[inline(always)]
    pub const fn dmac_ftr0(&self) -> &DmacFtr0 {
        &self.dmac_ftr0
    }
    #[doc = "0x100..0x120 - Channel Status Registers"]
    #[inline(always)]
    pub const fn dmac_csr(&self, n: usize) -> &DmacCsr {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(256)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x120 - Channel Status Registers"]
    #[inline(always)]
    pub fn dmac_csr_iter(&self) -> impl Iterator<Item = &DmacCsr> {
        (0..8).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(256)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x104..0x124 - Channel Program Counter Registers"]
    #[inline(always)]
    pub const fn dmac_cpc(&self, n: usize) -> &DmacCpc {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(260)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x104..0x124 - Channel Program Counter Registers"]
    #[inline(always)]
    pub fn dmac_cpc_iter(&self) -> impl Iterator<Item = &DmacCpc> {
        (0..8).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(260)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x400..0x420 - Source Address Registers"]
    #[inline(always)]
    pub const fn dmac_sar(&self, n: usize) -> &DmacSar {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(1024)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x400..0x420 - Source Address Registers"]
    #[inline(always)]
    pub fn dmac_sar_iter(&self) -> impl Iterator<Item = &DmacSar> {
        (0..8).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(1024)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x404..0x424 - DestinationAddress Registers"]
    #[inline(always)]
    pub const fn dmac_dar(&self, n: usize) -> &DmacDar {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(1028)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x404..0x424 - DestinationAddress Registers"]
    #[inline(always)]
    pub fn dmac_dar_iter(&self) -> impl Iterator<Item = &DmacDar> {
        (0..8).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(1028)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x408..0x428 - Channel Control Registers"]
    #[inline(always)]
    pub const fn dmac_ccr(&self, n: usize) -> &DmacCcr {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(1032)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x408..0x428 - Channel Control Registers"]
    #[inline(always)]
    pub fn dmac_ccr_iter(&self) -> impl Iterator<Item = &DmacCcr> {
        (0..8).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(1032)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x40c..0x42c - Loop Counter 0 Registers"]
    #[inline(always)]
    pub const fn dmac_lc0_(&self, n: usize) -> &DmacLc0_ {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(1036)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x40c..0x42c - Loop Counter 0 Registers"]
    #[inline(always)]
    pub fn dmac_lc0__iter(&self) -> impl Iterator<Item = &DmacLc0_> {
        (0..8).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(1036)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x410..0x430 - Loop Counter 1 Registers"]
    #[inline(always)]
    pub const fn dmac_lc1_(&self, n: usize) -> &DmacLc1_ {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(1040)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x410..0x430 - Loop Counter 1 Registers"]
    #[inline(always)]
    pub fn dmac_lc1__iter(&self) -> impl Iterator<Item = &DmacLc1_> {
        (0..8).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(1040)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0xd00 - Debug Status Register"]
    #[inline(always)]
    pub const fn dmac_dbgstatus(&self) -> &DmacDbgstatus {
        &self.dmac_dbgstatus
    }
    #[doc = "0xd04 - Debug Command Register"]
    #[inline(always)]
    pub const fn dmac_dbgcmd(&self) -> &DmacDbgcmd {
        &self.dmac_dbgcmd
    }
    #[doc = "0xd08 - Debug Instruction-0 Register"]
    #[inline(always)]
    pub const fn dmac_dbginst0(&self) -> &DmacDbginst0 {
        &self.dmac_dbginst0
    }
    #[doc = "0xd0c - Debug Instruction-1 Register"]
    #[inline(always)]
    pub const fn dmac_dbginst1(&self) -> &DmacDbginst1 {
        &self.dmac_dbginst1
    }
    #[doc = "0xe00 - Configuration Register 0"]
    #[inline(always)]
    pub const fn dmac_cr0(&self) -> &DmacCr0 {
        &self.dmac_cr0
    }
    #[doc = "0xe04 - Configuration Register 1"]
    #[inline(always)]
    pub const fn dmac_cr1(&self) -> &DmacCr1 {
        &self.dmac_cr1
    }
    #[doc = "0xe08 - Configuration Register 2"]
    #[inline(always)]
    pub const fn dmac_cr2(&self) -> &DmacCr2 {
        &self.dmac_cr2
    }
    #[doc = "0xe0c - Configuration Register 3"]
    #[inline(always)]
    pub const fn dmac_cr3(&self) -> &DmacCr3 {
        &self.dmac_cr3
    }
    #[doc = "0xe10 - Configuration Register 4"]
    #[inline(always)]
    pub const fn dmac_cr4(&self) -> &DmacCr4 {
        &self.dmac_cr4
    }
    #[doc = "0xe14 - DMA Configuration Register"]
    #[inline(always)]
    pub const fn dmac_crdn(&self) -> &DmacCrdn {
        &self.dmac_crdn
    }
    #[doc = "0xe80 - DMA Watchdog Register"]
    #[inline(always)]
    pub const fn dmac_wd(&self) -> &DmacWd {
        &self.dmac_wd
    }
}
#[doc = "DMAC_DSR (r) register accessor: DMA Manager Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_dsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmac_dsr`]
module"]
#[doc(alias = "DMAC_DSR")]
pub type DmacDsr = crate::Reg<dmac_dsr::DmacDsrSpec>;
#[doc = "DMA Manager Status Register"]
pub mod dmac_dsr;
#[doc = "DMAC_DPC (r) register accessor: DMA Program Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_dpc::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmac_dpc`]
module"]
#[doc(alias = "DMAC_DPC")]
pub type DmacDpc = crate::Reg<dmac_dpc::DmacDpcSpec>;
#[doc = "DMA Program Counter Register"]
pub mod dmac_dpc;
#[doc = "DMAC_INTEN (rw) register accessor: Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmac_inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmac_inten`]
module"]
#[doc(alias = "DMAC_INTEN")]
pub type DmacInten = crate::Reg<dmac_inten::DmacIntenSpec>;
#[doc = "Interrupt Enable Register"]
pub mod dmac_inten;
#[doc = "DMAC_EVENT_RIS (r) register accessor: Event-Interrupt Raw Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_event_ris::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmac_event_ris`]
module"]
#[doc(alias = "DMAC_EVENT_RIS")]
pub type DmacEventRis = crate::Reg<dmac_event_ris::DmacEventRisSpec>;
#[doc = "Event-Interrupt Raw Status Register"]
pub mod dmac_event_ris;
#[doc = "DMAC_INTMIS (r) register accessor: Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_intmis::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmac_intmis`]
module"]
#[doc(alias = "DMAC_INTMIS")]
pub type DmacIntmis = crate::Reg<dmac_intmis::DmacIntmisSpec>;
#[doc = "Interrupt Status Register"]
pub mod dmac_intmis;
#[doc = "DMAC_INTCLR (w) register accessor: Interrupt Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmac_intclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmac_intclr`]
module"]
#[doc(alias = "DMAC_INTCLR")]
pub type DmacIntclr = crate::Reg<dmac_intclr::DmacIntclrSpec>;
#[doc = "Interrupt Clear Register"]
pub mod dmac_intclr;
#[doc = "DMAC_FSRD (r) register accessor: Fault Status DMA Manager Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_fsrd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmac_fsrd`]
module"]
#[doc(alias = "DMAC_FSRD")]
pub type DmacFsrd = crate::Reg<dmac_fsrd::DmacFsrdSpec>;
#[doc = "Fault Status DMA Manager Register"]
pub mod dmac_fsrd;
#[doc = "DMAC_FSRC (r) register accessor: Fault Status DMA Channel Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_fsrc::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmac_fsrc`]
module"]
#[doc(alias = "DMAC_FSRC")]
pub type DmacFsrc = crate::Reg<dmac_fsrc::DmacFsrcSpec>;
#[doc = "Fault Status DMA Channel Register"]
pub mod dmac_fsrc;
#[doc = "DMAC_FTRD (rw) register accessor: Fault Type DMA Manager Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_ftrd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmac_ftrd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmac_ftrd`]
module"]
#[doc(alias = "DMAC_FTRD")]
pub type DmacFtrd = crate::Reg<dmac_ftrd::DmacFtrdSpec>;
#[doc = "Fault Type DMA Manager Register"]
pub mod dmac_ftrd;
#[doc = "DMAC_FTR0 (r) register accessor: Fault Type DMA Channel Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_ftr0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmac_ftr0`]
module"]
#[doc(alias = "DMAC_FTR0")]
pub type DmacFtr0 = crate::Reg<dmac_ftr0::DmacFtr0Spec>;
#[doc = "Fault Type DMA Channel Register"]
pub mod dmac_ftr0;
#[doc = "DMAC_CSR (r) register accessor: Channel Status Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_csr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmac_csr`]
module"]
#[doc(alias = "DMAC_CSR")]
pub type DmacCsr = crate::Reg<dmac_csr::DmacCsrSpec>;
#[doc = "Channel Status Registers"]
pub mod dmac_csr;
#[doc = "DMAC_CPC (r) register accessor: Channel Program Counter Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_cpc::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmac_cpc`]
module"]
#[doc(alias = "DMAC_CPC")]
pub type DmacCpc = crate::Reg<dmac_cpc::DmacCpcSpec>;
#[doc = "Channel Program Counter Registers"]
pub mod dmac_cpc;
#[doc = "DMAC_SAR (r) register accessor: Source Address Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_sar::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmac_sar`]
module"]
#[doc(alias = "DMAC_SAR")]
pub type DmacSar = crate::Reg<dmac_sar::DmacSarSpec>;
#[doc = "Source Address Registers"]
pub mod dmac_sar;
#[doc = "DMAC_DAR (r) register accessor: DestinationAddress Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_dar::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmac_dar`]
module"]
#[doc(alias = "DMAC_DAR")]
pub type DmacDar = crate::Reg<dmac_dar::DmacDarSpec>;
#[doc = "DestinationAddress Registers"]
pub mod dmac_dar;
#[doc = "DMAC_CCR (r) register accessor: Channel Control Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_ccr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmac_ccr`]
module"]
#[doc(alias = "DMAC_CCR")]
pub type DmacCcr = crate::Reg<dmac_ccr::DmacCcrSpec>;
#[doc = "Channel Control Registers"]
pub mod dmac_ccr;
#[doc = "DMAC_LC0_ (r) register accessor: Loop Counter 0 Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_lc0_::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmac_lc0_`]
module"]
#[doc(alias = "DMAC_LC0_")]
pub type DmacLc0_ = crate::Reg<dmac_lc0_::DmacLc0_Spec>;
#[doc = "Loop Counter 0 Registers"]
pub mod dmac_lc0_;
#[doc = "DMAC_LC1_ (r) register accessor: Loop Counter 1 Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_lc1_::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmac_lc1_`]
module"]
#[doc(alias = "DMAC_LC1_")]
pub type DmacLc1_ = crate::Reg<dmac_lc1_::DmacLc1_Spec>;
#[doc = "Loop Counter 1 Registers"]
pub mod dmac_lc1_;
#[doc = "DMAC_DBGSTATUS (r) register accessor: Debug Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_dbgstatus::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmac_dbgstatus`]
module"]
#[doc(alias = "DMAC_DBGSTATUS")]
pub type DmacDbgstatus = crate::Reg<dmac_dbgstatus::DmacDbgstatusSpec>;
#[doc = "Debug Status Register"]
pub mod dmac_dbgstatus;
#[doc = "DMAC_DBGCMD (w) register accessor: Debug Command Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmac_dbgcmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmac_dbgcmd`]
module"]
#[doc(alias = "DMAC_DBGCMD")]
pub type DmacDbgcmd = crate::Reg<dmac_dbgcmd::DmacDbgcmdSpec>;
#[doc = "Debug Command Register"]
pub mod dmac_dbgcmd;
#[doc = "DMAC_DBGINST0 (w) register accessor: Debug Instruction-0 Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmac_dbginst0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmac_dbginst0`]
module"]
#[doc(alias = "DMAC_DBGINST0")]
pub type DmacDbginst0 = crate::Reg<dmac_dbginst0::DmacDbginst0Spec>;
#[doc = "Debug Instruction-0 Register"]
pub mod dmac_dbginst0;
#[doc = "DMAC_DBGINST1 (w) register accessor: Debug Instruction-1 Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmac_dbginst1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmac_dbginst1`]
module"]
#[doc(alias = "DMAC_DBGINST1")]
pub type DmacDbginst1 = crate::Reg<dmac_dbginst1::DmacDbginst1Spec>;
#[doc = "Debug Instruction-1 Register"]
pub mod dmac_dbginst1;
#[doc = "DMAC_CR0 (r) register accessor: Configuration Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_cr0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmac_cr0`]
module"]
#[doc(alias = "DMAC_CR0")]
pub type DmacCr0 = crate::Reg<dmac_cr0::DmacCr0Spec>;
#[doc = "Configuration Register 0"]
pub mod dmac_cr0;
#[doc = "DMAC_CR1 (r) register accessor: Configuration Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_cr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmac_cr1`]
module"]
#[doc(alias = "DMAC_CR1")]
pub type DmacCr1 = crate::Reg<dmac_cr1::DmacCr1Spec>;
#[doc = "Configuration Register 1"]
pub mod dmac_cr1;
#[doc = "DMAC_CR2 (r) register accessor: Configuration Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_cr2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmac_cr2`]
module"]
#[doc(alias = "DMAC_CR2")]
pub type DmacCr2 = crate::Reg<dmac_cr2::DmacCr2Spec>;
#[doc = "Configuration Register 2"]
pub mod dmac_cr2;
#[doc = "DMAC_CR3 (r) register accessor: Configuration Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_cr3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmac_cr3`]
module"]
#[doc(alias = "DMAC_CR3")]
pub type DmacCr3 = crate::Reg<dmac_cr3::DmacCr3Spec>;
#[doc = "Configuration Register 3"]
pub mod dmac_cr3;
#[doc = "DMAC_CR4 (r) register accessor: Configuration Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_cr4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmac_cr4`]
module"]
#[doc(alias = "DMAC_CR4")]
pub type DmacCr4 = crate::Reg<dmac_cr4::DmacCr4Spec>;
#[doc = "Configuration Register 4"]
pub mod dmac_cr4;
#[doc = "DMAC_CRDn (r) register accessor: DMA Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_crdn::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmac_crdn`]
module"]
#[doc(alias = "DMAC_CRDn")]
pub type DmacCrdn = crate::Reg<dmac_crdn::DmacCrdnSpec>;
#[doc = "DMA Configuration Register"]
pub mod dmac_crdn;
#[doc = "DMAC_WD (rw) register accessor: DMA Watchdog Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_wd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmac_wd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmac_wd`]
module"]
#[doc(alias = "DMAC_WD")]
pub type DmacWd = crate::Reg<dmac_wd::DmacWdSpec>;
#[doc = "DMA Watchdog Register"]
pub mod dmac_wd;
