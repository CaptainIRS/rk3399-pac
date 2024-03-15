#[doc = "Register `PCIE_CLIENT_INT_STATUS` reader"]
pub type R = crate::R<PcieClientIntStatusSpec>;
#[doc = "Register `PCIE_CLIENT_INT_STATUS` writer"]
pub type W = crate::W<PcieClientIntStatusSpec>;
#[doc = "Power state change interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PwrStcgInt {
    #[doc = "0: interrupt The core asserts this output when the power state of a Physical or Virtual Function is being changed to the D1 or D3 states by a write into its Power Management Control Register. The core maintains this output high until the client asserts the pwr_stcg_ack input to the core. While interrupt remains high, the core will not return completions for any pending configuration read or write transaction received by the core. The intent is to delay the completion for the configuration write transaction that caused the state change until the client is ready to transition to the low- power state. When interrupt is asserted, the Function number associated with the configuration write transaction is provided on the pwr_stcg_fc_num. When the client asserts pwr_stcg_ack, the new state of the Function that underwent the state change will be reflected on the fc_pwr_st (for PFs) or the vf_pwr_st (for VFs) outputs of the core."]
    B0 = 0,
    #[doc = "1: interrupt The core asserts this output when the power state of a Physical or Virtual Function is being changed to the D1 or D3 states by a write into its Power Management Control Register. The core maintains this output high until the client asserts the pwr_stcg_ack input to the core. While interrupt remains high, the core will not return completions for any pending configuration read or write transaction received by the core. The intent is to delay the completion for the configuration write transaction that caused the state change until the client is ready to transition to the low- power state. When interrupt is asserted, the Function number associated with the configuration write transaction is provided on the pwr_stcg_fc_num. When the client asserts pwr_stcg_ack, the new state of the Function that underwent the state change will be reflected on the fc_pwr_st (for PFs) or the vf_pwr_st (for VFs) outputs of the core."]
    B1 = 1,
}
impl From<PwrStcgInt> for bool {
    #[inline(always)]
    fn from(variant: PwrStcgInt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWR_STCG_INT` reader - Power state change interrupt"]
pub type PwrStcgIntR = crate::BitReader<PwrStcgInt>;
impl PwrStcgIntR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PwrStcgInt {
        match self.bits {
            false => PwrStcgInt::B0,
            true => PwrStcgInt::B1,
        }
    }
    #[doc = "interrupt The core asserts this output when the power state of a Physical or Virtual Function is being changed to the D1 or D3 states by a write into its Power Management Control Register. The core maintains this output high until the client asserts the pwr_stcg_ack input to the core. While interrupt remains high, the core will not return completions for any pending configuration read or write transaction received by the core. The intent is to delay the completion for the configuration write transaction that caused the state change until the client is ready to transition to the low- power state. When interrupt is asserted, the Function number associated with the configuration write transaction is provided on the pwr_stcg_fc_num. When the client asserts pwr_stcg_ack, the new state of the Function that underwent the state change will be reflected on the fc_pwr_st (for PFs) or the vf_pwr_st (for VFs) outputs of the core."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PwrStcgInt::B0
    }
    #[doc = "interrupt The core asserts this output when the power state of a Physical or Virtual Function is being changed to the D1 or D3 states by a write into its Power Management Control Register. The core maintains this output high until the client asserts the pwr_stcg_ack input to the core. While interrupt remains high, the core will not return completions for any pending configuration read or write transaction received by the core. The intent is to delay the completion for the configuration write transaction that caused the state change until the client is ready to transition to the low- power state. When interrupt is asserted, the Function number associated with the configuration write transaction is provided on the pwr_stcg_fc_num. When the client asserts pwr_stcg_ack, the new state of the Function that underwent the state change will be reflected on the fc_pwr_st (for PFs) or the vf_pwr_st (for VFs) outputs of the core."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PwrStcgInt::B1
    }
}
#[doc = "Hot plug interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HotPlugInt {
    #[doc = "0: interrupt Hot Plug Interrupt Output for Software Notification of Hot Plug events. Currently, this interrupt reserved"]
    B0 = 0,
    #[doc = "1: interrupt Hot Plug Interrupt Output for Software Notification of Hot Plug events. Currently, this interrupt reserved"]
    B1 = 1,
}
impl From<HotPlugInt> for bool {
    #[inline(always)]
    fn from(variant: HotPlugInt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HOT_PLUG_INT` reader - Hot plug interrupt"]
pub type HotPlugIntR = crate::BitReader<HotPlugInt>;
impl HotPlugIntR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HotPlugInt {
        match self.bits {
            false => HotPlugInt::B0,
            true => HotPlugInt::B1,
        }
    }
    #[doc = "interrupt Hot Plug Interrupt Output for Software Notification of Hot Plug events. Currently, this interrupt reserved"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == HotPlugInt::B0
    }
    #[doc = "interrupt Hot Plug Interrupt Output for Software Notification of Hot Plug events. Currently, this interrupt reserved"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == HotPlugInt::B1
    }
}
#[doc = "Phy interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PhyInt {
    #[doc = "0: interrupt This interrupt is used by the core in the RP mode to signal one of the following link training-related events: 1. The link bandwidth changed as a result of the change in the link width or operating speed and the change was initiated locally (not by the link partner), without the link going down. This interrupt is enabled by the Link Bandwidth Management Interrupt Enable bit in the Link Control Register. The status of this interrupt can be read from the Link Bandwidth Management Status bit of the Link Status Register. 2. The link bandwidth changed autonomously as a result of the change in the link width or operating speed and the change was initiated by the remote node. This interrupt is enabled by the Link Autonomous Bandwidth Interrupt Enable bit in the Link Control Register. The status of this interrupt can be read from the Link Autonomous Bandwidth Status bit of the Link Status Register. The phy interrupt is not active when the core is configured as an EndPoint."]
    B0 = 0,
    #[doc = "1: interrupt This interrupt is used by the core in the RP mode to signal one of the following link training-related events: 1. The link bandwidth changed as a result of the change in the link width or operating speed and the change was initiated locally (not by the link partner), without the link going down. This interrupt is enabled by the Link Bandwidth Management Interrupt Enable bit in the Link Control Register. The status of this interrupt can be read from the Link Bandwidth Management Status bit of the Link Status Register. 2. The link bandwidth changed autonomously as a result of the change in the link width or operating speed and the change was initiated by the remote node. This interrupt is enabled by the Link Autonomous Bandwidth Interrupt Enable bit in the Link Control Register. The status of this interrupt can be read from the Link Autonomous Bandwidth Status bit of the Link Status Register. The phy interrupt is not active when the core is configured as an EndPoint."]
    B1 = 1,
}
impl From<PhyInt> for bool {
    #[inline(always)]
    fn from(variant: PhyInt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PHY_INT` reader - Phy interrupt"]
pub type PhyIntR = crate::BitReader<PhyInt>;
impl PhyIntR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PhyInt {
        match self.bits {
            false => PhyInt::B0,
            true => PhyInt::B1,
        }
    }
    #[doc = "interrupt This interrupt is used by the core in the RP mode to signal one of the following link training-related events: 1. The link bandwidth changed as a result of the change in the link width or operating speed and the change was initiated locally (not by the link partner), without the link going down. This interrupt is enabled by the Link Bandwidth Management Interrupt Enable bit in the Link Control Register. The status of this interrupt can be read from the Link Bandwidth Management Status bit of the Link Status Register. 2. The link bandwidth changed autonomously as a result of the change in the link width or operating speed and the change was initiated by the remote node. This interrupt is enabled by the Link Autonomous Bandwidth Interrupt Enable bit in the Link Control Register. The status of this interrupt can be read from the Link Autonomous Bandwidth Status bit of the Link Status Register. The phy interrupt is not active when the core is configured as an EndPoint."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PhyInt::B0
    }
    #[doc = "interrupt This interrupt is used by the core in the RP mode to signal one of the following link training-related events: 1. The link bandwidth changed as a result of the change in the link width or operating speed and the change was initiated locally (not by the link partner), without the link going down. This interrupt is enabled by the Link Bandwidth Management Interrupt Enable bit in the Link Control Register. The status of this interrupt can be read from the Link Bandwidth Management Status bit of the Link Status Register. 2. The link bandwidth changed autonomously as a result of the change in the link width or operating speed and the change was initiated by the remote node. This interrupt is enabled by the Link Autonomous Bandwidth Interrupt Enable bit in the Link Control Register. The status of this interrupt can be read from the Link Autonomous Bandwidth Status bit of the Link Status Register. The phy interrupt is not active when the core is configured as an EndPoint."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PhyInt::B1
    }
}
#[doc = "uDMA interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UdmaInt {
    #[doc = "0: interrupt DMA Interrupt to the system processor. Will be asserted on a \"DMA Done\" or a \"DMA Error\" event"]
    B0 = 0,
    #[doc = "1: interrupt DMA Interrupt to the system processor. Will be asserted on a \"DMA Done\" or a \"DMA Error\" event"]
    B1 = 1,
}
impl From<UdmaInt> for bool {
    #[inline(always)]
    fn from(variant: UdmaInt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UDMA_INT` reader - uDMA interrupt"]
pub type UdmaIntR = crate::BitReader<UdmaInt>;
impl UdmaIntR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UdmaInt {
        match self.bits {
            false => UdmaInt::B0,
            true => UdmaInt::B1,
        }
    }
    #[doc = "interrupt DMA Interrupt to the system processor. Will be asserted on a \"DMA Done\" or a \"DMA Error\" event"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == UdmaInt::B0
    }
    #[doc = "interrupt DMA Interrupt to the system processor. Will be asserted on a \"DMA Done\" or a \"DMA Error\" event"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == UdmaInt::B1
    }
}
#[doc = "Local interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LocalInt {
    #[doc = "0: interrupt Local Error and Status Register Interrupt. This is a level interrupt till cleared by software Detail information refers to Local Error and Status Register description in PCIe Core register section \"Local Management Registers\""]
    B0 = 0,
    #[doc = "1: interrupt Local Error and Status Register Interrupt. This is a level interrupt till cleared by software Detail information refers to Local Error and Status Register description in PCIe Core register section \"Local Management Registers\""]
    B1 = 1,
}
impl From<LocalInt> for bool {
    #[inline(always)]
    fn from(variant: LocalInt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCAL_INT` reader - Local interrupt"]
pub type LocalIntR = crate::BitReader<LocalInt>;
impl LocalIntR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LocalInt {
        match self.bits {
            false => LocalInt::B0,
            true => LocalInt::B1,
        }
    }
    #[doc = "interrupt Local Error and Status Register Interrupt. This is a level interrupt till cleared by software Detail information refers to Local Error and Status Register description in PCIe Core register section \"Local Management Registers\""]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == LocalInt::B0
    }
    #[doc = "interrupt Local Error and Status Register Interrupt. This is a level interrupt till cleared by software Detail information refers to Local Error and Status Register description in PCIe Core register section \"Local Management Registers\""]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == LocalInt::B1
    }
}
#[doc = "INTA interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Inta {
    #[doc = "0: interrupt When the core is configured as RC, this interrupt emulate the PCI legacy interrupts INTA. The core asserts an interrupt output when it has received an Assert_INTA message from the link, and deasserts it when it receives a Deassert_INTA message."]
    B0 = 0,
    #[doc = "1: interrupt When the core is configured as RC, this interrupt emulate the PCI legacy interrupts INTA. The core asserts an interrupt output when it has received an Assert_INTA message from the link, and deasserts it when it receives a Deassert_INTA message."]
    B1 = 1,
}
impl From<Inta> for bool {
    #[inline(always)]
    fn from(variant: Inta) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTA` reader - INTA interrupt"]
pub type IntaR = crate::BitReader<Inta>;
impl IntaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Inta {
        match self.bits {
            false => Inta::B0,
            true => Inta::B1,
        }
    }
    #[doc = "interrupt When the core is configured as RC, this interrupt emulate the PCI legacy interrupts INTA. The core asserts an interrupt output when it has received an Assert_INTA message from the link, and deasserts it when it receives a Deassert_INTA message."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Inta::B0
    }
    #[doc = "interrupt When the core is configured as RC, this interrupt emulate the PCI legacy interrupts INTA. The core asserts an interrupt output when it has received an Assert_INTA message from the link, and deasserts it when it receives a Deassert_INTA message."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Inta::B1
    }
}
#[doc = "INTB interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Intb {
    #[doc = "0: interrupt When the core is configured as RC, this interrupt emulate the PCI legacy interrupts INTB. The core asserts an interrupt output when it has received an Assert_INTB message from the link, and deasserts it when it receives a Deassert_INTB message."]
    B0 = 0,
    #[doc = "1: interrupt When the core is configured as RC, this interrupt emulate the PCI legacy interrupts INTB. The core asserts an interrupt output when it has received an Assert_INTB message from the link, and deasserts it when it receives a Deassert_INTB message."]
    B1 = 1,
}
impl From<Intb> for bool {
    #[inline(always)]
    fn from(variant: Intb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTB` reader - INTB interrupt"]
pub type IntbR = crate::BitReader<Intb>;
impl IntbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Intb {
        match self.bits {
            false => Intb::B0,
            true => Intb::B1,
        }
    }
    #[doc = "interrupt When the core is configured as RC, this interrupt emulate the PCI legacy interrupts INTB. The core asserts an interrupt output when it has received an Assert_INTB message from the link, and deasserts it when it receives a Deassert_INTB message."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Intb::B0
    }
    #[doc = "interrupt When the core is configured as RC, this interrupt emulate the PCI legacy interrupts INTB. The core asserts an interrupt output when it has received an Assert_INTB message from the link, and deasserts it when it receives a Deassert_INTB message."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Intb::B1
    }
}
#[doc = "INTC interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Intc {
    #[doc = "0: interrupt When the core is configured as RC, this interrupt emulate the PCI legacy interrupts INTC. The core asserts an interrupt output when it has received an Assert_INTC message from the link, and deasserts it when it receives a Deassert_INTC message."]
    B0 = 0,
    #[doc = "1: interrupt When the core is configured as RC, this interrupt emulate the PCI legacy interrupts INTC. The core asserts an interrupt output when it has received an Assert_INTC message from the link, and deasserts it when it receives a Deassert_INTC message."]
    B1 = 1,
}
impl From<Intc> for bool {
    #[inline(always)]
    fn from(variant: Intc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTC` reader - INTC interrupt"]
pub type IntcR = crate::BitReader<Intc>;
impl IntcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Intc {
        match self.bits {
            false => Intc::B0,
            true => Intc::B1,
        }
    }
    #[doc = "interrupt When the core is configured as RC, this interrupt emulate the PCI legacy interrupts INTC. The core asserts an interrupt output when it has received an Assert_INTC message from the link, and deasserts it when it receives a Deassert_INTC message."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Intc::B0
    }
    #[doc = "interrupt When the core is configured as RC, this interrupt emulate the PCI legacy interrupts INTC. The core asserts an interrupt output when it has received an Assert_INTC message from the link, and deasserts it when it receives a Deassert_INTC message."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Intc::B1
    }
}
#[doc = "INTD interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Intd {
    #[doc = "0: interrupt When the core is configured as RC, this interrupt emulate the PCI legacy interrupts INTD. The core asserts an interrupt output when it has received an Assert_INTD message from the link, and deasserts it when it receives a Deassert_INTD message."]
    B0 = 0,
    #[doc = "1: interrupt When the core is configured as RC, this interrupt emulate the PCI legacy interrupts INTD. The core asserts an interrupt output when it has received an Assert_INTD message from the link, and deasserts it when it receives a Deassert_INTD message."]
    B1 = 1,
}
impl From<Intd> for bool {
    #[inline(always)]
    fn from(variant: Intd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTD` reader - INTD interrupt"]
pub type IntdR = crate::BitReader<Intd>;
impl IntdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Intd {
        match self.bits {
            false => Intd::B0,
            true => Intd::B1,
        }
    }
    #[doc = "interrupt When the core is configured as RC, this interrupt emulate the PCI legacy interrupts INTD. The core asserts an interrupt output when it has received an Assert_INTD message from the link, and deasserts it when it receives a Deassert_INTD message."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Intd::B0
    }
    #[doc = "interrupt When the core is configured as RC, this interrupt emulate the PCI legacy interrupts INTD. The core asserts an interrupt output when it has received an Assert_INTD message from the link, and deasserts it when it receives a Deassert_INTD message."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Intd::B1
    }
}
#[doc = "Correctable error interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CorrErrInt {
    #[doc = "0: interrupt In the EP mode, the core activates this output for one cycle when it has detected a correctable error and its reporting is not masked. In multi-Function cores, this is the logical OR of the correctable error status bits in the Device Status Registers of all Functions. In the RC mode, this output is activated on detection of a local correctable error, when its reporting is not masked. This signal also gets activated in response to an error message received from the link if Correctable Error Reporting is enabled in the Root Error Command register."]
    B0 = 0,
    #[doc = "1: interrupt In the EP mode, the core activates this output for one cycle when it has detected a correctable error and its reporting is not masked. In multi-Function cores, this is the logical OR of the correctable error status bits in the Device Status Registers of all Functions. In the RC mode, this output is activated on detection of a local correctable error, when its reporting is not masked. This signal also gets activated in response to an error message received from the link if Correctable Error Reporting is enabled in the Root Error Command register."]
    B1 = 1,
}
impl From<CorrErrInt> for bool {
    #[inline(always)]
    fn from(variant: CorrErrInt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CORR_ERR_INT` reader - Correctable error interrupt"]
pub type CorrErrIntR = crate::BitReader<CorrErrInt>;
impl CorrErrIntR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CorrErrInt {
        match self.bits {
            false => CorrErrInt::B0,
            true => CorrErrInt::B1,
        }
    }
    #[doc = "interrupt In the EP mode, the core activates this output for one cycle when it has detected a correctable error and its reporting is not masked. In multi-Function cores, this is the logical OR of the correctable error status bits in the Device Status Registers of all Functions. In the RC mode, this output is activated on detection of a local correctable error, when its reporting is not masked. This signal also gets activated in response to an error message received from the link if Correctable Error Reporting is enabled in the Root Error Command register."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == CorrErrInt::B0
    }
    #[doc = "interrupt In the EP mode, the core activates this output for one cycle when it has detected a correctable error and its reporting is not masked. In multi-Function cores, this is the logical OR of the correctable error status bits in the Device Status Registers of all Functions. In the RC mode, this output is activated on detection of a local correctable error, when its reporting is not masked. This signal also gets activated in response to an error message received from the link if Correctable Error Reporting is enabled in the Root Error Command register."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == CorrErrInt::B1
    }
}
#[doc = "Field `CORR_ERR_INT` writer - Correctable error interrupt"]
pub type CorrErrIntW<'a, REG> = crate::BitWriter1C<'a, REG, CorrErrInt>;
impl<'a, REG> CorrErrIntW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "interrupt In the EP mode, the core activates this output for one cycle when it has detected a correctable error and its reporting is not masked. In multi-Function cores, this is the logical OR of the correctable error status bits in the Device Status Registers of all Functions. In the RC mode, this output is activated on detection of a local correctable error, when its reporting is not masked. This signal also gets activated in response to an error message received from the link if Correctable Error Reporting is enabled in the Root Error Command register."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(CorrErrInt::B0)
    }
    #[doc = "interrupt In the EP mode, the core activates this output for one cycle when it has detected a correctable error and its reporting is not masked. In multi-Function cores, this is the logical OR of the correctable error status bits in the Device Status Registers of all Functions. In the RC mode, this output is activated on detection of a local correctable error, when its reporting is not masked. This signal also gets activated in response to an error message received from the link if Correctable Error Reporting is enabled in the Root Error Command register."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(CorrErrInt::B1)
    }
}
#[doc = "Non-fatal error interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NfatalErrInt {
    #[doc = "0: interrupt In the EP mode, the core activates this output for one cycle when it has detected a non-fatal error and its reporting is not masked. In multi-Function cores, this is the logical OR of the non-fatal error status bits in the Device Status Registers of all Functions. In the RC mode, this output is activated on detection of a local fatal error, when its reporting is not masked. This signal also gets activated in response to an error message received from the link if Fatal Error Reporting is enabled in the Root Error Command register."]
    B0 = 0,
    #[doc = "1: interrupt In the EP mode, the core activates this output for one cycle when it has detected a non-fatal error and its reporting is not masked. In multi-Function cores, this is the logical OR of the non-fatal error status bits in the Device Status Registers of all Functions. In the RC mode, this output is activated on detection of a local fatal error, when its reporting is not masked. This signal also gets activated in response to an error message received from the link if Fatal Error Reporting is enabled in the Root Error Command register."]
    B1 = 1,
}
impl From<NfatalErrInt> for bool {
    #[inline(always)]
    fn from(variant: NfatalErrInt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NFATAL_ERR_INT` reader - Non-fatal error interrupt"]
pub type NfatalErrIntR = crate::BitReader<NfatalErrInt>;
impl NfatalErrIntR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NfatalErrInt {
        match self.bits {
            false => NfatalErrInt::B0,
            true => NfatalErrInt::B1,
        }
    }
    #[doc = "interrupt In the EP mode, the core activates this output for one cycle when it has detected a non-fatal error and its reporting is not masked. In multi-Function cores, this is the logical OR of the non-fatal error status bits in the Device Status Registers of all Functions. In the RC mode, this output is activated on detection of a local fatal error, when its reporting is not masked. This signal also gets activated in response to an error message received from the link if Fatal Error Reporting is enabled in the Root Error Command register."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == NfatalErrInt::B0
    }
    #[doc = "interrupt In the EP mode, the core activates this output for one cycle when it has detected a non-fatal error and its reporting is not masked. In multi-Function cores, this is the logical OR of the non-fatal error status bits in the Device Status Registers of all Functions. In the RC mode, this output is activated on detection of a local fatal error, when its reporting is not masked. This signal also gets activated in response to an error message received from the link if Fatal Error Reporting is enabled in the Root Error Command register."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == NfatalErrInt::B1
    }
}
#[doc = "Field `NFATAL_ERR_INT` writer - Non-fatal error interrupt"]
pub type NfatalErrIntW<'a, REG> = crate::BitWriter1C<'a, REG, NfatalErrInt>;
impl<'a, REG> NfatalErrIntW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "interrupt In the EP mode, the core activates this output for one cycle when it has detected a non-fatal error and its reporting is not masked. In multi-Function cores, this is the logical OR of the non-fatal error status bits in the Device Status Registers of all Functions. In the RC mode, this output is activated on detection of a local fatal error, when its reporting is not masked. This signal also gets activated in response to an error message received from the link if Fatal Error Reporting is enabled in the Root Error Command register."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(NfatalErrInt::B0)
    }
    #[doc = "interrupt In the EP mode, the core activates this output for one cycle when it has detected a non-fatal error and its reporting is not masked. In multi-Function cores, this is the logical OR of the non-fatal error status bits in the Device Status Registers of all Functions. In the RC mode, this output is activated on detection of a local fatal error, when its reporting is not masked. This signal also gets activated in response to an error message received from the link if Fatal Error Reporting is enabled in the Root Error Command register."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(NfatalErrInt::B1)
    }
}
#[doc = "Fatal error interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FatalErrInt {
    #[doc = "0: interrupt In the EP mode, the core activates this output for one cycle when it has detected a fatal error and its reporting is not masked. In multi-Function cores, this is the logical OR of the fatal error status bits in the Device Status Registers of all Functions. In the RP mode, this output is activated on detection of a local fatal error, when its reporting is not masked. This signal also gets activated in response to an error message received from the link if Fatal Error Reporting is enabled in the Root Error Command register."]
    B0 = 0,
    #[doc = "1: interrupt In the EP mode, the core activates this output for one cycle when it has detected a fatal error and its reporting is not masked. In multi-Function cores, this is the logical OR of the fatal error status bits in the Device Status Registers of all Functions. In the RP mode, this output is activated on detection of a local fatal error, when its reporting is not masked. This signal also gets activated in response to an error message received from the link if Fatal Error Reporting is enabled in the Root Error Command register."]
    B1 = 1,
}
impl From<FatalErrInt> for bool {
    #[inline(always)]
    fn from(variant: FatalErrInt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FATAL_ERR_INT` reader - Fatal error interrupt"]
pub type FatalErrIntR = crate::BitReader<FatalErrInt>;
impl FatalErrIntR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FatalErrInt {
        match self.bits {
            false => FatalErrInt::B0,
            true => FatalErrInt::B1,
        }
    }
    #[doc = "interrupt In the EP mode, the core activates this output for one cycle when it has detected a fatal error and its reporting is not masked. In multi-Function cores, this is the logical OR of the fatal error status bits in the Device Status Registers of all Functions. In the RP mode, this output is activated on detection of a local fatal error, when its reporting is not masked. This signal also gets activated in response to an error message received from the link if Fatal Error Reporting is enabled in the Root Error Command register."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == FatalErrInt::B0
    }
    #[doc = "interrupt In the EP mode, the core activates this output for one cycle when it has detected a fatal error and its reporting is not masked. In multi-Function cores, this is the logical OR of the fatal error status bits in the Device Status Registers of all Functions. In the RP mode, this output is activated on detection of a local fatal error, when its reporting is not masked. This signal also gets activated in response to an error message received from the link if Fatal Error Reporting is enabled in the Root Error Command register."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == FatalErrInt::B1
    }
}
#[doc = "Field `FATAL_ERR_INT` writer - Fatal error interrupt"]
pub type FatalErrIntW<'a, REG> = crate::BitWriter1C<'a, REG, FatalErrInt>;
impl<'a, REG> FatalErrIntW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "interrupt In the EP mode, the core activates this output for one cycle when it has detected a fatal error and its reporting is not masked. In multi-Function cores, this is the logical OR of the fatal error status bits in the Device Status Registers of all Functions. In the RP mode, this output is activated on detection of a local fatal error, when its reporting is not masked. This signal also gets activated in response to an error message received from the link if Fatal Error Reporting is enabled in the Root Error Command register."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(FatalErrInt::B0)
    }
    #[doc = "interrupt In the EP mode, the core activates this output for one cycle when it has detected a fatal error and its reporting is not masked. In multi-Function cores, this is the logical OR of the fatal error status bits in the Device Status Registers of all Functions. In the RP mode, this output is activated on detection of a local fatal error, when its reporting is not masked. This signal also gets activated in response to an error message received from the link if Fatal Error Reporting is enabled in the Root Error Command register."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(FatalErrInt::B1)
    }
}
#[doc = "DPA interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DpaInt {
    #[doc = "0: interrupt The core generates an interrupt when a Configuration Write transaction writes into the Dynamic Power Allocation Control Register to modify the DPA power state of the device. A interrupt indicates such a DPA event for PF 0, and so on. The local software running on the End Point must read the DPA Control Register of the corresponding Function to determine the DPA substate requested by the host and set the power state of the device accordingly"]
    B0 = 0,
    #[doc = "1: interrupt The core generates an interrupt when a Configuration Write transaction writes into the Dynamic Power Allocation Control Register to modify the DPA power state of the device. A interrupt indicates such a DPA event for PF 0, and so on. The local software running on the End Point must read the DPA Control Register of the corresponding Function to determine the DPA substate requested by the host and set the power state of the device accordingly"]
    B1 = 1,
}
impl From<DpaInt> for bool {
    #[inline(always)]
    fn from(variant: DpaInt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DPA_INT` reader - DPA interrupt"]
pub type DpaIntR = crate::BitReader<DpaInt>;
impl DpaIntR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DpaInt {
        match self.bits {
            false => DpaInt::B0,
            true => DpaInt::B1,
        }
    }
    #[doc = "interrupt The core generates an interrupt when a Configuration Write transaction writes into the Dynamic Power Allocation Control Register to modify the DPA power state of the device. A interrupt indicates such a DPA event for PF 0, and so on. The local software running on the End Point must read the DPA Control Register of the corresponding Function to determine the DPA substate requested by the host and set the power state of the device accordingly"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DpaInt::B0
    }
    #[doc = "interrupt The core generates an interrupt when a Configuration Write transaction writes into the Dynamic Power Allocation Control Register to modify the DPA power state of the device. A interrupt indicates such a DPA event for PF 0, and so on. The local software running on the End Point must read the DPA Control Register of the corresponding Function to determine the DPA substate requested by the host and set the power state of the device accordingly"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DpaInt::B1
    }
}
#[doc = "Field `DPA_INT` writer - DPA interrupt"]
pub type DpaIntW<'a, REG> = crate::BitWriter1C<'a, REG, DpaInt>;
impl<'a, REG> DpaIntW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "interrupt The core generates an interrupt when a Configuration Write transaction writes into the Dynamic Power Allocation Control Register to modify the DPA power state of the device. A interrupt indicates such a DPA event for PF 0, and so on. The local software running on the End Point must read the DPA Control Register of the corresponding Function to determine the DPA substate requested by the host and set the power state of the device accordingly"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(DpaInt::B0)
    }
    #[doc = "interrupt The core generates an interrupt when a Configuration Write transaction writes into the Dynamic Power Allocation Control Register to modify the DPA power state of the device. A interrupt indicates such a DPA event for PF 0, and so on. The local software running on the End Point must read the DPA Control Register of the corresponding Function to determine the DPA substate requested by the host and set the power state of the device accordingly"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(DpaInt::B1)
    }
}
#[doc = "Hot reset interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HotResetInt {
    #[doc = "0: interrupt When a hot reset send done interrupt generated in RC mode, it indicates that the Endpoint Device has also received the Hot Reset, and then the hot_reset_in can be de-assert"]
    B0 = 0,
    #[doc = "1: interrupt When a hot reset send done interrupt generated in RC mode, it indicates that the Endpoint Device has also received the Hot Reset, and then the hot_reset_in can be de-assert"]
    B1 = 1,
}
impl From<HotResetInt> for bool {
    #[inline(always)]
    fn from(variant: HotResetInt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HOT_RESET_INT` reader - Hot reset interrupt"]
pub type HotResetIntR = crate::BitReader<HotResetInt>;
impl HotResetIntR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HotResetInt {
        match self.bits {
            false => HotResetInt::B0,
            true => HotResetInt::B1,
        }
    }
    #[doc = "interrupt When a hot reset send done interrupt generated in RC mode, it indicates that the Endpoint Device has also received the Hot Reset, and then the hot_reset_in can be de-assert"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == HotResetInt::B0
    }
    #[doc = "interrupt When a hot reset send done interrupt generated in RC mode, it indicates that the Endpoint Device has also received the Hot Reset, and then the hot_reset_in can be de-assert"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == HotResetInt::B1
    }
}
#[doc = "Field `HOT_RESET_INT` writer - Hot reset interrupt"]
pub type HotResetIntW<'a, REG> = crate::BitWriter1C<'a, REG, HotResetInt>;
impl<'a, REG> HotResetIntW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "interrupt When a hot reset send done interrupt generated in RC mode, it indicates that the Endpoint Device has also received the Hot Reset, and then the hot_reset_in can be de-assert"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(HotResetInt::B0)
    }
    #[doc = "interrupt When a hot reset send done interrupt generated in RC mode, it indicates that the Endpoint Device has also received the Hot Reset, and then the hot_reset_in can be de-assert"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(HotResetInt::B1)
    }
}
#[doc = "Message receive done interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MsgInt {
    #[doc = "0: interrupt When a message received done by Client message FIFO, an interrupt will generate"]
    B0 = 0,
    #[doc = "1: interrupt When a message received done by Client message FIFO, an interrupt will generate"]
    B1 = 1,
}
impl From<MsgInt> for bool {
    #[inline(always)]
    fn from(variant: MsgInt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSG_INT` reader - Message receive done interrupt"]
pub type MsgIntR = crate::BitReader<MsgInt>;
impl MsgIntR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MsgInt {
        match self.bits {
            false => MsgInt::B0,
            true => MsgInt::B1,
        }
    }
    #[doc = "interrupt When a message received done by Client message FIFO, an interrupt will generate"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == MsgInt::B0
    }
    #[doc = "interrupt When a message received done by Client message FIFO, an interrupt will generate"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == MsgInt::B1
    }
}
#[doc = "Field `MSG_INT` writer - Message receive done interrupt"]
pub type MsgIntW<'a, REG> = crate::BitWriter1C<'a, REG, MsgInt>;
impl<'a, REG> MsgIntW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "interrupt When a message received done by Client message FIFO, an interrupt will generate"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(MsgInt::B0)
    }
    #[doc = "interrupt When a message received done by Client message FIFO, an interrupt will generate"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(MsgInt::B1)
    }
}
#[doc = "Legacy interrupt send done interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LegacyDoneInt {
    #[doc = "0: interrupt A pulse on this output indicates that the core has sent an INTx Assert or Deassert message in response to a change in the state of one of the int_in"]
    B0 = 0,
    #[doc = "1: interrupt A pulse on this output indicates that the core has sent an INTx Assert or Deassert message in response to a change in the state of one of the int_in"]
    B1 = 1,
}
impl From<LegacyDoneInt> for bool {
    #[inline(always)]
    fn from(variant: LegacyDoneInt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LEGACY_DONE_INT` reader - Legacy interrupt send done interrupt"]
pub type LegacyDoneIntR = crate::BitReader<LegacyDoneInt>;
impl LegacyDoneIntR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LegacyDoneInt {
        match self.bits {
            false => LegacyDoneInt::B0,
            true => LegacyDoneInt::B1,
        }
    }
    #[doc = "interrupt A pulse on this output indicates that the core has sent an INTx Assert or Deassert message in response to a change in the state of one of the int_in"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == LegacyDoneInt::B0
    }
    #[doc = "interrupt A pulse on this output indicates that the core has sent an INTx Assert or Deassert message in response to a change in the state of one of the int_in"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == LegacyDoneInt::B1
    }
}
#[doc = "Field `LEGACY_DONE_INT` writer - Legacy interrupt send done interrupt"]
pub type LegacyDoneIntW<'a, REG> = crate::BitWriter1C<'a, REG, LegacyDoneInt>;
impl<'a, REG> LegacyDoneIntW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "interrupt A pulse on this output indicates that the core has sent an INTx Assert or Deassert message in response to a change in the state of one of the int_in"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(LegacyDoneInt::B0)
    }
    #[doc = "interrupt A pulse on this output indicates that the core has sent an INTx Assert or Deassert message in response to a change in the state of one of the int_in"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(LegacyDoneInt::B1)
    }
}
impl R {
    #[doc = "Bit 0 - Power state change interrupt"]
    #[inline(always)]
    pub fn pwr_stcg_int(&self) -> PwrStcgIntR {
        PwrStcgIntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Hot plug interrupt"]
    #[inline(always)]
    pub fn hot_plug_int(&self) -> HotPlugIntR {
        HotPlugIntR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Phy interrupt"]
    #[inline(always)]
    pub fn phy_int(&self) -> PhyIntR {
        PhyIntR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - uDMA interrupt"]
    #[inline(always)]
    pub fn udma_int(&self) -> UdmaIntR {
        UdmaIntR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Local interrupt"]
    #[inline(always)]
    pub fn local_int(&self) -> LocalIntR {
        LocalIntR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - INTA interrupt"]
    #[inline(always)]
    pub fn inta(&self) -> IntaR {
        IntaR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - INTB interrupt"]
    #[inline(always)]
    pub fn intb(&self) -> IntbR {
        IntbR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - INTC interrupt"]
    #[inline(always)]
    pub fn intc(&self) -> IntcR {
        IntcR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - INTD interrupt"]
    #[inline(always)]
    pub fn intd(&self) -> IntdR {
        IntdR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Correctable error interrupt"]
    #[inline(always)]
    pub fn corr_err_int(&self) -> CorrErrIntR {
        CorrErrIntR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Non-fatal error interrupt"]
    #[inline(always)]
    pub fn nfatal_err_int(&self) -> NfatalErrIntR {
        NfatalErrIntR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Fatal error interrupt"]
    #[inline(always)]
    pub fn fatal_err_int(&self) -> FatalErrIntR {
        FatalErrIntR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DPA interrupt"]
    #[inline(always)]
    pub fn dpa_int(&self) -> DpaIntR {
        DpaIntR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Hot reset interrupt"]
    #[inline(always)]
    pub fn hot_reset_int(&self) -> HotResetIntR {
        HotResetIntR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Message receive done interrupt"]
    #[inline(always)]
    pub fn msg_int(&self) -> MsgIntR {
        MsgIntR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Legacy interrupt send done interrupt"]
    #[inline(always)]
    pub fn legacy_done_int(&self) -> LegacyDoneIntR {
        LegacyDoneIntR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 9 - Correctable error interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn corr_err_int(&mut self) -> CorrErrIntW<PcieClientIntStatusSpec> {
        CorrErrIntW::new(self, 9)
    }
    #[doc = "Bit 10 - Non-fatal error interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn nfatal_err_int(&mut self) -> NfatalErrIntW<PcieClientIntStatusSpec> {
        NfatalErrIntW::new(self, 10)
    }
    #[doc = "Bit 11 - Fatal error interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn fatal_err_int(&mut self) -> FatalErrIntW<PcieClientIntStatusSpec> {
        FatalErrIntW::new(self, 11)
    }
    #[doc = "Bit 12 - DPA interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn dpa_int(&mut self) -> DpaIntW<PcieClientIntStatusSpec> {
        DpaIntW::new(self, 12)
    }
    #[doc = "Bit 13 - Hot reset interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn hot_reset_int(&mut self) -> HotResetIntW<PcieClientIntStatusSpec> {
        HotResetIntW::new(self, 13)
    }
    #[doc = "Bit 14 - Message receive done interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn msg_int(&mut self) -> MsgIntW<PcieClientIntStatusSpec> {
        MsgIntW::new(self, 14)
    }
    #[doc = "Bit 15 - Legacy interrupt send done interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn legacy_done_int(&mut self) -> LegacyDoneIntW<PcieClientIntStatusSpec> {
        LegacyDoneIntW::new(self, 15)
    }
}
#[doc = "Interrupt status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_client_int_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_client_int_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieClientIntStatusSpec;
impl crate::RegisterSpec for PcieClientIntStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_client_int_status::R`](R) reader structure"]
impl crate::Readable for PcieClientIntStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`pcie_client_int_status::W`](W) writer structure"]
impl crate::Writable for PcieClientIntStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xfe00;
}
#[doc = "`reset()` method sets PCIE_CLIENT_INT_STATUS to value 0"]
impl crate::Resettable for PcieClientIntStatusSpec {
    const RESET_VALUE: u32 = 0;
}
