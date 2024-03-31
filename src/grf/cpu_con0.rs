#[doc = "Register `CPU_CON0` reader"]
pub type R = crate::R<CpuCon0Spec>;
#[doc = "Register `CPU_CON0` writer"]
pub type W = crate::W<CpuCon0Spec>;
#[doc = "pd_core_l cpu broadcastinner bit control\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BroadcastinnerPdCoreL {
    #[doc = "1: enable"]
    B1 = 1,
    #[doc = "0: disable"]
    B0 = 0,
}
impl From<BroadcastinnerPdCoreL> for bool {
    #[inline(always)]
    fn from(variant: BroadcastinnerPdCoreL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BROADCASTINNER_PD_CORE_L` reader - pd_core_l cpu broadcastinner bit control"]
pub type BroadcastinnerPdCoreLR = crate::BitReader<BroadcastinnerPdCoreL>;
impl BroadcastinnerPdCoreLR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BroadcastinnerPdCoreL {
        match self.bits {
            true => BroadcastinnerPdCoreL::B1,
            false => BroadcastinnerPdCoreL::B0,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == BroadcastinnerPdCoreL::B1
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == BroadcastinnerPdCoreL::B0
    }
}
#[doc = "Field `BROADCASTINNER_PD_CORE_L` writer - pd_core_l cpu broadcastinner bit control"]
pub type BroadcastinnerPdCoreLW<'a, REG> = crate::BitWriter<'a, REG, BroadcastinnerPdCoreL>;
impl<'a, REG> BroadcastinnerPdCoreLW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(BroadcastinnerPdCoreL::B1)
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(BroadcastinnerPdCoreL::B0)
    }
}
#[doc = "pd_core_l cpu broadcastouter bit control\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BroadcastouterPdCoreL {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<BroadcastouterPdCoreL> for bool {
    #[inline(always)]
    fn from(variant: BroadcastouterPdCoreL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BROADCASTOUTER_PD_CORE_L` reader - pd_core_l cpu broadcastouter bit control"]
pub type BroadcastouterPdCoreLR = crate::BitReader<BroadcastouterPdCoreL>;
impl BroadcastouterPdCoreLR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BroadcastouterPdCoreL {
        match self.bits {
            false => BroadcastouterPdCoreL::B0,
            true => BroadcastouterPdCoreL::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == BroadcastouterPdCoreL::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == BroadcastouterPdCoreL::B1
    }
}
#[doc = "Field `BROADCASTOUTER_PD_CORE_L` writer - pd_core_l cpu broadcastouter bit control"]
pub type BroadcastouterPdCoreLW<'a, REG> = crate::BitWriter<'a, REG, BroadcastouterPdCoreL>;
impl<'a, REG> BroadcastouterPdCoreLW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(BroadcastouterPdCoreL::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(BroadcastouterPdCoreL::B1)
    }
}
#[doc = "pd_core_l cpu broadcastcachemaint bit\n\ncontrol\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BroadcastcachemaintPdCoreL {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<BroadcastcachemaintPdCoreL> for bool {
    #[inline(always)]
    fn from(variant: BroadcastcachemaintPdCoreL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BROADCASTCACHEMAINT_PD_CORE_L` reader - pd_core_l cpu broadcastcachemaint bit\n\ncontrol"]
pub type BroadcastcachemaintPdCoreLR = crate::BitReader<BroadcastcachemaintPdCoreL>;
impl BroadcastcachemaintPdCoreLR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BroadcastcachemaintPdCoreL {
        match self.bits {
            false => BroadcastcachemaintPdCoreL::B0,
            true => BroadcastcachemaintPdCoreL::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == BroadcastcachemaintPdCoreL::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == BroadcastcachemaintPdCoreL::B1
    }
}
#[doc = "Field `BROADCASTCACHEMAINT_PD_CORE_L` writer - pd_core_l cpu broadcastcachemaint bit\n\ncontrol"]
pub type BroadcastcachemaintPdCoreLW<'a, REG> =
    crate::BitWriter<'a, REG, BroadcastcachemaintPdCoreL>;
impl<'a, REG> BroadcastcachemaintPdCoreLW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(BroadcastcachemaintPdCoreL::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(BroadcastcachemaintPdCoreL::B1)
    }
}
#[doc = "pd_core_l cpu sysbardisable bit control\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SysbardisablePdCoreL {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<SysbardisablePdCoreL> for bool {
    #[inline(always)]
    fn from(variant: SysbardisablePdCoreL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSBARDISABLE_PD_CORE_L` reader - pd_core_l cpu sysbardisable bit control"]
pub type SysbardisablePdCoreLR = crate::BitReader<SysbardisablePdCoreL>;
impl SysbardisablePdCoreLR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SysbardisablePdCoreL {
        match self.bits {
            false => SysbardisablePdCoreL::B0,
            true => SysbardisablePdCoreL::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SysbardisablePdCoreL::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SysbardisablePdCoreL::B1
    }
}
#[doc = "Field `SYSBARDISABLE_PD_CORE_L` writer - pd_core_l cpu sysbardisable bit control"]
pub type SysbardisablePdCoreLW<'a, REG> = crate::BitWriter<'a, REG, SysbardisablePdCoreL>;
impl<'a, REG> SysbardisablePdCoreLW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SysbardisablePdCoreL::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SysbardisablePdCoreL::B1)
    }
}
#[doc = "pd_core_l cpu clrexmonreq bit control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClrexmonreqPdCoreL {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<ClrexmonreqPdCoreL> for bool {
    #[inline(always)]
    fn from(variant: ClrexmonreqPdCoreL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLREXMONREQ_PD_CORE_L` reader - pd_core_l cpu clrexmonreq bit control"]
pub type ClrexmonreqPdCoreLR = crate::BitReader<ClrexmonreqPdCoreL>;
impl ClrexmonreqPdCoreLR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClrexmonreqPdCoreL {
        match self.bits {
            false => ClrexmonreqPdCoreL::B0,
            true => ClrexmonreqPdCoreL::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ClrexmonreqPdCoreL::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ClrexmonreqPdCoreL::B1
    }
}
#[doc = "Field `CLREXMONREQ_PD_CORE_L` writer - pd_core_l cpu clrexmonreq bit control"]
pub type ClrexmonreqPdCoreLW<'a, REG> = crate::BitWriter<'a, REG, ClrexmonreqPdCoreL>;
impl<'a, REG> ClrexmonreqPdCoreLW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ClrexmonreqPdCoreL::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ClrexmonreqPdCoreL::B1)
    }
}
#[doc = "pd_core_l cpu dbgl1rstdisable bit control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dbgl1rstdisablePdCoreL {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Dbgl1rstdisablePdCoreL> for bool {
    #[inline(always)]
    fn from(variant: Dbgl1rstdisablePdCoreL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBGL1RSTDISABLE_PD_CORE_L` reader - pd_core_l cpu dbgl1rstdisable bit control"]
pub type Dbgl1rstdisablePdCoreLR = crate::BitReader<Dbgl1rstdisablePdCoreL>;
impl Dbgl1rstdisablePdCoreLR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dbgl1rstdisablePdCoreL {
        match self.bits {
            false => Dbgl1rstdisablePdCoreL::B0,
            true => Dbgl1rstdisablePdCoreL::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Dbgl1rstdisablePdCoreL::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Dbgl1rstdisablePdCoreL::B1
    }
}
#[doc = "Field `DBGL1RSTDISABLE_PD_CORE_L` writer - pd_core_l cpu dbgl1rstdisable bit control"]
pub type Dbgl1rstdisablePdCoreLW<'a, REG> = crate::BitWriter<'a, REG, Dbgl1rstdisablePdCoreL>;
impl<'a, REG> Dbgl1rstdisablePdCoreLW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Dbgl1rstdisablePdCoreL::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Dbgl1rstdisablePdCoreL::B1)
    }
}
#[doc = "pd_core_l cpu l2rstdisable bit control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L2rstdisablePdCoreL {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<L2rstdisablePdCoreL> for bool {
    #[inline(always)]
    fn from(variant: L2rstdisablePdCoreL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `L2RSTDISABLE_PD_CORE_L` reader - pd_core_l cpu l2rstdisable bit control"]
pub type L2rstdisablePdCoreLR = crate::BitReader<L2rstdisablePdCoreL>;
impl L2rstdisablePdCoreLR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> L2rstdisablePdCoreL {
        match self.bits {
            false => L2rstdisablePdCoreL::B0,
            true => L2rstdisablePdCoreL::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == L2rstdisablePdCoreL::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == L2rstdisablePdCoreL::B1
    }
}
#[doc = "Field `L2RSTDISABLE_PD_CORE_L` writer - pd_core_l cpu l2rstdisable bit control"]
pub type L2rstdisablePdCoreLW<'a, REG> = crate::BitWriter<'a, REG, L2rstdisablePdCoreL>;
impl<'a, REG> L2rstdisablePdCoreLW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(L2rstdisablePdCoreL::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(L2rstdisablePdCoreL::B1)
    }
}
#[doc = "pd_core_l cpu cfgend bit control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CfgendPdCoreL {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<CfgendPdCoreL> for u8 {
    #[inline(always)]
    fn from(variant: CfgendPdCoreL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CfgendPdCoreL {
    type Ux = u8;
}
#[doc = "Field `CFGEND_PD_CORE_L` reader - pd_core_l cpu cfgend bit control"]
pub type CfgendPdCoreLR = crate::FieldReader<CfgendPdCoreL>;
impl CfgendPdCoreLR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CfgendPdCoreL> {
        match self.bits {
            0 => Some(CfgendPdCoreL::B0),
            1 => Some(CfgendPdCoreL::B1),
            _ => None,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == CfgendPdCoreL::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == CfgendPdCoreL::B1
    }
}
#[doc = "Field `CFGEND_PD_CORE_L` writer - pd_core_l cpu cfgend bit control"]
pub type CfgendPdCoreLW<'a, REG> = crate::FieldWriter<'a, REG, 4, CfgendPdCoreL>;
impl<'a, REG> CfgendPdCoreLW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(CfgendPdCoreL::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(CfgendPdCoreL::B1)
    }
}
#[doc = "pd_core_l cpu cfgte bit control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CfgtePdCoreL {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<CfgtePdCoreL> for u8 {
    #[inline(always)]
    fn from(variant: CfgtePdCoreL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CfgtePdCoreL {
    type Ux = u8;
}
#[doc = "Field `CFGTE_PD_CORE_L` reader - pd_core_l cpu cfgte bit control"]
pub type CfgtePdCoreLR = crate::FieldReader<CfgtePdCoreL>;
impl CfgtePdCoreLR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CfgtePdCoreL> {
        match self.bits {
            0 => Some(CfgtePdCoreL::B0),
            1 => Some(CfgtePdCoreL::B1),
            _ => None,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == CfgtePdCoreL::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == CfgtePdCoreL::B1
    }
}
#[doc = "Field `CFGTE_PD_CORE_L` writer - pd_core_l cpu cfgte bit control"]
pub type CfgtePdCoreLW<'a, REG> = crate::FieldWriter<'a, REG, 4, CfgtePdCoreL>;
impl<'a, REG> CfgtePdCoreLW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(CfgtePdCoreL::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(CfgtePdCoreL::B1)
    }
}
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - pd_core_l cpu broadcastinner bit control"]
    #[inline(always)]
    pub fn broadcastinner_pd_core_l(&self) -> BroadcastinnerPdCoreLR {
        BroadcastinnerPdCoreLR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - pd_core_l cpu broadcastouter bit control"]
    #[inline(always)]
    pub fn broadcastouter_pd_core_l(&self) -> BroadcastouterPdCoreLR {
        BroadcastouterPdCoreLR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - pd_core_l cpu broadcastcachemaint bit\n\ncontrol"]
    #[inline(always)]
    pub fn broadcastcachemaint_pd_core_l(&self) -> BroadcastcachemaintPdCoreLR {
        BroadcastcachemaintPdCoreLR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - pd_core_l cpu sysbardisable bit control"]
    #[inline(always)]
    pub fn sysbardisable_pd_core_l(&self) -> SysbardisablePdCoreLR {
        SysbardisablePdCoreLR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - pd_core_l cpu clrexmonreq bit control"]
    #[inline(always)]
    pub fn clrexmonreq_pd_core_l(&self) -> ClrexmonreqPdCoreLR {
        ClrexmonreqPdCoreLR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - pd_core_l cpu dbgl1rstdisable bit control"]
    #[inline(always)]
    pub fn dbgl1rstdisable_pd_core_l(&self) -> Dbgl1rstdisablePdCoreLR {
        Dbgl1rstdisablePdCoreLR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - pd_core_l cpu l2rstdisable bit control"]
    #[inline(always)]
    pub fn l2rstdisable_pd_core_l(&self) -> L2rstdisablePdCoreLR {
        L2rstdisablePdCoreLR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - pd_core_l cpu cfgend bit control"]
    #[inline(always)]
    pub fn cfgend_pd_core_l(&self) -> CfgendPdCoreLR {
        CfgendPdCoreLR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - pd_core_l cpu cfgte bit control"]
    #[inline(always)]
    pub fn cfgte_pd_core_l(&self) -> CfgtePdCoreLR {
        CfgtePdCoreLR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - pd_core_l cpu broadcastinner bit control"]
    #[inline(always)]
    #[must_use]
    pub fn broadcastinner_pd_core_l(&mut self) -> BroadcastinnerPdCoreLW<CpuCon0Spec> {
        BroadcastinnerPdCoreLW::new(self, 0)
    }
    #[doc = "Bit 1 - pd_core_l cpu broadcastouter bit control"]
    #[inline(always)]
    #[must_use]
    pub fn broadcastouter_pd_core_l(&mut self) -> BroadcastouterPdCoreLW<CpuCon0Spec> {
        BroadcastouterPdCoreLW::new(self, 1)
    }
    #[doc = "Bit 2 - pd_core_l cpu broadcastcachemaint bit\n\ncontrol"]
    #[inline(always)]
    #[must_use]
    pub fn broadcastcachemaint_pd_core_l(&mut self) -> BroadcastcachemaintPdCoreLW<CpuCon0Spec> {
        BroadcastcachemaintPdCoreLW::new(self, 2)
    }
    #[doc = "Bit 3 - pd_core_l cpu sysbardisable bit control"]
    #[inline(always)]
    #[must_use]
    pub fn sysbardisable_pd_core_l(&mut self) -> SysbardisablePdCoreLW<CpuCon0Spec> {
        SysbardisablePdCoreLW::new(self, 3)
    }
    #[doc = "Bit 4 - pd_core_l cpu clrexmonreq bit control"]
    #[inline(always)]
    #[must_use]
    pub fn clrexmonreq_pd_core_l(&mut self) -> ClrexmonreqPdCoreLW<CpuCon0Spec> {
        ClrexmonreqPdCoreLW::new(self, 4)
    }
    #[doc = "Bit 6 - pd_core_l cpu dbgl1rstdisable bit control"]
    #[inline(always)]
    #[must_use]
    pub fn dbgl1rstdisable_pd_core_l(&mut self) -> Dbgl1rstdisablePdCoreLW<CpuCon0Spec> {
        Dbgl1rstdisablePdCoreLW::new(self, 6)
    }
    #[doc = "Bit 7 - pd_core_l cpu l2rstdisable bit control"]
    #[inline(always)]
    #[must_use]
    pub fn l2rstdisable_pd_core_l(&mut self) -> L2rstdisablePdCoreLW<CpuCon0Spec> {
        L2rstdisablePdCoreLW::new(self, 7)
    }
    #[doc = "Bits 8:11 - pd_core_l cpu cfgend bit control"]
    #[inline(always)]
    #[must_use]
    pub fn cfgend_pd_core_l(&mut self) -> CfgendPdCoreLW<CpuCon0Spec> {
        CfgendPdCoreLW::new(self, 8)
    }
    #[doc = "Bits 12:15 - pd_core_l cpu cfgte bit control"]
    #[inline(always)]
    #[must_use]
    pub fn cfgte_pd_core_l(&mut self) -> CfgtePdCoreLW<CpuCon0Spec> {
        CfgtePdCoreLW::new(self, 12)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<CpuCon0Spec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "cpu control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_con0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_con0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpuCon0Spec;
impl crate::RegisterSpec for CpuCon0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu_con0::R`](R) reader structure"]
impl crate::Readable for CpuCon0Spec {}
#[doc = "`write(|w| ..)` method takes [`cpu_con0::W`](W) writer structure"]
impl crate::Writable for CpuCon0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPU_CON0 to value 0x0b"]
impl crate::Resettable for CpuCon0Spec {
    const RESET_VALUE: u32 = 0x0b;
}
