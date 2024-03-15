#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sdmmc_ctrl: SdmmcCtrl,
    sdmmc_pwren: SdmmcPwren,
    sdmmc_clkdiv: SdmmcClkdiv,
    sdmmc_clksrc: SdmmcClksrc,
    sdmmc_clkena: SdmmcClkena,
    sdmmc_tmout: SdmmcTmout,
    sdmmc_ctype: SdmmcCtype,
    sdmmc_blksiz: SdmmcBlksiz,
    sdmmc_bytcnt: SdmmcBytcnt,
    sdmmc_intmask: SdmmcIntmask,
    sdmmc_cmdarg: SdmmcCmdarg,
    sdmmc_cmd: SdmmcCmd,
    sdmmc_resp0: SdmmcResp0,
    sdmmc_resp1: SdmmcResp1,
    sdmmc_resp2: SdmmcResp2,
    sdmmc_resp3: SdmmcResp3,
    sdmmc_mintsts: SdmmcMintsts,
    sdmmc_rintsts: SdmmcRintsts,
    sdmmc_status: SdmmcStatus,
    sdmmc_fifoth: SdmmcFifoth,
    sdmmc_cdetect: SdmmcCdetect,
    sdmmc_wrtprt: SdmmcWrtprt,
    _reserved22: [u8; 0x04],
    sdmmc_tcbcnt: SdmmcTcbcnt,
    sdmmc_tbbcnt: SdmmcTbbcnt,
    sdmmc_debnce: SdmmcDebnce,
    sdmmc_usrid: SdmmcUsrid,
    sdmmc_verid: SdmmcVerid,
    sdmmc_hcon: SdmmcHcon,
    sdmmc_uhs_reg: SdmmcUhsReg,
    sdmmc_rst_n: SdmmcRstN,
    _reserved30: [u8; 0x04],
    sdmmc_bmod: SdmmcBmod,
    sdmmc_pldmnd: SdmmcPldmnd,
    sdmmc_dbaddr: SdmmcDbaddr,
    sdmmc_idsts: SdmmcIdsts,
    sdmmc_idinten: SdmmcIdinten,
    sdmmc_dscaddr: SdmmcDscaddr,
    sdmmc_bufaddr: SdmmcBufaddr,
    _reserved37: [u8; 0x64],
    sdmmc_cardthrctl: SdmmcCardthrctl,
    sdmmc_back_end_power: SdmmcBackEndPower,
    _reserved39: [u8; 0x04],
    sdmmc_emmc_ddr_reg: SdmmcEmmcDdrReg,
    _reserved40: [u8; 0xf0],
    sdmmc_fifo_base: SdmmcFifoBase,
}
impl RegisterBlock {
    #[doc = "0x00 - Control register"]
    #[inline(always)]
    pub const fn sdmmc_ctrl(&self) -> &SdmmcCtrl {
        &self.sdmmc_ctrl
    }
    #[doc = "0x04 - Power-enable register"]
    #[inline(always)]
    pub const fn sdmmc_pwren(&self) -> &SdmmcPwren {
        &self.sdmmc_pwren
    }
    #[doc = "0x08 - Clock-divider register"]
    #[inline(always)]
    pub const fn sdmmc_clkdiv(&self) -> &SdmmcClkdiv {
        &self.sdmmc_clkdiv
    }
    #[doc = "0x0c - SD clock source register"]
    #[inline(always)]
    pub const fn sdmmc_clksrc(&self) -> &SdmmcClksrc {
        &self.sdmmc_clksrc
    }
    #[doc = "0x10 - Clock-enable register"]
    #[inline(always)]
    pub const fn sdmmc_clkena(&self) -> &SdmmcClkena {
        &self.sdmmc_clkena
    }
    #[doc = "0x14 - Time-out register"]
    #[inline(always)]
    pub const fn sdmmc_tmout(&self) -> &SdmmcTmout {
        &self.sdmmc_tmout
    }
    #[doc = "0x18 - Card-type register"]
    #[inline(always)]
    pub const fn sdmmc_ctype(&self) -> &SdmmcCtype {
        &self.sdmmc_ctype
    }
    #[doc = "0x1c - Block-size register"]
    #[inline(always)]
    pub const fn sdmmc_blksiz(&self) -> &SdmmcBlksiz {
        &self.sdmmc_blksiz
    }
    #[doc = "0x20 - Byte-count register"]
    #[inline(always)]
    pub const fn sdmmc_bytcnt(&self) -> &SdmmcBytcnt {
        &self.sdmmc_bytcnt
    }
    #[doc = "0x24 - Interrupt-mask register"]
    #[inline(always)]
    pub const fn sdmmc_intmask(&self) -> &SdmmcIntmask {
        &self.sdmmc_intmask
    }
    #[doc = "0x28 - Command-argument register"]
    #[inline(always)]
    pub const fn sdmmc_cmdarg(&self) -> &SdmmcCmdarg {
        &self.sdmmc_cmdarg
    }
    #[doc = "0x2c - Command register"]
    #[inline(always)]
    pub const fn sdmmc_cmd(&self) -> &SdmmcCmd {
        &self.sdmmc_cmd
    }
    #[doc = "0x30 - Response-0 register"]
    #[inline(always)]
    pub const fn sdmmc_resp0(&self) -> &SdmmcResp0 {
        &self.sdmmc_resp0
    }
    #[doc = "0x34 - Response-1 register"]
    #[inline(always)]
    pub const fn sdmmc_resp1(&self) -> &SdmmcResp1 {
        &self.sdmmc_resp1
    }
    #[doc = "0x38 - Response-2 register"]
    #[inline(always)]
    pub const fn sdmmc_resp2(&self) -> &SdmmcResp2 {
        &self.sdmmc_resp2
    }
    #[doc = "0x3c - Response-3 register"]
    #[inline(always)]
    pub const fn sdmmc_resp3(&self) -> &SdmmcResp3 {
        &self.sdmmc_resp3
    }
    #[doc = "0x40 - Masked interrupt-status register"]
    #[inline(always)]
    pub const fn sdmmc_mintsts(&self) -> &SdmmcMintsts {
        &self.sdmmc_mintsts
    }
    #[doc = "0x44 - Raw interrupt-status register"]
    #[inline(always)]
    pub const fn sdmmc_rintsts(&self) -> &SdmmcRintsts {
        &self.sdmmc_rintsts
    }
    #[doc = "0x48 - Status register"]
    #[inline(always)]
    pub const fn sdmmc_status(&self) -> &SdmmcStatus {
        &self.sdmmc_status
    }
    #[doc = "0x4c - FIFO threshold register"]
    #[inline(always)]
    pub const fn sdmmc_fifoth(&self) -> &SdmmcFifoth {
        &self.sdmmc_fifoth
    }
    #[doc = "0x50 - Card-detect register"]
    #[inline(always)]
    pub const fn sdmmc_cdetect(&self) -> &SdmmcCdetect {
        &self.sdmmc_cdetect
    }
    #[doc = "0x54 - Write-protect register"]
    #[inline(always)]
    pub const fn sdmmc_wrtprt(&self) -> &SdmmcWrtprt {
        &self.sdmmc_wrtprt
    }
    #[doc = "0x5c - Transferred CIU card byte count"]
    #[inline(always)]
    pub const fn sdmmc_tcbcnt(&self) -> &SdmmcTcbcnt {
        &self.sdmmc_tcbcnt
    }
    #[doc = "0x60 - Transferred host/DMA to/from BIU-FIFO byte count"]
    #[inline(always)]
    pub const fn sdmmc_tbbcnt(&self) -> &SdmmcTbbcnt {
        &self.sdmmc_tbbcnt
    }
    #[doc = "0x64 - Card detect debounce register"]
    #[inline(always)]
    pub const fn sdmmc_debnce(&self) -> &SdmmcDebnce {
        &self.sdmmc_debnce
    }
    #[doc = "0x68 - User ID register"]
    #[inline(always)]
    pub const fn sdmmc_usrid(&self) -> &SdmmcUsrid {
        &self.sdmmc_usrid
    }
    #[doc = "0x6c - Version ID register"]
    #[inline(always)]
    pub const fn sdmmc_verid(&self) -> &SdmmcVerid {
        &self.sdmmc_verid
    }
    #[doc = "0x70 - Hardware configuration register"]
    #[inline(always)]
    pub const fn sdmmc_hcon(&self) -> &SdmmcHcon {
        &self.sdmmc_hcon
    }
    #[doc = "0x74 - UHS-1 register"]
    #[inline(always)]
    pub const fn sdmmc_uhs_reg(&self) -> &SdmmcUhsReg {
        &self.sdmmc_uhs_reg
    }
    #[doc = "0x78 - Hardware reset register"]
    #[inline(always)]
    pub const fn sdmmc_rst_n(&self) -> &SdmmcRstN {
        &self.sdmmc_rst_n
    }
    #[doc = "0x80 - Bus mode register"]
    #[inline(always)]
    pub const fn sdmmc_bmod(&self) -> &SdmmcBmod {
        &self.sdmmc_bmod
    }
    #[doc = "0x84 - Poll demand register"]
    #[inline(always)]
    pub const fn sdmmc_pldmnd(&self) -> &SdmmcPldmnd {
        &self.sdmmc_pldmnd
    }
    #[doc = "0x88 - Descriptor list base address register"]
    #[inline(always)]
    pub const fn sdmmc_dbaddr(&self) -> &SdmmcDbaddr {
        &self.sdmmc_dbaddr
    }
    #[doc = "0x8c - Internal DMAC status register"]
    #[inline(always)]
    pub const fn sdmmc_idsts(&self) -> &SdmmcIdsts {
        &self.sdmmc_idsts
    }
    #[doc = "0x90 - Internal DMAC interrupt enable register"]
    #[inline(always)]
    pub const fn sdmmc_idinten(&self) -> &SdmmcIdinten {
        &self.sdmmc_idinten
    }
    #[doc = "0x94 - Current host descriptor address register"]
    #[inline(always)]
    pub const fn sdmmc_dscaddr(&self) -> &SdmmcDscaddr {
        &self.sdmmc_dscaddr
    }
    #[doc = "0x98 - Current buffer descriptor address register"]
    #[inline(always)]
    pub const fn sdmmc_bufaddr(&self) -> &SdmmcBufaddr {
        &self.sdmmc_bufaddr
    }
    #[doc = "0x100 - Card read threshold enable register"]
    #[inline(always)]
    pub const fn sdmmc_cardthrctl(&self) -> &SdmmcCardthrctl {
        &self.sdmmc_cardthrctl
    }
    #[doc = "0x104 - Back-end power register"]
    #[inline(always)]
    pub const fn sdmmc_back_end_power(&self) -> &SdmmcBackEndPower {
        &self.sdmmc_back_end_power
    }
    #[doc = "0x10c - eMMC4.5 DDR start bit detection control register"]
    #[inline(always)]
    pub const fn sdmmc_emmc_ddr_reg(&self) -> &SdmmcEmmcDdrReg {
        &self.sdmmc_emmc_ddr_reg
    }
    #[doc = "0x200 - FIFO base address register"]
    #[inline(always)]
    pub const fn sdmmc_fifo_base(&self) -> &SdmmcFifoBase {
        &self.sdmmc_fifo_base
    }
}
#[doc = "SDMMC_CTRL (rw) register accessor: Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmmc_ctrl`]
module"]
#[doc(alias = "SDMMC_CTRL")]
pub type SdmmcCtrl = crate::Reg<sdmmc_ctrl::SdmmcCtrlSpec>;
#[doc = "Control register"]
pub mod sdmmc_ctrl;
#[doc = "SDMMC_PWREN (rw) register accessor: Power-enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_pwren::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_pwren::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmmc_pwren`]
module"]
#[doc(alias = "SDMMC_PWREN")]
pub type SdmmcPwren = crate::Reg<sdmmc_pwren::SdmmcPwrenSpec>;
#[doc = "Power-enable register"]
pub mod sdmmc_pwren;
#[doc = "SDMMC_CLKDIV (rw) register accessor: Clock-divider register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_clkdiv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_clkdiv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmmc_clkdiv`]
module"]
#[doc(alias = "SDMMC_CLKDIV")]
pub type SdmmcClkdiv = crate::Reg<sdmmc_clkdiv::SdmmcClkdivSpec>;
#[doc = "Clock-divider register"]
pub mod sdmmc_clkdiv;
#[doc = "SDMMC_CLKSRC (rw) register accessor: SD clock source register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_clksrc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_clksrc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmmc_clksrc`]
module"]
#[doc(alias = "SDMMC_CLKSRC")]
pub type SdmmcClksrc = crate::Reg<sdmmc_clksrc::SdmmcClksrcSpec>;
#[doc = "SD clock source register"]
pub mod sdmmc_clksrc;
#[doc = "SDMMC_CLKENA (rw) register accessor: Clock-enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_clkena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_clkena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmmc_clkena`]
module"]
#[doc(alias = "SDMMC_CLKENA")]
pub type SdmmcClkena = crate::Reg<sdmmc_clkena::SdmmcClkenaSpec>;
#[doc = "Clock-enable register"]
pub mod sdmmc_clkena;
#[doc = "SDMMC_TMOUT (rw) register accessor: Time-out register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_tmout::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_tmout::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmmc_tmout`]
module"]
#[doc(alias = "SDMMC_TMOUT")]
pub type SdmmcTmout = crate::Reg<sdmmc_tmout::SdmmcTmoutSpec>;
#[doc = "Time-out register"]
pub mod sdmmc_tmout;
#[doc = "SDMMC_CTYPE (rw) register accessor: Card-type register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_ctype::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_ctype::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmmc_ctype`]
module"]
#[doc(alias = "SDMMC_CTYPE")]
pub type SdmmcCtype = crate::Reg<sdmmc_ctype::SdmmcCtypeSpec>;
#[doc = "Card-type register"]
pub mod sdmmc_ctype;
#[doc = "SDMMC_BLKSIZ (rw) register accessor: Block-size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_blksiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_blksiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmmc_blksiz`]
module"]
#[doc(alias = "SDMMC_BLKSIZ")]
pub type SdmmcBlksiz = crate::Reg<sdmmc_blksiz::SdmmcBlksizSpec>;
#[doc = "Block-size register"]
pub mod sdmmc_blksiz;
#[doc = "SDMMC_BYTCNT (rw) register accessor: Byte-count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_bytcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_bytcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmmc_bytcnt`]
module"]
#[doc(alias = "SDMMC_BYTCNT")]
pub type SdmmcBytcnt = crate::Reg<sdmmc_bytcnt::SdmmcBytcntSpec>;
#[doc = "Byte-count register"]
pub mod sdmmc_bytcnt;
#[doc = "SDMMC_INTMASK (rw) register accessor: Interrupt-mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_intmask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_intmask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmmc_intmask`]
module"]
#[doc(alias = "SDMMC_INTMASK")]
pub type SdmmcIntmask = crate::Reg<sdmmc_intmask::SdmmcIntmaskSpec>;
#[doc = "Interrupt-mask register"]
pub mod sdmmc_intmask;
#[doc = "SDMMC_CMDARG (rw) register accessor: Command-argument register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_cmdarg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_cmdarg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmmc_cmdarg`]
module"]
#[doc(alias = "SDMMC_CMDARG")]
pub type SdmmcCmdarg = crate::Reg<sdmmc_cmdarg::SdmmcCmdargSpec>;
#[doc = "Command-argument register"]
pub mod sdmmc_cmdarg;
#[doc = "SDMMC_CMD (rw) register accessor: Command register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmmc_cmd`]
module"]
#[doc(alias = "SDMMC_CMD")]
pub type SdmmcCmd = crate::Reg<sdmmc_cmd::SdmmcCmdSpec>;
#[doc = "Command register"]
pub mod sdmmc_cmd;
#[doc = "SDMMC_RESP0 (r) register accessor: Response-0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_resp0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmmc_resp0`]
module"]
#[doc(alias = "SDMMC_RESP0")]
pub type SdmmcResp0 = crate::Reg<sdmmc_resp0::SdmmcResp0Spec>;
#[doc = "Response-0 register"]
pub mod sdmmc_resp0;
#[doc = "SDMMC_RESP1 (r) register accessor: Response-1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_resp1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmmc_resp1`]
module"]
#[doc(alias = "SDMMC_RESP1")]
pub type SdmmcResp1 = crate::Reg<sdmmc_resp1::SdmmcResp1Spec>;
#[doc = "Response-1 register"]
pub mod sdmmc_resp1;
#[doc = "SDMMC_RESP2 (r) register accessor: Response-2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_resp2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmmc_resp2`]
module"]
#[doc(alias = "SDMMC_RESP2")]
pub type SdmmcResp2 = crate::Reg<sdmmc_resp2::SdmmcResp2Spec>;
#[doc = "Response-2 register"]
pub mod sdmmc_resp2;
#[doc = "SDMMC_RESP3 (r) register accessor: Response-3 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_resp3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmmc_resp3`]
module"]
#[doc(alias = "SDMMC_RESP3")]
pub type SdmmcResp3 = crate::Reg<sdmmc_resp3::SdmmcResp3Spec>;
#[doc = "Response-3 register"]
pub mod sdmmc_resp3;
#[doc = "SDMMC_MINTSTS (rw) register accessor: Masked interrupt-status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_mintsts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_mintsts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmmc_mintsts`]
module"]
#[doc(alias = "SDMMC_MINTSTS")]
pub type SdmmcMintsts = crate::Reg<sdmmc_mintsts::SdmmcMintstsSpec>;
#[doc = "Masked interrupt-status register"]
pub mod sdmmc_mintsts;
#[doc = "SDMMC_RINTSTS (rw) register accessor: Raw interrupt-status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_rintsts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_rintsts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmmc_rintsts`]
module"]
#[doc(alias = "SDMMC_RINTSTS")]
pub type SdmmcRintsts = crate::Reg<sdmmc_rintsts::SdmmcRintstsSpec>;
#[doc = "Raw interrupt-status register"]
pub mod sdmmc_rintsts;
#[doc = "SDMMC_STATUS (r) register accessor: Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmmc_status`]
module"]
#[doc(alias = "SDMMC_STATUS")]
pub type SdmmcStatus = crate::Reg<sdmmc_status::SdmmcStatusSpec>;
#[doc = "Status register"]
pub mod sdmmc_status;
#[doc = "SDMMC_FIFOTH (rw) register accessor: FIFO threshold register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_fifoth::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_fifoth::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmmc_fifoth`]
module"]
#[doc(alias = "SDMMC_FIFOTH")]
pub type SdmmcFifoth = crate::Reg<sdmmc_fifoth::SdmmcFifothSpec>;
#[doc = "FIFO threshold register"]
pub mod sdmmc_fifoth;
#[doc = "SDMMC_CDETECT (r) register accessor: Card-detect register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_cdetect::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmmc_cdetect`]
module"]
#[doc(alias = "SDMMC_CDETECT")]
pub type SdmmcCdetect = crate::Reg<sdmmc_cdetect::SdmmcCdetectSpec>;
#[doc = "Card-detect register"]
pub mod sdmmc_cdetect;
#[doc = "SDMMC_WRTPRT (rw) register accessor: Write-protect register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_wrtprt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_wrtprt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmmc_wrtprt`]
module"]
#[doc(alias = "SDMMC_WRTPRT")]
pub type SdmmcWrtprt = crate::Reg<sdmmc_wrtprt::SdmmcWrtprtSpec>;
#[doc = "Write-protect register"]
pub mod sdmmc_wrtprt;
#[doc = "SDMMC_TCBCNT (r) register accessor: Transferred CIU card byte count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_tcbcnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmmc_tcbcnt`]
module"]
#[doc(alias = "SDMMC_TCBCNT")]
pub type SdmmcTcbcnt = crate::Reg<sdmmc_tcbcnt::SdmmcTcbcntSpec>;
#[doc = "Transferred CIU card byte count"]
pub mod sdmmc_tcbcnt;
#[doc = "SDMMC_TBBCNT (r) register accessor: Transferred host/DMA to/from BIU-FIFO byte count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_tbbcnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmmc_tbbcnt`]
module"]
#[doc(alias = "SDMMC_TBBCNT")]
pub type SdmmcTbbcnt = crate::Reg<sdmmc_tbbcnt::SdmmcTbbcntSpec>;
#[doc = "Transferred host/DMA to/from BIU-FIFO byte count"]
pub mod sdmmc_tbbcnt;
#[doc = "SDMMC_DEBNCE (rw) register accessor: Card detect debounce register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_debnce::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_debnce::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmmc_debnce`]
module"]
#[doc(alias = "SDMMC_DEBNCE")]
pub type SdmmcDebnce = crate::Reg<sdmmc_debnce::SdmmcDebnceSpec>;
#[doc = "Card detect debounce register"]
pub mod sdmmc_debnce;
#[doc = "SDMMC_USRID (rw) register accessor: User ID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_usrid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_usrid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmmc_usrid`]
module"]
#[doc(alias = "SDMMC_USRID")]
pub type SdmmcUsrid = crate::Reg<sdmmc_usrid::SdmmcUsridSpec>;
#[doc = "User ID register"]
pub mod sdmmc_usrid;
#[doc = "SDMMC_VERID (r) register accessor: Version ID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_verid::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmmc_verid`]
module"]
#[doc(alias = "SDMMC_VERID")]
pub type SdmmcVerid = crate::Reg<sdmmc_verid::SdmmcVeridSpec>;
#[doc = "Version ID register"]
pub mod sdmmc_verid;
#[doc = "SDMMC_HCON (r) register accessor: Hardware configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_hcon::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmmc_hcon`]
module"]
#[doc(alias = "SDMMC_HCON")]
pub type SdmmcHcon = crate::Reg<sdmmc_hcon::SdmmcHconSpec>;
#[doc = "Hardware configuration register"]
pub mod sdmmc_hcon;
#[doc = "SDMMC_UHS_REG (rw) register accessor: UHS-1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_uhs_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_uhs_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmmc_uhs_reg`]
module"]
#[doc(alias = "SDMMC_UHS_REG")]
pub type SdmmcUhsReg = crate::Reg<sdmmc_uhs_reg::SdmmcUhsRegSpec>;
#[doc = "UHS-1 register"]
pub mod sdmmc_uhs_reg;
#[doc = "SDMMC_RST_n (rw) register accessor: Hardware reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_rst_n::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_rst_n::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmmc_rst_n`]
module"]
#[doc(alias = "SDMMC_RST_n")]
pub type SdmmcRstN = crate::Reg<sdmmc_rst_n::SdmmcRstNSpec>;
#[doc = "Hardware reset register"]
pub mod sdmmc_rst_n;
#[doc = "SDMMC_BMOD (rw) register accessor: Bus mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_bmod::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_bmod::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmmc_bmod`]
module"]
#[doc(alias = "SDMMC_BMOD")]
pub type SdmmcBmod = crate::Reg<sdmmc_bmod::SdmmcBmodSpec>;
#[doc = "Bus mode register"]
pub mod sdmmc_bmod;
#[doc = "SDMMC_PLDMND (w) register accessor: Poll demand register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_pldmnd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmmc_pldmnd`]
module"]
#[doc(alias = "SDMMC_PLDMND")]
pub type SdmmcPldmnd = crate::Reg<sdmmc_pldmnd::SdmmcPldmndSpec>;
#[doc = "Poll demand register"]
pub mod sdmmc_pldmnd;
#[doc = "SDMMC_DBADDR (rw) register accessor: Descriptor list base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_dbaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_dbaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmmc_dbaddr`]
module"]
#[doc(alias = "SDMMC_DBADDR")]
pub type SdmmcDbaddr = crate::Reg<sdmmc_dbaddr::SdmmcDbaddrSpec>;
#[doc = "Descriptor list base address register"]
pub mod sdmmc_dbaddr;
#[doc = "SDMMC_IDSTS (rw) register accessor: Internal DMAC status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_idsts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_idsts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmmc_idsts`]
module"]
#[doc(alias = "SDMMC_IDSTS")]
pub type SdmmcIdsts = crate::Reg<sdmmc_idsts::SdmmcIdstsSpec>;
#[doc = "Internal DMAC status register"]
pub mod sdmmc_idsts;
#[doc = "SDMMC_IDINTEN (rw) register accessor: Internal DMAC interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_idinten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_idinten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmmc_idinten`]
module"]
#[doc(alias = "SDMMC_IDINTEN")]
pub type SdmmcIdinten = crate::Reg<sdmmc_idinten::SdmmcIdintenSpec>;
#[doc = "Internal DMAC interrupt enable register"]
pub mod sdmmc_idinten;
#[doc = "SDMMC_DSCADDR (rw) register accessor: Current host descriptor address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_dscaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_dscaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmmc_dscaddr`]
module"]
#[doc(alias = "SDMMC_DSCADDR")]
pub type SdmmcDscaddr = crate::Reg<sdmmc_dscaddr::SdmmcDscaddrSpec>;
#[doc = "Current host descriptor address register"]
pub mod sdmmc_dscaddr;
#[doc = "SDMMC_BUFADDR (rw) register accessor: Current buffer descriptor address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_bufaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_bufaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmmc_bufaddr`]
module"]
#[doc(alias = "SDMMC_BUFADDR")]
pub type SdmmcBufaddr = crate::Reg<sdmmc_bufaddr::SdmmcBufaddrSpec>;
#[doc = "Current buffer descriptor address register"]
pub mod sdmmc_bufaddr;
#[doc = "SDMMC_CARDTHRCTL (rw) register accessor: Card read threshold enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_cardthrctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_cardthrctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmmc_cardthrctl`]
module"]
#[doc(alias = "SDMMC_CARDTHRCTL")]
pub type SdmmcCardthrctl = crate::Reg<sdmmc_cardthrctl::SdmmcCardthrctlSpec>;
#[doc = "Card read threshold enable register"]
pub mod sdmmc_cardthrctl;
#[doc = "SDMMC_BACK_END_POWER (rw) register accessor: Back-end power register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_back_end_power::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_back_end_power::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmmc_back_end_power`]
module"]
#[doc(alias = "SDMMC_BACK_END_POWER")]
pub type SdmmcBackEndPower = crate::Reg<sdmmc_back_end_power::SdmmcBackEndPowerSpec>;
#[doc = "Back-end power register"]
pub mod sdmmc_back_end_power;
#[doc = "SDMMC_EMMC_DDR_REG (rw) register accessor: eMMC4.5 DDR start bit detection control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_emmc_ddr_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_emmc_ddr_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmmc_emmc_ddr_reg`]
module"]
#[doc(alias = "SDMMC_EMMC_DDR_REG")]
pub type SdmmcEmmcDdrReg = crate::Reg<sdmmc_emmc_ddr_reg::SdmmcEmmcDdrRegSpec>;
#[doc = "eMMC4.5 DDR start bit detection control register"]
pub mod sdmmc_emmc_ddr_reg;
#[doc = "SDMMC_FIFO_BASE (rw) register accessor: FIFO base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_fifo_base::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_fifo_base::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmmc_fifo_base`]
module"]
#[doc(alias = "SDMMC_FIFO_BASE")]
pub type SdmmcFifoBase = crate::Reg<sdmmc_fifo_base::SdmmcFifoBaseSpec>;
#[doc = "FIFO base address register"]
pub mod sdmmc_fifo_base;
