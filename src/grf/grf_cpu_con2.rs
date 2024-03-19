#[doc = "Register `GRF_CPU_CON2` reader"]
pub type R = crate::R<GrfCpuCon2Spec>;
#[doc = "Register `GRF_CPU_CON2` writer"]
pub type W = crate::W<GrfCpuCon2Spec>;
#[doc = "pd_core_b cpu broadcastinner bit control\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BroadcastinnerPdCoreB {
    #[doc = "1: enable"]
    B1 = 1,
    #[doc = "0: disable"]
    B0 = 0,
}
impl From<BroadcastinnerPdCoreB> for bool {
    #[inline(always)]
    fn from(variant: BroadcastinnerPdCoreB) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BROADCASTINNER_PD_CORE_B` reader - pd_core_b cpu broadcastinner bit control"]
pub type BroadcastinnerPdCoreBR = crate::BitReader<BroadcastinnerPdCoreB>;
impl BroadcastinnerPdCoreBR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BroadcastinnerPdCoreB {
        match self.bits {
            true => BroadcastinnerPdCoreB::B1,
            false => BroadcastinnerPdCoreB::B0,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == BroadcastinnerPdCoreB::B1
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == BroadcastinnerPdCoreB::B0
    }
}
#[doc = "Field `BROADCASTINNER_PD_CORE_B` writer - pd_core_b cpu broadcastinner bit control"]
pub type BroadcastinnerPdCoreBW<'a, REG> = crate::BitWriter<'a, REG, BroadcastinnerPdCoreB>;
impl<'a, REG> BroadcastinnerPdCoreBW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(BroadcastinnerPdCoreB::B1)
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(BroadcastinnerPdCoreB::B0)
    }
}
#[doc = "pd_core_b cpu broadcastouter bit control\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BroadcastouterPdCoreB {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<BroadcastouterPdCoreB> for bool {
    #[inline(always)]
    fn from(variant: BroadcastouterPdCoreB) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BROADCASTOUTER_PD_CORE_B` reader - pd_core_b cpu broadcastouter bit control"]
pub type BroadcastouterPdCoreBR = crate::BitReader<BroadcastouterPdCoreB>;
impl BroadcastouterPdCoreBR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BroadcastouterPdCoreB {
        match self.bits {
            false => BroadcastouterPdCoreB::B0,
            true => BroadcastouterPdCoreB::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == BroadcastouterPdCoreB::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == BroadcastouterPdCoreB::B1
    }
}
#[doc = "Field `BROADCASTOUTER_PD_CORE_B` writer - pd_core_b cpu broadcastouter bit control"]
pub type BroadcastouterPdCoreBW<'a, REG> = crate::BitWriter<'a, REG, BroadcastouterPdCoreB>;
impl<'a, REG> BroadcastouterPdCoreBW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(BroadcastouterPdCoreB::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(BroadcastouterPdCoreB::B1)
    }
}
#[doc = "pd_core_b cpu broadcastcachemaint bit\n\ncontrol\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BroadcastcachemaintPdCoreB {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<BroadcastcachemaintPdCoreB> for bool {
    #[inline(always)]
    fn from(variant: BroadcastcachemaintPdCoreB) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BROADCASTCACHEMAINT_PD_CORE_B` reader - pd_core_b cpu broadcastcachemaint bit\n\ncontrol"]
pub type BroadcastcachemaintPdCoreBR = crate::BitReader<BroadcastcachemaintPdCoreB>;
impl BroadcastcachemaintPdCoreBR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BroadcastcachemaintPdCoreB {
        match self.bits {
            false => BroadcastcachemaintPdCoreB::B0,
            true => BroadcastcachemaintPdCoreB::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == BroadcastcachemaintPdCoreB::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == BroadcastcachemaintPdCoreB::B1
    }
}
#[doc = "Field `BROADCASTCACHEMAINT_PD_CORE_B` writer - pd_core_b cpu broadcastcachemaint bit\n\ncontrol"]
pub type BroadcastcachemaintPdCoreBW<'a, REG> =
    crate::BitWriter<'a, REG, BroadcastcachemaintPdCoreB>;
impl<'a, REG> BroadcastcachemaintPdCoreBW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(BroadcastcachemaintPdCoreB::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(BroadcastcachemaintPdCoreB::B1)
    }
}
#[doc = "pd_core_b cpu sysbardisable bit control\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SysbardisablePdCoreB {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<SysbardisablePdCoreB> for bool {
    #[inline(always)]
    fn from(variant: SysbardisablePdCoreB) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSBARDISABLE_PD_CORE_B` reader - pd_core_b cpu sysbardisable bit control"]
pub type SysbardisablePdCoreBR = crate::BitReader<SysbardisablePdCoreB>;
impl SysbardisablePdCoreBR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SysbardisablePdCoreB {
        match self.bits {
            false => SysbardisablePdCoreB::B0,
            true => SysbardisablePdCoreB::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SysbardisablePdCoreB::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SysbardisablePdCoreB::B1
    }
}
#[doc = "Field `SYSBARDISABLE_PD_CORE_B` writer - pd_core_b cpu sysbardisable bit control"]
pub type SysbardisablePdCoreBW<'a, REG> = crate::BitWriter<'a, REG, SysbardisablePdCoreB>;
impl<'a, REG> SysbardisablePdCoreBW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SysbardisablePdCoreB::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SysbardisablePdCoreB::B1)
    }
}
#[doc = "pd_core_b cpu clrexmonreq bit control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClrexmonreqPdCoreB {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<ClrexmonreqPdCoreB> for bool {
    #[inline(always)]
    fn from(variant: ClrexmonreqPdCoreB) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLREXMONREQ_PD_CORE_B` reader - pd_core_b cpu clrexmonreq bit control"]
pub type ClrexmonreqPdCoreBR = crate::BitReader<ClrexmonreqPdCoreB>;
impl ClrexmonreqPdCoreBR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClrexmonreqPdCoreB {
        match self.bits {
            false => ClrexmonreqPdCoreB::B0,
            true => ClrexmonreqPdCoreB::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ClrexmonreqPdCoreB::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ClrexmonreqPdCoreB::B1
    }
}
#[doc = "Field `CLREXMONREQ_PD_CORE_B` writer - pd_core_b cpu clrexmonreq bit control"]
pub type ClrexmonreqPdCoreBW<'a, REG> = crate::BitWriter<'a, REG, ClrexmonreqPdCoreB>;
impl<'a, REG> ClrexmonreqPdCoreBW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ClrexmonreqPdCoreB::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ClrexmonreqPdCoreB::B1)
    }
}
#[doc = "pd_core_b cpu dbgl1rstdisable bit control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dbgl1rstdisablePdCoreB {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Dbgl1rstdisablePdCoreB> for bool {
    #[inline(always)]
    fn from(variant: Dbgl1rstdisablePdCoreB) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBGL1RSTDISABLE_PD_CORE_B` reader - pd_core_b cpu dbgl1rstdisable bit control"]
pub type Dbgl1rstdisablePdCoreBR = crate::BitReader<Dbgl1rstdisablePdCoreB>;
impl Dbgl1rstdisablePdCoreBR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dbgl1rstdisablePdCoreB {
        match self.bits {
            false => Dbgl1rstdisablePdCoreB::B0,
            true => Dbgl1rstdisablePdCoreB::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Dbgl1rstdisablePdCoreB::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Dbgl1rstdisablePdCoreB::B1
    }
}
#[doc = "Field `DBGL1RSTDISABLE_PD_CORE_B` writer - pd_core_b cpu dbgl1rstdisable bit control"]
pub type Dbgl1rstdisablePdCoreBW<'a, REG> = crate::BitWriter<'a, REG, Dbgl1rstdisablePdCoreB>;
impl<'a, REG> Dbgl1rstdisablePdCoreBW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Dbgl1rstdisablePdCoreB::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Dbgl1rstdisablePdCoreB::B1)
    }
}
#[doc = "pd_core_b cpu l2rstdisable bit control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L2rstdisablePdCoreB {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<L2rstdisablePdCoreB> for bool {
    #[inline(always)]
    fn from(variant: L2rstdisablePdCoreB) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `L2RSTDISABLE_PD_CORE_B` reader - pd_core_b cpu l2rstdisable bit control"]
pub type L2rstdisablePdCoreBR = crate::BitReader<L2rstdisablePdCoreB>;
impl L2rstdisablePdCoreBR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> L2rstdisablePdCoreB {
        match self.bits {
            false => L2rstdisablePdCoreB::B0,
            true => L2rstdisablePdCoreB::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == L2rstdisablePdCoreB::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == L2rstdisablePdCoreB::B1
    }
}
#[doc = "Field `L2RSTDISABLE_PD_CORE_B` writer - pd_core_b cpu l2rstdisable bit control"]
pub type L2rstdisablePdCoreBW<'a, REG> = crate::BitWriter<'a, REG, L2rstdisablePdCoreB>;
impl<'a, REG> L2rstdisablePdCoreBW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(L2rstdisablePdCoreB::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(L2rstdisablePdCoreB::B1)
    }
}
#[doc = "pd_core_b cpu cfgend bit control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CfgendPdCoreB {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<CfgendPdCoreB> for u8 {
    #[inline(always)]
    fn from(variant: CfgendPdCoreB) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CfgendPdCoreB {
    type Ux = u8;
}
#[doc = "Field `CFGEND_PD_CORE_B` reader - pd_core_b cpu cfgend bit control"]
pub type CfgendPdCoreBR = crate::FieldReader<CfgendPdCoreB>;
impl CfgendPdCoreBR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CfgendPdCoreB> {
        match self.bits {
            0 => Some(CfgendPdCoreB::B0),
            1 => Some(CfgendPdCoreB::B1),
            _ => None,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == CfgendPdCoreB::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == CfgendPdCoreB::B1
    }
}
#[doc = "Field `CFGEND_PD_CORE_B` writer - pd_core_b cpu cfgend bit control"]
pub type CfgendPdCoreBW<'a, REG> = crate::FieldWriter<'a, REG, 2, CfgendPdCoreB>;
impl<'a, REG> CfgendPdCoreBW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(CfgendPdCoreB::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(CfgendPdCoreB::B1)
    }
}
#[doc = "pd_core_b cpu cfgte bit control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CfgtePdCoreB {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<CfgtePdCoreB> for u8 {
    #[inline(always)]
    fn from(variant: CfgtePdCoreB) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CfgtePdCoreB {
    type Ux = u8;
}
#[doc = "Field `CFGTE_PD_CORE_B` reader - pd_core_b cpu cfgte bit control"]
pub type CfgtePdCoreBR = crate::FieldReader<CfgtePdCoreB>;
impl CfgtePdCoreBR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CfgtePdCoreB> {
        match self.bits {
            0 => Some(CfgtePdCoreB::B0),
            1 => Some(CfgtePdCoreB::B1),
            _ => None,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == CfgtePdCoreB::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == CfgtePdCoreB::B1
    }
}
#[doc = "Field `CFGTE_PD_CORE_B` writer - pd_core_b cpu cfgte bit control"]
pub type CfgtePdCoreBW<'a, REG> = crate::FieldWriter<'a, REG, 2, CfgtePdCoreB>;
impl<'a, REG> CfgtePdCoreBW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(CfgtePdCoreB::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(CfgtePdCoreB::B1)
    }
}
#[doc = "gic axi master error acknowledges\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GicAximErrAck {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<GicAximErrAck> for bool {
    #[inline(always)]
    fn from(variant: GicAximErrAck) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GIC_AXIM_ERR_ACK` reader - gic axi master error acknowledges"]
pub type GicAximErrAckR = crate::BitReader<GicAximErrAck>;
impl GicAximErrAckR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GicAximErrAck {
        match self.bits {
            false => GicAximErrAck::B0,
            true => GicAximErrAck::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == GicAximErrAck::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == GicAximErrAck::B1
    }
}
#[doc = "Field `GIC_AXIM_ERR_ACK` writer - gic axi master error acknowledges"]
pub type GicAximErrAckW<'a, REG> = crate::BitWriter<'a, REG, GicAximErrAck>;
impl<'a, REG> GicAximErrAckW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(GicAximErrAck::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(GicAximErrAck::B1)
    }
}
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - pd_core_b cpu broadcastinner bit control"]
    #[inline(always)]
    pub fn broadcastinner_pd_core_b(&self) -> BroadcastinnerPdCoreBR {
        BroadcastinnerPdCoreBR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - pd_core_b cpu broadcastouter bit control"]
    #[inline(always)]
    pub fn broadcastouter_pd_core_b(&self) -> BroadcastouterPdCoreBR {
        BroadcastouterPdCoreBR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - pd_core_b cpu broadcastcachemaint bit\n\ncontrol"]
    #[inline(always)]
    pub fn broadcastcachemaint_pd_core_b(&self) -> BroadcastcachemaintPdCoreBR {
        BroadcastcachemaintPdCoreBR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - pd_core_b cpu sysbardisable bit control"]
    #[inline(always)]
    pub fn sysbardisable_pd_core_b(&self) -> SysbardisablePdCoreBR {
        SysbardisablePdCoreBR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - pd_core_b cpu clrexmonreq bit control"]
    #[inline(always)]
    pub fn clrexmonreq_pd_core_b(&self) -> ClrexmonreqPdCoreBR {
        ClrexmonreqPdCoreBR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - pd_core_b cpu dbgl1rstdisable bit control"]
    #[inline(always)]
    pub fn dbgl1rstdisable_pd_core_b(&self) -> Dbgl1rstdisablePdCoreBR {
        Dbgl1rstdisablePdCoreBR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - pd_core_b cpu l2rstdisable bit control"]
    #[inline(always)]
    pub fn l2rstdisable_pd_core_b(&self) -> L2rstdisablePdCoreBR {
        L2rstdisablePdCoreBR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - pd_core_b cpu cfgend bit control"]
    #[inline(always)]
    pub fn cfgend_pd_core_b(&self) -> CfgendPdCoreBR {
        CfgendPdCoreBR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - pd_core_b cpu cfgte bit control"]
    #[inline(always)]
    pub fn cfgte_pd_core_b(&self) -> CfgtePdCoreBR {
        CfgtePdCoreBR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - gic axi master error acknowledges"]
    #[inline(always)]
    pub fn gic_axim_err_ack(&self) -> GicAximErrAckR {
        GicAximErrAckR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - pd_core_b cpu broadcastinner bit control"]
    #[inline(always)]
    #[must_use]
    pub fn broadcastinner_pd_core_b(&mut self) -> BroadcastinnerPdCoreBW<GrfCpuCon2Spec> {
        BroadcastinnerPdCoreBW::new(self, 0)
    }
    #[doc = "Bit 1 - pd_core_b cpu broadcastouter bit control"]
    #[inline(always)]
    #[must_use]
    pub fn broadcastouter_pd_core_b(&mut self) -> BroadcastouterPdCoreBW<GrfCpuCon2Spec> {
        BroadcastouterPdCoreBW::new(self, 1)
    }
    #[doc = "Bit 2 - pd_core_b cpu broadcastcachemaint bit\n\ncontrol"]
    #[inline(always)]
    #[must_use]
    pub fn broadcastcachemaint_pd_core_b(&mut self) -> BroadcastcachemaintPdCoreBW<GrfCpuCon2Spec> {
        BroadcastcachemaintPdCoreBW::new(self, 2)
    }
    #[doc = "Bit 3 - pd_core_b cpu sysbardisable bit control"]
    #[inline(always)]
    #[must_use]
    pub fn sysbardisable_pd_core_b(&mut self) -> SysbardisablePdCoreBW<GrfCpuCon2Spec> {
        SysbardisablePdCoreBW::new(self, 3)
    }
    #[doc = "Bit 4 - pd_core_b cpu clrexmonreq bit control"]
    #[inline(always)]
    #[must_use]
    pub fn clrexmonreq_pd_core_b(&mut self) -> ClrexmonreqPdCoreBW<GrfCpuCon2Spec> {
        ClrexmonreqPdCoreBW::new(self, 4)
    }
    #[doc = "Bit 6 - pd_core_b cpu dbgl1rstdisable bit control"]
    #[inline(always)]
    #[must_use]
    pub fn dbgl1rstdisable_pd_core_b(&mut self) -> Dbgl1rstdisablePdCoreBW<GrfCpuCon2Spec> {
        Dbgl1rstdisablePdCoreBW::new(self, 6)
    }
    #[doc = "Bit 7 - pd_core_b cpu l2rstdisable bit control"]
    #[inline(always)]
    #[must_use]
    pub fn l2rstdisable_pd_core_b(&mut self) -> L2rstdisablePdCoreBW<GrfCpuCon2Spec> {
        L2rstdisablePdCoreBW::new(self, 7)
    }
    #[doc = "Bits 8:9 - pd_core_b cpu cfgend bit control"]
    #[inline(always)]
    #[must_use]
    pub fn cfgend_pd_core_b(&mut self) -> CfgendPdCoreBW<GrfCpuCon2Spec> {
        CfgendPdCoreBW::new(self, 8)
    }
    #[doc = "Bits 12:13 - pd_core_b cpu cfgte bit control"]
    #[inline(always)]
    #[must_use]
    pub fn cfgte_pd_core_b(&mut self) -> CfgtePdCoreBW<GrfCpuCon2Spec> {
        CfgtePdCoreBW::new(self, 12)
    }
    #[doc = "Bit 14 - gic axi master error acknowledges"]
    #[inline(always)]
    #[must_use]
    pub fn gic_axim_err_ack(&mut self) -> GicAximErrAckW<GrfCpuCon2Spec> {
        GicAximErrAckW::new(self, 14)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<GrfCpuCon2Spec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "cpu control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_cpu_con2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_cpu_con2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfCpuCon2Spec;
impl crate::RegisterSpec for GrfCpuCon2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_cpu_con2::R`](R) reader structure"]
impl crate::Readable for GrfCpuCon2Spec {}
#[doc = "`write(|w| ..)` method takes [`grf_cpu_con2::W`](W) writer structure"]
impl crate::Writable for GrfCpuCon2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_CPU_CON2 to value 0x0b"]
impl crate::Resettable for GrfCpuCon2Spec {
    const RESET_VALUE: u32 = 0x0b;
}
