#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    pwren: Pwren,
    clkdiv: Clkdiv,
    clksrc: Clksrc,
    clkena: Clkena,
    tmout: Tmout,
    ctype: Ctype,
    blksiz: Blksiz,
    bytcnt: Bytcnt,
    intmask: Intmask,
    cmdarg: Cmdarg,
    cmd: Cmd,
    resp0: Resp0,
    resp1: Resp1,
    resp2: Resp2,
    resp3: Resp3,
    mintsts: Mintsts,
    rintsts: Rintsts,
    status: Status,
    fifoth: Fifoth,
    cdetect: Cdetect,
    wrtprt: Wrtprt,
    _reserved22: [u8; 0x04],
    tcbcnt: Tcbcnt,
    tbbcnt: Tbbcnt,
    debnce: Debnce,
    usrid: Usrid,
    verid: Verid,
    hcon: Hcon,
    uhs_reg: UhsReg,
    rst_n: RstN,
    _reserved30: [u8; 0x04],
    bmod: Bmod,
    pldmnd: Pldmnd,
    dbaddr: Dbaddr,
    idsts: Idsts,
    idinten: Idinten,
    dscaddr: Dscaddr,
    bufaddr: Bufaddr,
    _reserved37: [u8; 0x64],
    cardthrctl: Cardthrctl,
    back_end_power: BackEndPower,
    _reserved39: [u8; 0x04],
    emmc_ddr_reg: EmmcDdrReg,
    _reserved40: [u8; 0xf0],
    fifo_base: FifoBase,
}
impl RegisterBlock {
    #[doc = "0x00 - Control register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x04 - Power-enable register"]
    #[inline(always)]
    pub const fn pwren(&self) -> &Pwren {
        &self.pwren
    }
    #[doc = "0x08 - Clock-divider register"]
    #[inline(always)]
    pub const fn clkdiv(&self) -> &Clkdiv {
        &self.clkdiv
    }
    #[doc = "0x0c - SD clock source register"]
    #[inline(always)]
    pub const fn clksrc(&self) -> &Clksrc {
        &self.clksrc
    }
    #[doc = "0x10 - Clock-enable register"]
    #[inline(always)]
    pub const fn clkena(&self) -> &Clkena {
        &self.clkena
    }
    #[doc = "0x14 - Time-out register"]
    #[inline(always)]
    pub const fn tmout(&self) -> &Tmout {
        &self.tmout
    }
    #[doc = "0x18 - Card-type register"]
    #[inline(always)]
    pub const fn ctype(&self) -> &Ctype {
        &self.ctype
    }
    #[doc = "0x1c - Block-size register"]
    #[inline(always)]
    pub const fn blksiz(&self) -> &Blksiz {
        &self.blksiz
    }
    #[doc = "0x20 - Byte-count register"]
    #[inline(always)]
    pub const fn bytcnt(&self) -> &Bytcnt {
        &self.bytcnt
    }
    #[doc = "0x24 - Interrupt-mask register"]
    #[inline(always)]
    pub const fn intmask(&self) -> &Intmask {
        &self.intmask
    }
    #[doc = "0x28 - Command-argument register"]
    #[inline(always)]
    pub const fn cmdarg(&self) -> &Cmdarg {
        &self.cmdarg
    }
    #[doc = "0x2c - Command register"]
    #[inline(always)]
    pub const fn cmd(&self) -> &Cmd {
        &self.cmd
    }
    #[doc = "0x30 - Response-0 register"]
    #[inline(always)]
    pub const fn resp0(&self) -> &Resp0 {
        &self.resp0
    }
    #[doc = "0x34 - Response-1 register"]
    #[inline(always)]
    pub const fn resp1(&self) -> &Resp1 {
        &self.resp1
    }
    #[doc = "0x38 - Response-2 register"]
    #[inline(always)]
    pub const fn resp2(&self) -> &Resp2 {
        &self.resp2
    }
    #[doc = "0x3c - Response-3 register"]
    #[inline(always)]
    pub const fn resp3(&self) -> &Resp3 {
        &self.resp3
    }
    #[doc = "0x40 - Masked interrupt-status register"]
    #[inline(always)]
    pub const fn mintsts(&self) -> &Mintsts {
        &self.mintsts
    }
    #[doc = "0x44 - Raw interrupt-status register"]
    #[inline(always)]
    pub const fn rintsts(&self) -> &Rintsts {
        &self.rintsts
    }
    #[doc = "0x48 - Status register"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x4c - FIFO threshold register"]
    #[inline(always)]
    pub const fn fifoth(&self) -> &Fifoth {
        &self.fifoth
    }
    #[doc = "0x50 - Card-detect register"]
    #[inline(always)]
    pub const fn cdetect(&self) -> &Cdetect {
        &self.cdetect
    }
    #[doc = "0x54 - Write-protect register"]
    #[inline(always)]
    pub const fn wrtprt(&self) -> &Wrtprt {
        &self.wrtprt
    }
    #[doc = "0x5c - Transferred CIU card byte count"]
    #[inline(always)]
    pub const fn tcbcnt(&self) -> &Tcbcnt {
        &self.tcbcnt
    }
    #[doc = "0x60 - Transferred host/DMA to/from BIU-FIFO byte count"]
    #[inline(always)]
    pub const fn tbbcnt(&self) -> &Tbbcnt {
        &self.tbbcnt
    }
    #[doc = "0x64 - Card detect debounce register"]
    #[inline(always)]
    pub const fn debnce(&self) -> &Debnce {
        &self.debnce
    }
    #[doc = "0x68 - User ID register"]
    #[inline(always)]
    pub const fn usrid(&self) -> &Usrid {
        &self.usrid
    }
    #[doc = "0x6c - Version ID register"]
    #[inline(always)]
    pub const fn verid(&self) -> &Verid {
        &self.verid
    }
    #[doc = "0x70 - Hardware configuration register"]
    #[inline(always)]
    pub const fn hcon(&self) -> &Hcon {
        &self.hcon
    }
    #[doc = "0x74 - UHS-1 register"]
    #[inline(always)]
    pub const fn uhs_reg(&self) -> &UhsReg {
        &self.uhs_reg
    }
    #[doc = "0x78 - Hardware reset register"]
    #[inline(always)]
    pub const fn rst_n(&self) -> &RstN {
        &self.rst_n
    }
    #[doc = "0x80 - Bus mode register"]
    #[inline(always)]
    pub const fn bmod(&self) -> &Bmod {
        &self.bmod
    }
    #[doc = "0x84 - Poll demand register"]
    #[inline(always)]
    pub const fn pldmnd(&self) -> &Pldmnd {
        &self.pldmnd
    }
    #[doc = "0x88 - Descriptor list base address register"]
    #[inline(always)]
    pub const fn dbaddr(&self) -> &Dbaddr {
        &self.dbaddr
    }
    #[doc = "0x8c - Internal DMAC status register"]
    #[inline(always)]
    pub const fn idsts(&self) -> &Idsts {
        &self.idsts
    }
    #[doc = "0x90 - Internal DMAC interrupt enable register"]
    #[inline(always)]
    pub const fn idinten(&self) -> &Idinten {
        &self.idinten
    }
    #[doc = "0x94 - Current host descriptor address register"]
    #[inline(always)]
    pub const fn dscaddr(&self) -> &Dscaddr {
        &self.dscaddr
    }
    #[doc = "0x98 - Current buffer descriptor address register"]
    #[inline(always)]
    pub const fn bufaddr(&self) -> &Bufaddr {
        &self.bufaddr
    }
    #[doc = "0x100 - Card read threshold enable register"]
    #[inline(always)]
    pub const fn cardthrctl(&self) -> &Cardthrctl {
        &self.cardthrctl
    }
    #[doc = "0x104 - Back-end power register"]
    #[inline(always)]
    pub const fn back_end_power(&self) -> &BackEndPower {
        &self.back_end_power
    }
    #[doc = "0x10c - eMMC4.5 DDR start bit detection control register"]
    #[inline(always)]
    pub const fn emmc_ddr_reg(&self) -> &EmmcDdrReg {
        &self.emmc_ddr_reg
    }
    #[doc = "0x200 - FIFO base address register"]
    #[inline(always)]
    pub const fn fifo_base(&self) -> &FifoBase {
        &self.fifo_base
    }
}
#[doc = "CTRL (rw) register accessor: Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Control register"]
pub mod ctrl;
#[doc = "PWREN (rw) register accessor: Power-enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwren::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwren::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwren`]
module"]
#[doc(alias = "PWREN")]
pub type Pwren = crate::Reg<pwren::PwrenSpec>;
#[doc = "Power-enable register"]
pub mod pwren;
#[doc = "CLKDIV (rw) register accessor: Clock-divider register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkdiv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkdiv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkdiv`]
module"]
#[doc(alias = "CLKDIV")]
pub type Clkdiv = crate::Reg<clkdiv::ClkdivSpec>;
#[doc = "Clock-divider register"]
pub mod clkdiv;
#[doc = "CLKSRC (rw) register accessor: SD clock source register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksrc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksrc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksrc`]
module"]
#[doc(alias = "CLKSRC")]
pub type Clksrc = crate::Reg<clksrc::ClksrcSpec>;
#[doc = "SD clock source register"]
pub mod clksrc;
#[doc = "CLKENA (rw) register accessor: Clock-enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkena`]
module"]
#[doc(alias = "CLKENA")]
pub type Clkena = crate::Reg<clkena::ClkenaSpec>;
#[doc = "Clock-enable register"]
pub mod clkena;
#[doc = "TMOUT (rw) register accessor: Time-out register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmout::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmout::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmout`]
module"]
#[doc(alias = "TMOUT")]
pub type Tmout = crate::Reg<tmout::TmoutSpec>;
#[doc = "Time-out register"]
pub mod tmout;
#[doc = "CTYPE (rw) register accessor: Card-type register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctype::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctype::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctype`]
module"]
#[doc(alias = "CTYPE")]
pub type Ctype = crate::Reg<ctype::CtypeSpec>;
#[doc = "Card-type register"]
pub mod ctype;
#[doc = "BLKSIZ (rw) register accessor: Block-size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blksiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blksiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blksiz`]
module"]
#[doc(alias = "BLKSIZ")]
pub type Blksiz = crate::Reg<blksiz::BlksizSpec>;
#[doc = "Block-size register"]
pub mod blksiz;
#[doc = "BYTCNT (rw) register accessor: Byte-count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bytcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bytcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bytcnt`]
module"]
#[doc(alias = "BYTCNT")]
pub type Bytcnt = crate::Reg<bytcnt::BytcntSpec>;
#[doc = "Byte-count register"]
pub mod bytcnt;
#[doc = "INTMASK (rw) register accessor: Interrupt-mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intmask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intmask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intmask`]
module"]
#[doc(alias = "INTMASK")]
pub type Intmask = crate::Reg<intmask::IntmaskSpec>;
#[doc = "Interrupt-mask register"]
pub mod intmask;
#[doc = "CMDARG (rw) register accessor: Command-argument register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdarg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdarg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdarg`]
module"]
#[doc(alias = "CMDARG")]
pub type Cmdarg = crate::Reg<cmdarg::CmdargSpec>;
#[doc = "Command-argument register"]
pub mod cmdarg;
#[doc = "CMD (rw) register accessor: Command register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`]
module"]
#[doc(alias = "CMD")]
pub type Cmd = crate::Reg<cmd::CmdSpec>;
#[doc = "Command register"]
pub mod cmd;
#[doc = "RESP0 (r) register accessor: Response-0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resp0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resp0`]
module"]
#[doc(alias = "RESP0")]
pub type Resp0 = crate::Reg<resp0::Resp0Spec>;
#[doc = "Response-0 register"]
pub mod resp0;
#[doc = "RESP1 (r) register accessor: Response-1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resp1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resp1`]
module"]
#[doc(alias = "RESP1")]
pub type Resp1 = crate::Reg<resp1::Resp1Spec>;
#[doc = "Response-1 register"]
pub mod resp1;
#[doc = "RESP2 (r) register accessor: Response-2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resp2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resp2`]
module"]
#[doc(alias = "RESP2")]
pub type Resp2 = crate::Reg<resp2::Resp2Spec>;
#[doc = "Response-2 register"]
pub mod resp2;
#[doc = "RESP3 (r) register accessor: Response-3 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resp3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resp3`]
module"]
#[doc(alias = "RESP3")]
pub type Resp3 = crate::Reg<resp3::Resp3Spec>;
#[doc = "Response-3 register"]
pub mod resp3;
#[doc = "MINTSTS (rw) register accessor: Masked interrupt-status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mintsts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mintsts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mintsts`]
module"]
#[doc(alias = "MINTSTS")]
pub type Mintsts = crate::Reg<mintsts::MintstsSpec>;
#[doc = "Masked interrupt-status register"]
pub mod mintsts;
#[doc = "RINTSTS (rw) register accessor: Raw interrupt-status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rintsts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rintsts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rintsts`]
module"]
#[doc(alias = "RINTSTS")]
pub type Rintsts = crate::Reg<rintsts::RintstsSpec>;
#[doc = "Raw interrupt-status register"]
pub mod rintsts;
#[doc = "STATUS (r) register accessor: Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Status register"]
pub mod status;
#[doc = "FIFOTH (rw) register accessor: FIFO threshold register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifoth::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifoth::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifoth`]
module"]
#[doc(alias = "FIFOTH")]
pub type Fifoth = crate::Reg<fifoth::FifothSpec>;
#[doc = "FIFO threshold register"]
pub mod fifoth;
#[doc = "CDETECT (r) register accessor: Card-detect register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdetect::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cdetect`]
module"]
#[doc(alias = "CDETECT")]
pub type Cdetect = crate::Reg<cdetect::CdetectSpec>;
#[doc = "Card-detect register"]
pub mod cdetect;
#[doc = "WRTPRT (rw) register accessor: Write-protect register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wrtprt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wrtprt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wrtprt`]
module"]
#[doc(alias = "WRTPRT")]
pub type Wrtprt = crate::Reg<wrtprt::WrtprtSpec>;
#[doc = "Write-protect register"]
pub mod wrtprt;
#[doc = "TCBCNT (r) register accessor: Transferred CIU card byte count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tcbcnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcbcnt`]
module"]
#[doc(alias = "TCBCNT")]
pub type Tcbcnt = crate::Reg<tcbcnt::TcbcntSpec>;
#[doc = "Transferred CIU card byte count"]
pub mod tcbcnt;
#[doc = "TBBCNT (r) register accessor: Transferred host/DMA to/from BIU-FIFO byte count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tbbcnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbbcnt`]
module"]
#[doc(alias = "TBBCNT")]
pub type Tbbcnt = crate::Reg<tbbcnt::TbbcntSpec>;
#[doc = "Transferred host/DMA to/from BIU-FIFO byte count"]
pub mod tbbcnt;
#[doc = "DEBNCE (rw) register accessor: Card detect debounce register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`debnce::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`debnce::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debnce`]
module"]
#[doc(alias = "DEBNCE")]
pub type Debnce = crate::Reg<debnce::DebnceSpec>;
#[doc = "Card detect debounce register"]
pub mod debnce;
#[doc = "USRID (rw) register accessor: User ID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usrid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usrid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usrid`]
module"]
#[doc(alias = "USRID")]
pub type Usrid = crate::Reg<usrid::UsridSpec>;
#[doc = "User ID register"]
pub mod usrid;
#[doc = "VERID (r) register accessor: Version ID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`verid::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@verid`]
module"]
#[doc(alias = "VERID")]
pub type Verid = crate::Reg<verid::VeridSpec>;
#[doc = "Version ID register"]
pub mod verid;
#[doc = "HCON (r) register accessor: Hardware configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcon::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcon`]
module"]
#[doc(alias = "HCON")]
pub type Hcon = crate::Reg<hcon::HconSpec>;
#[doc = "Hardware configuration register"]
pub mod hcon;
#[doc = "UHS_REG (rw) register accessor: UHS-1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uhs_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uhs_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uhs_reg`]
module"]
#[doc(alias = "UHS_REG")]
pub type UhsReg = crate::Reg<uhs_reg::UhsRegSpec>;
#[doc = "UHS-1 register"]
pub mod uhs_reg;
#[doc = "RST_n (rw) register accessor: Hardware reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rst_n::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rst_n::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rst_n`]
module"]
#[doc(alias = "RST_n")]
pub type RstN = crate::Reg<rst_n::RstNSpec>;
#[doc = "Hardware reset register"]
pub mod rst_n;
#[doc = "BMOD (rw) register accessor: Bus mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bmod::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bmod::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bmod`]
module"]
#[doc(alias = "BMOD")]
pub type Bmod = crate::Reg<bmod::BmodSpec>;
#[doc = "Bus mode register"]
pub mod bmod;
#[doc = "PLDMND (w) register accessor: Poll demand register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pldmnd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pldmnd`]
module"]
#[doc(alias = "PLDMND")]
pub type Pldmnd = crate::Reg<pldmnd::PldmndSpec>;
#[doc = "Poll demand register"]
pub mod pldmnd;
#[doc = "DBADDR (rw) register accessor: Descriptor list base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbaddr`]
module"]
#[doc(alias = "DBADDR")]
pub type Dbaddr = crate::Reg<dbaddr::DbaddrSpec>;
#[doc = "Descriptor list base address register"]
pub mod dbaddr;
#[doc = "IDSTS (rw) register accessor: Internal DMAC status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idsts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idsts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idsts`]
module"]
#[doc(alias = "IDSTS")]
pub type Idsts = crate::Reg<idsts::IdstsSpec>;
#[doc = "Internal DMAC status register"]
pub mod idsts;
#[doc = "IDINTEN (rw) register accessor: Internal DMAC interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idinten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idinten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idinten`]
module"]
#[doc(alias = "IDINTEN")]
pub type Idinten = crate::Reg<idinten::IdintenSpec>;
#[doc = "Internal DMAC interrupt enable register"]
pub mod idinten;
#[doc = "DSCADDR (rw) register accessor: Current host descriptor address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dscaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dscaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dscaddr`]
module"]
#[doc(alias = "DSCADDR")]
pub type Dscaddr = crate::Reg<dscaddr::DscaddrSpec>;
#[doc = "Current host descriptor address register"]
pub mod dscaddr;
#[doc = "BUFADDR (rw) register accessor: Current buffer descriptor address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bufaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bufaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bufaddr`]
module"]
#[doc(alias = "BUFADDR")]
pub type Bufaddr = crate::Reg<bufaddr::BufaddrSpec>;
#[doc = "Current buffer descriptor address register"]
pub mod bufaddr;
#[doc = "CARDTHRCTL (rw) register accessor: Card read threshold enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cardthrctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cardthrctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cardthrctl`]
module"]
#[doc(alias = "CARDTHRCTL")]
pub type Cardthrctl = crate::Reg<cardthrctl::CardthrctlSpec>;
#[doc = "Card read threshold enable register"]
pub mod cardthrctl;
#[doc = "BACK_END_POWER (rw) register accessor: Back-end power register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`back_end_power::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`back_end_power::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@back_end_power`]
module"]
#[doc(alias = "BACK_END_POWER")]
pub type BackEndPower = crate::Reg<back_end_power::BackEndPowerSpec>;
#[doc = "Back-end power register"]
pub mod back_end_power;
#[doc = "EMMC_DDR_REG (rw) register accessor: eMMC4.5 DDR start bit detection control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmc_ddr_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmc_ddr_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmc_ddr_reg`]
module"]
#[doc(alias = "EMMC_DDR_REG")]
pub type EmmcDdrReg = crate::Reg<emmc_ddr_reg::EmmcDdrRegSpec>;
#[doc = "eMMC4.5 DDR start bit detection control register"]
pub mod emmc_ddr_reg;
#[doc = "FIFO_BASE (rw) register accessor: FIFO base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo_base::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo_base::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo_base`]
module"]
#[doc(alias = "FIFO_BASE")]
pub type FifoBase = crate::Reg<fifo_base::FifoBaseSpec>;
#[doc = "FIFO base address register"]
pub mod fifo_base;
