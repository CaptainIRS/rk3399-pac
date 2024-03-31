#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dsr: Dsr,
    dpc: Dpc,
    _reserved2: [u8; 0x18],
    inten: Inten,
    event_ris: EventRis,
    intmis: Intmis,
    intclr: Intclr,
    fsrd: Fsrd,
    fsrc: Fsrc,
    ftrd: Ftrd,
    _reserved9: [u8; 0x04],
    ftr0: Ftr0,
    _reserved10: [u8; 0xbc],
    csr: (),
    _reserved11: [u8; 0x04],
    cpc: (),
    _reserved12: [u8; 0x02fc],
    sar: (),
    _reserved13: [u8; 0x04],
    dar: (),
    _reserved14: [u8; 0x04],
    ccr: (),
    _reserved15: [u8; 0x04],
    lc0_: (),
    _reserved16: [u8; 0x04],
    lc1_: (),
    _reserved17: [u8; 0x08f0],
    dbgstatus: Dbgstatus,
    dbgcmd: Dbgcmd,
    dbginst0: Dbginst0,
    dbginst1: Dbginst1,
    _reserved21: [u8; 0xf0],
    cr0: Cr0,
    cr1: Cr1,
    cr2: Cr2,
    cr3: Cr3,
    cr4: Cr4,
    crdn: Crdn,
    _reserved27: [u8; 0x68],
    wd: Wd,
}
impl RegisterBlock {
    #[doc = "0x00 - DMA Manager Status Register"]
    #[inline(always)]
    pub const fn dsr(&self) -> &Dsr {
        &self.dsr
    }
    #[doc = "0x04 - DMA Program Counter Register"]
    #[inline(always)]
    pub const fn dpc(&self) -> &Dpc {
        &self.dpc
    }
    #[doc = "0x20 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn inten(&self) -> &Inten {
        &self.inten
    }
    #[doc = "0x24 - Event-Interrupt Raw Status Register"]
    #[inline(always)]
    pub const fn event_ris(&self) -> &EventRis {
        &self.event_ris
    }
    #[doc = "0x28 - Interrupt Status Register"]
    #[inline(always)]
    pub const fn intmis(&self) -> &Intmis {
        &self.intmis
    }
    #[doc = "0x2c - Interrupt Clear Register"]
    #[inline(always)]
    pub const fn intclr(&self) -> &Intclr {
        &self.intclr
    }
    #[doc = "0x30 - Fault Status DMA Manager Register"]
    #[inline(always)]
    pub const fn fsrd(&self) -> &Fsrd {
        &self.fsrd
    }
    #[doc = "0x34 - Fault Status DMA Channel Register"]
    #[inline(always)]
    pub const fn fsrc(&self) -> &Fsrc {
        &self.fsrc
    }
    #[doc = "0x38 - Fault Type DMA Manager Register"]
    #[inline(always)]
    pub const fn ftrd(&self) -> &Ftrd {
        &self.ftrd
    }
    #[doc = "0x40 - Fault Type DMA Channel Register"]
    #[inline(always)]
    pub const fn ftr0(&self) -> &Ftr0 {
        &self.ftr0
    }
    #[doc = "0x100..0x120 - Channel Status Registers"]
    #[inline(always)]
    pub const fn csr(&self, n: usize) -> &Csr {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(256)
                .add(32 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x120 - Channel Status Registers"]
    #[inline(always)]
    pub fn csr_iter(&self) -> impl Iterator<Item = &Csr> {
        (0..8).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(256)
                .add(32 * n)
                .cast()
        })
    }
    #[doc = "0x104..0x124 - Channel Program Counter Registers"]
    #[inline(always)]
    pub const fn cpc(&self, n: usize) -> &Cpc {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(260)
                .add(32 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x104..0x124 - Channel Program Counter Registers"]
    #[inline(always)]
    pub fn cpc_iter(&self) -> impl Iterator<Item = &Cpc> {
        (0..8).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(260)
                .add(32 * n)
                .cast()
        })
    }
    #[doc = "0x400..0x420 - Source Address Registers"]
    #[inline(always)]
    pub const fn sar(&self, n: usize) -> &Sar {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(1024)
                .add(32 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x400..0x420 - Source Address Registers"]
    #[inline(always)]
    pub fn sar_iter(&self) -> impl Iterator<Item = &Sar> {
        (0..8).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(1024)
                .add(32 * n)
                .cast()
        })
    }
    #[doc = "0x404..0x424 - DestinationAddress Registers"]
    #[inline(always)]
    pub const fn dar(&self, n: usize) -> &Dar {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(1028)
                .add(32 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x404..0x424 - DestinationAddress Registers"]
    #[inline(always)]
    pub fn dar_iter(&self) -> impl Iterator<Item = &Dar> {
        (0..8).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(1028)
                .add(32 * n)
                .cast()
        })
    }
    #[doc = "0x408..0x428 - Channel Control Registers"]
    #[inline(always)]
    pub const fn ccr(&self, n: usize) -> &Ccr {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(1032)
                .add(32 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x408..0x428 - Channel Control Registers"]
    #[inline(always)]
    pub fn ccr_iter(&self) -> impl Iterator<Item = &Ccr> {
        (0..8).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(1032)
                .add(32 * n)
                .cast()
        })
    }
    #[doc = "0x40c..0x42c - Loop Counter 0 Registers"]
    #[inline(always)]
    pub const fn lc0_(&self, n: usize) -> &Lc0_ {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(1036)
                .add(32 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x40c..0x42c - Loop Counter 0 Registers"]
    #[inline(always)]
    pub fn lc0__iter(&self) -> impl Iterator<Item = &Lc0_> {
        (0..8).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(1036)
                .add(32 * n)
                .cast()
        })
    }
    #[doc = "0x410..0x430 - Loop Counter 1 Registers"]
    #[inline(always)]
    pub const fn lc1_(&self, n: usize) -> &Lc1_ {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(1040)
                .add(32 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x410..0x430 - Loop Counter 1 Registers"]
    #[inline(always)]
    pub fn lc1__iter(&self) -> impl Iterator<Item = &Lc1_> {
        (0..8).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(1040)
                .add(32 * n)
                .cast()
        })
    }
    #[doc = "0xd00 - Debug Status Register"]
    #[inline(always)]
    pub const fn dbgstatus(&self) -> &Dbgstatus {
        &self.dbgstatus
    }
    #[doc = "0xd04 - Debug Command Register"]
    #[inline(always)]
    pub const fn dbgcmd(&self) -> &Dbgcmd {
        &self.dbgcmd
    }
    #[doc = "0xd08 - Debug Instruction-0 Register"]
    #[inline(always)]
    pub const fn dbginst0(&self) -> &Dbginst0 {
        &self.dbginst0
    }
    #[doc = "0xd0c - Debug Instruction-1 Register"]
    #[inline(always)]
    pub const fn dbginst1(&self) -> &Dbginst1 {
        &self.dbginst1
    }
    #[doc = "0xe00 - Configuration Register 0"]
    #[inline(always)]
    pub const fn cr0(&self) -> &Cr0 {
        &self.cr0
    }
    #[doc = "0xe04 - Configuration Register 1"]
    #[inline(always)]
    pub const fn cr1(&self) -> &Cr1 {
        &self.cr1
    }
    #[doc = "0xe08 - Configuration Register 2"]
    #[inline(always)]
    pub const fn cr2(&self) -> &Cr2 {
        &self.cr2
    }
    #[doc = "0xe0c - Configuration Register 3"]
    #[inline(always)]
    pub const fn cr3(&self) -> &Cr3 {
        &self.cr3
    }
    #[doc = "0xe10 - Configuration Register 4"]
    #[inline(always)]
    pub const fn cr4(&self) -> &Cr4 {
        &self.cr4
    }
    #[doc = "0xe14 - DMA Configuration Register"]
    #[inline(always)]
    pub const fn crdn(&self) -> &Crdn {
        &self.crdn
    }
    #[doc = "0xe80 - DMA Watchdog Register"]
    #[inline(always)]
    pub const fn wd(&self) -> &Wd {
        &self.wd
    }
}
#[doc = "DSR (r) register accessor: DMA Manager Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsr`]
module"]
#[doc(alias = "DSR")]
pub type Dsr = crate::Reg<dsr::DsrSpec>;
#[doc = "DMA Manager Status Register"]
pub mod dsr;
#[doc = "DPC (r) register accessor: DMA Program Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpc::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpc`]
module"]
#[doc(alias = "DPC")]
pub type Dpc = crate::Reg<dpc::DpcSpec>;
#[doc = "DMA Program Counter Register"]
pub mod dpc;
#[doc = "INTEN (rw) register accessor: Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten`]
module"]
#[doc(alias = "INTEN")]
pub type Inten = crate::Reg<inten::IntenSpec>;
#[doc = "Interrupt Enable Register"]
pub mod inten;
#[doc = "EVENT_RIS (r) register accessor: Event-Interrupt Raw Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`event_ris::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@event_ris`]
module"]
#[doc(alias = "EVENT_RIS")]
pub type EventRis = crate::Reg<event_ris::EventRisSpec>;
#[doc = "Event-Interrupt Raw Status Register"]
pub mod event_ris;
#[doc = "INTMIS (r) register accessor: Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intmis::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intmis`]
module"]
#[doc(alias = "INTMIS")]
pub type Intmis = crate::Reg<intmis::IntmisSpec>;
#[doc = "Interrupt Status Register"]
pub mod intmis;
#[doc = "INTCLR (w) register accessor: Interrupt Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intclr`]
module"]
#[doc(alias = "INTCLR")]
pub type Intclr = crate::Reg<intclr::IntclrSpec>;
#[doc = "Interrupt Clear Register"]
pub mod intclr;
#[doc = "FSRD (r) register accessor: Fault Status DMA Manager Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsrd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsrd`]
module"]
#[doc(alias = "FSRD")]
pub type Fsrd = crate::Reg<fsrd::FsrdSpec>;
#[doc = "Fault Status DMA Manager Register"]
pub mod fsrd;
#[doc = "FSRC (r) register accessor: Fault Status DMA Channel Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsrc::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsrc`]
module"]
#[doc(alias = "FSRC")]
pub type Fsrc = crate::Reg<fsrc::FsrcSpec>;
#[doc = "Fault Status DMA Channel Register"]
pub mod fsrc;
#[doc = "FTRD (rw) register accessor: Fault Type DMA Manager Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ftrd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ftrd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ftrd`]
module"]
#[doc(alias = "FTRD")]
pub type Ftrd = crate::Reg<ftrd::FtrdSpec>;
#[doc = "Fault Type DMA Manager Register"]
pub mod ftrd;
#[doc = "FTR0 (r) register accessor: Fault Type DMA Channel Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ftr0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ftr0`]
module"]
#[doc(alias = "FTR0")]
pub type Ftr0 = crate::Reg<ftr0::Ftr0Spec>;
#[doc = "Fault Type DMA Channel Register"]
pub mod ftr0;
#[doc = "CSR (r) register accessor: Channel Status Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr`]
module"]
#[doc(alias = "CSR")]
pub type Csr = crate::Reg<csr::CsrSpec>;
#[doc = "Channel Status Registers"]
pub mod csr;
#[doc = "CPC (r) register accessor: Channel Program Counter Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpc::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpc`]
module"]
#[doc(alias = "CPC")]
pub type Cpc = crate::Reg<cpc::CpcSpec>;
#[doc = "Channel Program Counter Registers"]
pub mod cpc;
#[doc = "SAR (r) register accessor: Source Address Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar`]
module"]
#[doc(alias = "SAR")]
pub type Sar = crate::Reg<sar::SarSpec>;
#[doc = "Source Address Registers"]
pub mod sar;
#[doc = "DAR (r) register accessor: DestinationAddress Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dar::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dar`]
module"]
#[doc(alias = "DAR")]
pub type Dar = crate::Reg<dar::DarSpec>;
#[doc = "DestinationAddress Registers"]
pub mod dar;
#[doc = "CCR (r) register accessor: Channel Control Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr`]
module"]
#[doc(alias = "CCR")]
pub type Ccr = crate::Reg<ccr::CcrSpec>;
#[doc = "Channel Control Registers"]
pub mod ccr;
#[doc = "LC0_ (r) register accessor: Loop Counter 0 Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lc0_::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lc0_`]
module"]
#[doc(alias = "LC0_")]
pub type Lc0_ = crate::Reg<lc0_::Lc0_Spec>;
#[doc = "Loop Counter 0 Registers"]
pub mod lc0_;
#[doc = "LC1_ (r) register accessor: Loop Counter 1 Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lc1_::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lc1_`]
module"]
#[doc(alias = "LC1_")]
pub type Lc1_ = crate::Reg<lc1_::Lc1_Spec>;
#[doc = "Loop Counter 1 Registers"]
pub mod lc1_;
#[doc = "DBGSTATUS (r) register accessor: Debug Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbgstatus::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbgstatus`]
module"]
#[doc(alias = "DBGSTATUS")]
pub type Dbgstatus = crate::Reg<dbgstatus::DbgstatusSpec>;
#[doc = "Debug Status Register"]
pub mod dbgstatus;
#[doc = "DBGCMD (w) register accessor: Debug Command Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbgcmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbgcmd`]
module"]
#[doc(alias = "DBGCMD")]
pub type Dbgcmd = crate::Reg<dbgcmd::DbgcmdSpec>;
#[doc = "Debug Command Register"]
pub mod dbgcmd;
#[doc = "DBGINST0 (w) register accessor: Debug Instruction-0 Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbginst0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbginst0`]
module"]
#[doc(alias = "DBGINST0")]
pub type Dbginst0 = crate::Reg<dbginst0::Dbginst0Spec>;
#[doc = "Debug Instruction-0 Register"]
pub mod dbginst0;
#[doc = "DBGINST1 (w) register accessor: Debug Instruction-1 Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbginst1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbginst1`]
module"]
#[doc(alias = "DBGINST1")]
pub type Dbginst1 = crate::Reg<dbginst1::Dbginst1Spec>;
#[doc = "Debug Instruction-1 Register"]
pub mod dbginst1;
#[doc = "CR0 (r) register accessor: Configuration Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr0`]
module"]
#[doc(alias = "CR0")]
pub type Cr0 = crate::Reg<cr0::Cr0Spec>;
#[doc = "Configuration Register 0"]
pub mod cr0;
#[doc = "CR1 (r) register accessor: Configuration Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1`]
module"]
#[doc(alias = "CR1")]
pub type Cr1 = crate::Reg<cr1::Cr1Spec>;
#[doc = "Configuration Register 1"]
pub mod cr1;
#[doc = "CR2 (r) register accessor: Configuration Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr2`]
module"]
#[doc(alias = "CR2")]
pub type Cr2 = crate::Reg<cr2::Cr2Spec>;
#[doc = "Configuration Register 2"]
pub mod cr2;
#[doc = "CR3 (r) register accessor: Configuration Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr3`]
module"]
#[doc(alias = "CR3")]
pub type Cr3 = crate::Reg<cr3::Cr3Spec>;
#[doc = "Configuration Register 3"]
pub mod cr3;
#[doc = "CR4 (r) register accessor: Configuration Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr4`]
module"]
#[doc(alias = "CR4")]
pub type Cr4 = crate::Reg<cr4::Cr4Spec>;
#[doc = "Configuration Register 4"]
pub mod cr4;
#[doc = "CRDn (r) register accessor: DMA Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crdn::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crdn`]
module"]
#[doc(alias = "CRDn")]
pub type Crdn = crate::Reg<crdn::CrdnSpec>;
#[doc = "DMA Configuration Register"]
pub mod crdn;
#[doc = "WD (rw) register accessor: DMA Watchdog Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wd`]
module"]
#[doc(alias = "WD")]
pub type Wd = crate::Reg<wd::WdSpec>;
#[doc = "DMA Watchdog Register"]
pub mod wd;
