#[doc = "Register `PCIE_CLIENT_INT_MASK` reader"]
pub type R = crate::R<PcieClientIntMaskSpec>;
#[doc = "Register `PCIE_CLIENT_INT_MASK` writer"]
pub type W = crate::W<PcieClientIntMaskSpec>;
#[doc = "Power state change interrupt mask\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PwrStcgIntMask {
    #[doc = "0: interrupt enable"]
    B0 = 0,
    #[doc = "1: interrupt mask"]
    B1 = 1,
}
impl From<PwrStcgIntMask> for bool {
    #[inline(always)]
    fn from(variant: PwrStcgIntMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWR_STCG_INT_MASK` reader - Power state change interrupt mask"]
pub type PwrStcgIntMaskR = crate::BitReader<PwrStcgIntMask>;
impl PwrStcgIntMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PwrStcgIntMask {
        match self.bits {
            false => PwrStcgIntMask::B0,
            true => PwrStcgIntMask::B1,
        }
    }
    #[doc = "interrupt enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PwrStcgIntMask::B0
    }
    #[doc = "interrupt mask"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PwrStcgIntMask::B1
    }
}
#[doc = "Field `PWR_STCG_INT_MASK` writer - Power state change interrupt mask"]
pub type PwrStcgIntMaskW<'a, REG> = crate::BitWriter<'a, REG, PwrStcgIntMask>;
impl<'a, REG> PwrStcgIntMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "interrupt enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PwrStcgIntMask::B0)
    }
    #[doc = "interrupt mask"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PwrStcgIntMask::B1)
    }
}
#[doc = "Hot plug interrupt mask\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HotPlugIntMask {
    #[doc = "0: interrupt enable"]
    B0 = 0,
    #[doc = "1: interrupt mask reserved"]
    B1 = 1,
}
impl From<HotPlugIntMask> for bool {
    #[inline(always)]
    fn from(variant: HotPlugIntMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HOT_PLUG_INT_MASK` reader - Hot plug interrupt mask"]
pub type HotPlugIntMaskR = crate::BitReader<HotPlugIntMask>;
impl HotPlugIntMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HotPlugIntMask {
        match self.bits {
            false => HotPlugIntMask::B0,
            true => HotPlugIntMask::B1,
        }
    }
    #[doc = "interrupt enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == HotPlugIntMask::B0
    }
    #[doc = "interrupt mask reserved"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == HotPlugIntMask::B1
    }
}
#[doc = "Field `HOT_PLUG_INT_MASK` writer - Hot plug interrupt mask"]
pub type HotPlugIntMaskW<'a, REG> = crate::BitWriter<'a, REG, HotPlugIntMask>;
impl<'a, REG> HotPlugIntMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "interrupt enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(HotPlugIntMask::B0)
    }
    #[doc = "interrupt mask reserved"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(HotPlugIntMask::B1)
    }
}
#[doc = "Phy interrupt mask\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PhyIntMask {
    #[doc = "0: interrupt enable"]
    B0 = 0,
    #[doc = "1: interrupt mask"]
    B1 = 1,
}
impl From<PhyIntMask> for bool {
    #[inline(always)]
    fn from(variant: PhyIntMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PHY_INT_MASK` reader - Phy interrupt mask"]
pub type PhyIntMaskR = crate::BitReader<PhyIntMask>;
impl PhyIntMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PhyIntMask {
        match self.bits {
            false => PhyIntMask::B0,
            true => PhyIntMask::B1,
        }
    }
    #[doc = "interrupt enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PhyIntMask::B0
    }
    #[doc = "interrupt mask"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PhyIntMask::B1
    }
}
#[doc = "Field `PHY_INT_MASK` writer - Phy interrupt mask"]
pub type PhyIntMaskW<'a, REG> = crate::BitWriter<'a, REG, PhyIntMask>;
impl<'a, REG> PhyIntMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "interrupt enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PhyIntMask::B0)
    }
    #[doc = "interrupt mask"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PhyIntMask::B1)
    }
}
#[doc = "uDMA interrupt mask\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UdmaIntMask {
    #[doc = "0: interrupt enable"]
    B0 = 0,
    #[doc = "1: interrupt mask"]
    B1 = 1,
}
impl From<UdmaIntMask> for bool {
    #[inline(always)]
    fn from(variant: UdmaIntMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UDMA_INT_MASK` reader - uDMA interrupt mask"]
pub type UdmaIntMaskR = crate::BitReader<UdmaIntMask>;
impl UdmaIntMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UdmaIntMask {
        match self.bits {
            false => UdmaIntMask::B0,
            true => UdmaIntMask::B1,
        }
    }
    #[doc = "interrupt enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == UdmaIntMask::B0
    }
    #[doc = "interrupt mask"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == UdmaIntMask::B1
    }
}
#[doc = "Field `UDMA_INT_MASK` writer - uDMA interrupt mask"]
pub type UdmaIntMaskW<'a, REG> = crate::BitWriter<'a, REG, UdmaIntMask>;
impl<'a, REG> UdmaIntMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "interrupt enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(UdmaIntMask::B0)
    }
    #[doc = "interrupt mask"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(UdmaIntMask::B1)
    }
}
#[doc = "Local interrupt mask\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LocalIntMask {
    #[doc = "0: interrupt enable"]
    B0 = 0,
    #[doc = "1: interrupt mask"]
    B1 = 1,
}
impl From<LocalIntMask> for bool {
    #[inline(always)]
    fn from(variant: LocalIntMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCAL_INT_MASK` reader - Local interrupt mask"]
pub type LocalIntMaskR = crate::BitReader<LocalIntMask>;
impl LocalIntMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LocalIntMask {
        match self.bits {
            false => LocalIntMask::B0,
            true => LocalIntMask::B1,
        }
    }
    #[doc = "interrupt enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == LocalIntMask::B0
    }
    #[doc = "interrupt mask"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == LocalIntMask::B1
    }
}
#[doc = "Field `LOCAL_INT_MASK` writer - Local interrupt mask"]
pub type LocalIntMaskW<'a, REG> = crate::BitWriter<'a, REG, LocalIntMask>;
impl<'a, REG> LocalIntMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "interrupt enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(LocalIntMask::B0)
    }
    #[doc = "interrupt mask"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(LocalIntMask::B1)
    }
}
#[doc = "INTA interrupt mask\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntaMask {
    #[doc = "0: interrupt enable"]
    B0 = 0,
    #[doc = "1: interrupt mask"]
    B1 = 1,
}
impl From<IntaMask> for bool {
    #[inline(always)]
    fn from(variant: IntaMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTA_MASK` reader - INTA interrupt mask"]
pub type IntaMaskR = crate::BitReader<IntaMask>;
impl IntaMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntaMask {
        match self.bits {
            false => IntaMask::B0,
            true => IntaMask::B1,
        }
    }
    #[doc = "interrupt enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IntaMask::B0
    }
    #[doc = "interrupt mask"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IntaMask::B1
    }
}
#[doc = "Field `INTA_MASK` writer - INTA interrupt mask"]
pub type IntaMaskW<'a, REG> = crate::BitWriter<'a, REG, IntaMask>;
impl<'a, REG> IntaMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "interrupt enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IntaMask::B0)
    }
    #[doc = "interrupt mask"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IntaMask::B1)
    }
}
#[doc = "INTB interrupt mask\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntbMask {
    #[doc = "0: interrupt enable"]
    B0 = 0,
    #[doc = "1: interrupt mask"]
    B1 = 1,
}
impl From<IntbMask> for bool {
    #[inline(always)]
    fn from(variant: IntbMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTB_MASK` reader - INTB interrupt mask"]
pub type IntbMaskR = crate::BitReader<IntbMask>;
impl IntbMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntbMask {
        match self.bits {
            false => IntbMask::B0,
            true => IntbMask::B1,
        }
    }
    #[doc = "interrupt enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IntbMask::B0
    }
    #[doc = "interrupt mask"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IntbMask::B1
    }
}
#[doc = "Field `INTB_MASK` writer - INTB interrupt mask"]
pub type IntbMaskW<'a, REG> = crate::BitWriter<'a, REG, IntbMask>;
impl<'a, REG> IntbMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "interrupt enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IntbMask::B0)
    }
    #[doc = "interrupt mask"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IntbMask::B1)
    }
}
#[doc = "INTC interrupt mask\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntcMask {
    #[doc = "0: interrupt enable"]
    B0 = 0,
    #[doc = "1: interrupt mask"]
    B1 = 1,
}
impl From<IntcMask> for bool {
    #[inline(always)]
    fn from(variant: IntcMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTC_MASK` reader - INTC interrupt mask"]
pub type IntcMaskR = crate::BitReader<IntcMask>;
impl IntcMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntcMask {
        match self.bits {
            false => IntcMask::B0,
            true => IntcMask::B1,
        }
    }
    #[doc = "interrupt enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IntcMask::B0
    }
    #[doc = "interrupt mask"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IntcMask::B1
    }
}
#[doc = "Field `INTC_MASK` writer - INTC interrupt mask"]
pub type IntcMaskW<'a, REG> = crate::BitWriter<'a, REG, IntcMask>;
impl<'a, REG> IntcMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "interrupt enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IntcMask::B0)
    }
    #[doc = "interrupt mask"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IntcMask::B1)
    }
}
#[doc = "INTD interrupt mask\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntdMask {
    #[doc = "0: interrupt enable"]
    B0 = 0,
    #[doc = "1: interrupt mask"]
    B1 = 1,
}
impl From<IntdMask> for bool {
    #[inline(always)]
    fn from(variant: IntdMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTD_MASK` reader - INTD interrupt mask"]
pub type IntdMaskR = crate::BitReader<IntdMask>;
impl IntdMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntdMask {
        match self.bits {
            false => IntdMask::B0,
            true => IntdMask::B1,
        }
    }
    #[doc = "interrupt enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IntdMask::B0
    }
    #[doc = "interrupt mask"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IntdMask::B1
    }
}
#[doc = "Field `INTD_MASK` writer - INTD interrupt mask"]
pub type IntdMaskW<'a, REG> = crate::BitWriter<'a, REG, IntdMask>;
impl<'a, REG> IntdMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "interrupt enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IntdMask::B0)
    }
    #[doc = "interrupt mask"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IntdMask::B1)
    }
}
#[doc = "Correctable error interrupt mask\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CorrErrIntMask {
    #[doc = "0: interrupt enable"]
    B0 = 0,
    #[doc = "1: interrupt mask"]
    B1 = 1,
}
impl From<CorrErrIntMask> for bool {
    #[inline(always)]
    fn from(variant: CorrErrIntMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CORR_ERR_INT_MASK` reader - Correctable error interrupt mask"]
pub type CorrErrIntMaskR = crate::BitReader<CorrErrIntMask>;
impl CorrErrIntMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CorrErrIntMask {
        match self.bits {
            false => CorrErrIntMask::B0,
            true => CorrErrIntMask::B1,
        }
    }
    #[doc = "interrupt enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == CorrErrIntMask::B0
    }
    #[doc = "interrupt mask"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == CorrErrIntMask::B1
    }
}
#[doc = "Field `CORR_ERR_INT_MASK` writer - Correctable error interrupt mask"]
pub type CorrErrIntMaskW<'a, REG> = crate::BitWriter<'a, REG, CorrErrIntMask>;
impl<'a, REG> CorrErrIntMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "interrupt enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(CorrErrIntMask::B0)
    }
    #[doc = "interrupt mask"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(CorrErrIntMask::B1)
    }
}
#[doc = "Non-fatal error interrupt mask\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NfatalErrIntMask {
    #[doc = "0: interrupt enable"]
    B0 = 0,
    #[doc = "1: interrupt mask"]
    B1 = 1,
}
impl From<NfatalErrIntMask> for bool {
    #[inline(always)]
    fn from(variant: NfatalErrIntMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NFATAL_ERR_INT_MASK` reader - Non-fatal error interrupt mask"]
pub type NfatalErrIntMaskR = crate::BitReader<NfatalErrIntMask>;
impl NfatalErrIntMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NfatalErrIntMask {
        match self.bits {
            false => NfatalErrIntMask::B0,
            true => NfatalErrIntMask::B1,
        }
    }
    #[doc = "interrupt enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == NfatalErrIntMask::B0
    }
    #[doc = "interrupt mask"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == NfatalErrIntMask::B1
    }
}
#[doc = "Field `NFATAL_ERR_INT_MASK` writer - Non-fatal error interrupt mask"]
pub type NfatalErrIntMaskW<'a, REG> = crate::BitWriter<'a, REG, NfatalErrIntMask>;
impl<'a, REG> NfatalErrIntMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "interrupt enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(NfatalErrIntMask::B0)
    }
    #[doc = "interrupt mask"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(NfatalErrIntMask::B1)
    }
}
#[doc = "Fatal error interrupt mask\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FatalErrIntMask {
    #[doc = "0: interrupt enable"]
    B0 = 0,
    #[doc = "1: interrupt mask"]
    B1 = 1,
}
impl From<FatalErrIntMask> for bool {
    #[inline(always)]
    fn from(variant: FatalErrIntMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FATAL_ERR_INT_MASK` reader - Fatal error interrupt mask"]
pub type FatalErrIntMaskR = crate::BitReader<FatalErrIntMask>;
impl FatalErrIntMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FatalErrIntMask {
        match self.bits {
            false => FatalErrIntMask::B0,
            true => FatalErrIntMask::B1,
        }
    }
    #[doc = "interrupt enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == FatalErrIntMask::B0
    }
    #[doc = "interrupt mask"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == FatalErrIntMask::B1
    }
}
#[doc = "Field `FATAL_ERR_INT_MASK` writer - Fatal error interrupt mask"]
pub type FatalErrIntMaskW<'a, REG> = crate::BitWriter<'a, REG, FatalErrIntMask>;
impl<'a, REG> FatalErrIntMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "interrupt enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(FatalErrIntMask::B0)
    }
    #[doc = "interrupt mask"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(FatalErrIntMask::B1)
    }
}
#[doc = "DPA interrupt mask\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DpaIntMask {
    #[doc = "0: interrupt enable"]
    B0 = 0,
    #[doc = "1: interrupt mask"]
    B1 = 1,
}
impl From<DpaIntMask> for bool {
    #[inline(always)]
    fn from(variant: DpaIntMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DPA_INT_MASK` reader - DPA interrupt mask"]
pub type DpaIntMaskR = crate::BitReader<DpaIntMask>;
impl DpaIntMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DpaIntMask {
        match self.bits {
            false => DpaIntMask::B0,
            true => DpaIntMask::B1,
        }
    }
    #[doc = "interrupt enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DpaIntMask::B0
    }
    #[doc = "interrupt mask"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DpaIntMask::B1
    }
}
#[doc = "Field `DPA_INT_MASK` writer - DPA interrupt mask"]
pub type DpaIntMaskW<'a, REG> = crate::BitWriter<'a, REG, DpaIntMask>;
impl<'a, REG> DpaIntMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "interrupt enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(DpaIntMask::B0)
    }
    #[doc = "interrupt mask"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(DpaIntMask::B1)
    }
}
#[doc = "Hot reset interrupt mask\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HotResetIntMask {
    #[doc = "0: interrupt enable"]
    B0 = 0,
    #[doc = "1: interrupt mask"]
    B1 = 1,
}
impl From<HotResetIntMask> for bool {
    #[inline(always)]
    fn from(variant: HotResetIntMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HOT_RESET_INT_MASK` reader - Hot reset interrupt mask"]
pub type HotResetIntMaskR = crate::BitReader<HotResetIntMask>;
impl HotResetIntMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HotResetIntMask {
        match self.bits {
            false => HotResetIntMask::B0,
            true => HotResetIntMask::B1,
        }
    }
    #[doc = "interrupt enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == HotResetIntMask::B0
    }
    #[doc = "interrupt mask"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == HotResetIntMask::B1
    }
}
#[doc = "Field `HOT_RESET_INT_MASK` writer - Hot reset interrupt mask"]
pub type HotResetIntMaskW<'a, REG> = crate::BitWriter<'a, REG, HotResetIntMask>;
impl<'a, REG> HotResetIntMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "interrupt enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(HotResetIntMask::B0)
    }
    #[doc = "interrupt mask"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(HotResetIntMask::B1)
    }
}
#[doc = "Message receive done interrupt mask\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MsgIntMask {
    #[doc = "0: interrupt enable"]
    B0 = 0,
    #[doc = "1: interrupt mask"]
    B1 = 1,
}
impl From<MsgIntMask> for bool {
    #[inline(always)]
    fn from(variant: MsgIntMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSG_INT_MASK` reader - Message receive done interrupt mask"]
pub type MsgIntMaskR = crate::BitReader<MsgIntMask>;
impl MsgIntMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MsgIntMask {
        match self.bits {
            false => MsgIntMask::B0,
            true => MsgIntMask::B1,
        }
    }
    #[doc = "interrupt enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == MsgIntMask::B0
    }
    #[doc = "interrupt mask"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == MsgIntMask::B1
    }
}
#[doc = "Field `MSG_INT_MASK` writer - Message receive done interrupt mask"]
pub type MsgIntMaskW<'a, REG> = crate::BitWriter<'a, REG, MsgIntMask>;
impl<'a, REG> MsgIntMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "interrupt enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(MsgIntMask::B0)
    }
    #[doc = "interrupt mask"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(MsgIntMask::B1)
    }
}
#[doc = "Legacy interrupt send done interrupt mask\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LegacyDoneIntMask {
    #[doc = "0: interrupt enable"]
    B0 = 0,
    #[doc = "1: interrupt mask"]
    B1 = 1,
}
impl From<LegacyDoneIntMask> for bool {
    #[inline(always)]
    fn from(variant: LegacyDoneIntMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LEGACY_DONE_INT_MASK` reader - Legacy interrupt send done interrupt mask"]
pub type LegacyDoneIntMaskR = crate::BitReader<LegacyDoneIntMask>;
impl LegacyDoneIntMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LegacyDoneIntMask {
        match self.bits {
            false => LegacyDoneIntMask::B0,
            true => LegacyDoneIntMask::B1,
        }
    }
    #[doc = "interrupt enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == LegacyDoneIntMask::B0
    }
    #[doc = "interrupt mask"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == LegacyDoneIntMask::B1
    }
}
#[doc = "Field `LEGACY_DONE_INT_MASK` writer - Legacy interrupt send done interrupt mask"]
pub type LegacyDoneIntMaskW<'a, REG> = crate::BitWriter<'a, REG, LegacyDoneIntMask>;
impl<'a, REG> LegacyDoneIntMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "interrupt enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(LegacyDoneIntMask::B0)
    }
    #[doc = "interrupt mask"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(LegacyDoneIntMask::B1)
    }
}
#[doc = "Write mask\n\nFor each served bit\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum WriteMask {
    #[doc = "0: write mask"]
    B0 = 0,
    #[doc = "1: write enable"]
    B1 = 1,
}
impl From<WriteMask> for u16 {
    #[inline(always)]
    fn from(variant: WriteMask) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WriteMask {
    type Ux = u16;
}
#[doc = "Field `WRITE_MASK` writer - Write mask\n\nFor each served bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, WriteMask>;
impl<'a, REG> WriteMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "write mask"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(WriteMask::B0)
    }
    #[doc = "write enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(WriteMask::B1)
    }
}
impl R {
    #[doc = "Bit 0 - Power state change interrupt mask"]
    #[inline(always)]
    pub fn pwr_stcg_int_mask(&self) -> PwrStcgIntMaskR {
        PwrStcgIntMaskR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Hot plug interrupt mask"]
    #[inline(always)]
    pub fn hot_plug_int_mask(&self) -> HotPlugIntMaskR {
        HotPlugIntMaskR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Phy interrupt mask"]
    #[inline(always)]
    pub fn phy_int_mask(&self) -> PhyIntMaskR {
        PhyIntMaskR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - uDMA interrupt mask"]
    #[inline(always)]
    pub fn udma_int_mask(&self) -> UdmaIntMaskR {
        UdmaIntMaskR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Local interrupt mask"]
    #[inline(always)]
    pub fn local_int_mask(&self) -> LocalIntMaskR {
        LocalIntMaskR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - INTA interrupt mask"]
    #[inline(always)]
    pub fn inta_mask(&self) -> IntaMaskR {
        IntaMaskR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - INTB interrupt mask"]
    #[inline(always)]
    pub fn intb_mask(&self) -> IntbMaskR {
        IntbMaskR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - INTC interrupt mask"]
    #[inline(always)]
    pub fn intc_mask(&self) -> IntcMaskR {
        IntcMaskR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - INTD interrupt mask"]
    #[inline(always)]
    pub fn intd_mask(&self) -> IntdMaskR {
        IntdMaskR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Correctable error interrupt mask"]
    #[inline(always)]
    pub fn corr_err_int_mask(&self) -> CorrErrIntMaskR {
        CorrErrIntMaskR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Non-fatal error interrupt mask"]
    #[inline(always)]
    pub fn nfatal_err_int_mask(&self) -> NfatalErrIntMaskR {
        NfatalErrIntMaskR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Fatal error interrupt mask"]
    #[inline(always)]
    pub fn fatal_err_int_mask(&self) -> FatalErrIntMaskR {
        FatalErrIntMaskR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DPA interrupt mask"]
    #[inline(always)]
    pub fn dpa_int_mask(&self) -> DpaIntMaskR {
        DpaIntMaskR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Hot reset interrupt mask"]
    #[inline(always)]
    pub fn hot_reset_int_mask(&self) -> HotResetIntMaskR {
        HotResetIntMaskR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Message receive done interrupt mask"]
    #[inline(always)]
    pub fn msg_int_mask(&self) -> MsgIntMaskR {
        MsgIntMaskR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Legacy interrupt send done interrupt mask"]
    #[inline(always)]
    pub fn legacy_done_int_mask(&self) -> LegacyDoneIntMaskR {
        LegacyDoneIntMaskR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power state change interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn pwr_stcg_int_mask(&mut self) -> PwrStcgIntMaskW<PcieClientIntMaskSpec> {
        PwrStcgIntMaskW::new(self, 0)
    }
    #[doc = "Bit 1 - Hot plug interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn hot_plug_int_mask(&mut self) -> HotPlugIntMaskW<PcieClientIntMaskSpec> {
        HotPlugIntMaskW::new(self, 1)
    }
    #[doc = "Bit 2 - Phy interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn phy_int_mask(&mut self) -> PhyIntMaskW<PcieClientIntMaskSpec> {
        PhyIntMaskW::new(self, 2)
    }
    #[doc = "Bit 3 - uDMA interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn udma_int_mask(&mut self) -> UdmaIntMaskW<PcieClientIntMaskSpec> {
        UdmaIntMaskW::new(self, 3)
    }
    #[doc = "Bit 4 - Local interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn local_int_mask(&mut self) -> LocalIntMaskW<PcieClientIntMaskSpec> {
        LocalIntMaskW::new(self, 4)
    }
    #[doc = "Bit 5 - INTA interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn inta_mask(&mut self) -> IntaMaskW<PcieClientIntMaskSpec> {
        IntaMaskW::new(self, 5)
    }
    #[doc = "Bit 6 - INTB interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn intb_mask(&mut self) -> IntbMaskW<PcieClientIntMaskSpec> {
        IntbMaskW::new(self, 6)
    }
    #[doc = "Bit 7 - INTC interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn intc_mask(&mut self) -> IntcMaskW<PcieClientIntMaskSpec> {
        IntcMaskW::new(self, 7)
    }
    #[doc = "Bit 8 - INTD interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn intd_mask(&mut self) -> IntdMaskW<PcieClientIntMaskSpec> {
        IntdMaskW::new(self, 8)
    }
    #[doc = "Bit 9 - Correctable error interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn corr_err_int_mask(&mut self) -> CorrErrIntMaskW<PcieClientIntMaskSpec> {
        CorrErrIntMaskW::new(self, 9)
    }
    #[doc = "Bit 10 - Non-fatal error interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn nfatal_err_int_mask(&mut self) -> NfatalErrIntMaskW<PcieClientIntMaskSpec> {
        NfatalErrIntMaskW::new(self, 10)
    }
    #[doc = "Bit 11 - Fatal error interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn fatal_err_int_mask(&mut self) -> FatalErrIntMaskW<PcieClientIntMaskSpec> {
        FatalErrIntMaskW::new(self, 11)
    }
    #[doc = "Bit 12 - DPA interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn dpa_int_mask(&mut self) -> DpaIntMaskW<PcieClientIntMaskSpec> {
        DpaIntMaskW::new(self, 12)
    }
    #[doc = "Bit 13 - Hot reset interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn hot_reset_int_mask(&mut self) -> HotResetIntMaskW<PcieClientIntMaskSpec> {
        HotResetIntMaskW::new(self, 13)
    }
    #[doc = "Bit 14 - Message receive done interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn msg_int_mask(&mut self) -> MsgIntMaskW<PcieClientIntMaskSpec> {
        MsgIntMaskW::new(self, 14)
    }
    #[doc = "Bit 15 - Legacy interrupt send done interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn legacy_done_int_mask(&mut self) -> LegacyDoneIntMaskW<PcieClientIntMaskSpec> {
        LegacyDoneIntMaskW::new(self, 15)
    }
    #[doc = "Bits 16:31 - Write mask\n\nFor each served bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<PcieClientIntMaskSpec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Interrupt mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_client_int_mask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_client_int_mask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieClientIntMaskSpec;
impl crate::RegisterSpec for PcieClientIntMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_client_int_mask::R`](R) reader structure"]
impl crate::Readable for PcieClientIntMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`pcie_client_int_mask::W`](W) writer structure"]
impl crate::Writable for PcieClientIntMaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCIE_CLIENT_INT_MASK to value 0xffff"]
impl crate::Resettable for PcieClientIntMaskSpec {
    const RESET_VALUE: u32 = 0xffff;
}
