#[doc = "Register `PMU_ADB400_CON` reader"]
pub type R = crate::R<PmuAdb400ConSpec>;
#[doc = "Register `PMU_ADB400_CON` writer"]
pub type W = crate::W<PmuAdb400ConSpec>;
#[doc = "software send idle request to cxcs low power interface\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PwrdwnReqCxcs {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<PwrdwnReqCxcs> for bool {
    #[inline(always)]
    fn from(variant: PwrdwnReqCxcs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRDWN_REQ_CXCS` reader - software send idle request to cxcs low power interface"]
pub type PwrdwnReqCxcsR = crate::BitReader<PwrdwnReqCxcs>;
impl PwrdwnReqCxcsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PwrdwnReqCxcs {
        match self.bits {
            false => PwrdwnReqCxcs::B0,
            true => PwrdwnReqCxcs::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PwrdwnReqCxcs::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PwrdwnReqCxcs::B1
    }
}
#[doc = "Field `PWRDWN_REQ_CXCS` writer - software send idle request to cxcs low power interface"]
pub type PwrdwnReqCxcsW<'a, REG> = crate::BitWriter<'a, REG, PwrdwnReqCxcs>;
impl<'a, REG> PwrdwnReqCxcsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PwrdwnReqCxcs::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PwrdwnReqCxcs::B1)
    }
}
#[doc = "software send idle request from core_l to cci low power interface\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PwrdwnReqCoreL {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<PwrdwnReqCoreL> for bool {
    #[inline(always)]
    fn from(variant: PwrdwnReqCoreL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRDWN_REQ_CORE_L` reader - software send idle request from core_l to cci low power interface"]
pub type PwrdwnReqCoreLR = crate::BitReader<PwrdwnReqCoreL>;
impl PwrdwnReqCoreLR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PwrdwnReqCoreL {
        match self.bits {
            false => PwrdwnReqCoreL::B0,
            true => PwrdwnReqCoreL::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PwrdwnReqCoreL::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PwrdwnReqCoreL::B1
    }
}
#[doc = "Field `PWRDWN_REQ_CORE_L` writer - software send idle request from core_l to cci low power interface"]
pub type PwrdwnReqCoreLW<'a, REG> = crate::BitWriter<'a, REG, PwrdwnReqCoreL>;
impl<'a, REG> PwrdwnReqCoreLW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PwrdwnReqCoreL::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PwrdwnReqCoreL::B1)
    }
}
#[doc = "software send idle request to path from core_l to gic low power interface\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PwrdwnReqCoreL2gic {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<PwrdwnReqCoreL2gic> for bool {
    #[inline(always)]
    fn from(variant: PwrdwnReqCoreL2gic) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRDWN_REQ_CORE_L_2GIC` reader - software send idle request to path from core_l to gic low power interface"]
pub type PwrdwnReqCoreL2gicR = crate::BitReader<PwrdwnReqCoreL2gic>;
impl PwrdwnReqCoreL2gicR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PwrdwnReqCoreL2gic {
        match self.bits {
            false => PwrdwnReqCoreL2gic::B0,
            true => PwrdwnReqCoreL2gic::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PwrdwnReqCoreL2gic::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PwrdwnReqCoreL2gic::B1
    }
}
#[doc = "Field `PWRDWN_REQ_CORE_L_2GIC` writer - software send idle request to path from core_l to gic low power interface"]
pub type PwrdwnReqCoreL2gicW<'a, REG> = crate::BitWriter<'a, REG, PwrdwnReqCoreL2gic>;
impl<'a, REG> PwrdwnReqCoreL2gicW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PwrdwnReqCoreL2gic::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PwrdwnReqCoreL2gic::B1)
    }
}
#[doc = "send idle request to path from gic to core_l low power interface\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PwrdwnReqGic2CoreL {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<PwrdwnReqGic2CoreL> for bool {
    #[inline(always)]
    fn from(variant: PwrdwnReqGic2CoreL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRDWN_REQ_GIC2_CORE_L` reader - send idle request to path from gic to core_l low power interface"]
pub type PwrdwnReqGic2CoreLR = crate::BitReader<PwrdwnReqGic2CoreL>;
impl PwrdwnReqGic2CoreLR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PwrdwnReqGic2CoreL {
        match self.bits {
            false => PwrdwnReqGic2CoreL::B0,
            true => PwrdwnReqGic2CoreL::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PwrdwnReqGic2CoreL::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PwrdwnReqGic2CoreL::B1
    }
}
#[doc = "Field `PWRDWN_REQ_GIC2_CORE_L` writer - send idle request to path from gic to core_l low power interface"]
pub type PwrdwnReqGic2CoreLW<'a, REG> = crate::BitWriter<'a, REG, PwrdwnReqGic2CoreL>;
impl<'a, REG> PwrdwnReqGic2CoreLW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PwrdwnReqGic2CoreL::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PwrdwnReqGic2CoreL::B1)
    }
}
#[doc = "software send idle request from core_b to cci low power interface\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PwrdwnReqCoreB {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<PwrdwnReqCoreB> for bool {
    #[inline(always)]
    fn from(variant: PwrdwnReqCoreB) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRDWN_REQ_CORE_B` reader - software send idle request from core_b to cci low power interface"]
pub type PwrdwnReqCoreBR = crate::BitReader<PwrdwnReqCoreB>;
impl PwrdwnReqCoreBR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PwrdwnReqCoreB {
        match self.bits {
            false => PwrdwnReqCoreB::B0,
            true => PwrdwnReqCoreB::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PwrdwnReqCoreB::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PwrdwnReqCoreB::B1
    }
}
#[doc = "Field `PWRDWN_REQ_CORE_B` writer - software send idle request from core_b to cci low power interface"]
pub type PwrdwnReqCoreBW<'a, REG> = crate::BitWriter<'a, REG, PwrdwnReqCoreB>;
impl<'a, REG> PwrdwnReqCoreBW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PwrdwnReqCoreB::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PwrdwnReqCoreB::B1)
    }
}
#[doc = "software send idle request to path from core_b to gic low power interface\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PwrdwnReqCoreB2gic {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<PwrdwnReqCoreB2gic> for bool {
    #[inline(always)]
    fn from(variant: PwrdwnReqCoreB2gic) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRDWN_REQ_CORE_B_2GIC` reader - software send idle request to path from core_b to gic low power interface"]
pub type PwrdwnReqCoreB2gicR = crate::BitReader<PwrdwnReqCoreB2gic>;
impl PwrdwnReqCoreB2gicR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PwrdwnReqCoreB2gic {
        match self.bits {
            false => PwrdwnReqCoreB2gic::B0,
            true => PwrdwnReqCoreB2gic::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PwrdwnReqCoreB2gic::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PwrdwnReqCoreB2gic::B1
    }
}
#[doc = "Field `PWRDWN_REQ_CORE_B_2GIC` writer - software send idle request to path from core_b to gic low power interface"]
pub type PwrdwnReqCoreB2gicW<'a, REG> = crate::BitWriter<'a, REG, PwrdwnReqCoreB2gic>;
impl<'a, REG> PwrdwnReqCoreB2gicW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PwrdwnReqCoreB2gic::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PwrdwnReqCoreB2gic::B1)
    }
}
#[doc = "send idle request to path from gic to core_b low power interface\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PwrdwnReqGic2CoreB {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<PwrdwnReqGic2CoreB> for bool {
    #[inline(always)]
    fn from(variant: PwrdwnReqGic2CoreB) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRDWN_REQ_GIC2_CORE_B` reader - send idle request to path from gic to core_b low power interface"]
pub type PwrdwnReqGic2CoreBR = crate::BitReader<PwrdwnReqGic2CoreB>;
impl PwrdwnReqGic2CoreBR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PwrdwnReqGic2CoreB {
        match self.bits {
            false => PwrdwnReqGic2CoreB::B0,
            true => PwrdwnReqGic2CoreB::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PwrdwnReqGic2CoreB::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PwrdwnReqGic2CoreB::B1
    }
}
#[doc = "Field `PWRDWN_REQ_GIC2_CORE_B` writer - send idle request to path from gic to core_b low power interface"]
pub type PwrdwnReqGic2CoreBW<'a, REG> = crate::BitWriter<'a, REG, PwrdwnReqGic2CoreB>;
impl<'a, REG> PwrdwnReqGic2CoreBW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PwrdwnReqGic2CoreB::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PwrdwnReqGic2CoreB::B1)
    }
}
#[doc = "hardware send idle request to cxcs low power interface\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClrCxcs {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<ClrCxcs> for bool {
    #[inline(always)]
    fn from(variant: ClrCxcs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_CXCS` reader - hardware send idle request to cxcs low power interface"]
pub type ClrCxcsR = crate::BitReader<ClrCxcs>;
impl ClrCxcsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClrCxcs {
        match self.bits {
            false => ClrCxcs::B0,
            true => ClrCxcs::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ClrCxcs::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ClrCxcs::B1
    }
}
#[doc = "Field `CLR_CXCS` writer - hardware send idle request to cxcs low power interface"]
pub type ClrCxcsW<'a, REG> = crate::BitWriter<'a, REG, ClrCxcs>;
impl<'a, REG> ClrCxcsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ClrCxcs::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ClrCxcs::B1)
    }
}
#[doc = "software send idle request from core_l to cci low power interface\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClrCoreL {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<ClrCoreL> for bool {
    #[inline(always)]
    fn from(variant: ClrCoreL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_CORE_L` reader - software send idle request from core_l to cci low power interface"]
pub type ClrCoreLR = crate::BitReader<ClrCoreL>;
impl ClrCoreLR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClrCoreL {
        match self.bits {
            false => ClrCoreL::B0,
            true => ClrCoreL::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ClrCoreL::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ClrCoreL::B1
    }
}
#[doc = "Field `CLR_CORE_L` writer - software send idle request from core_l to cci low power interface"]
pub type ClrCoreLW<'a, REG> = crate::BitWriter<'a, REG, ClrCoreL>;
impl<'a, REG> ClrCoreLW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ClrCoreL::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ClrCoreL::B1)
    }
}
#[doc = "hardware send idle request to path from core_l to gic low power interface\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClrCoreL2gic {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<ClrCoreL2gic> for bool {
    #[inline(always)]
    fn from(variant: ClrCoreL2gic) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_CORE_L_2GIC` reader - hardware send idle request to path from core_l to gic low power interface"]
pub type ClrCoreL2gicR = crate::BitReader<ClrCoreL2gic>;
impl ClrCoreL2gicR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClrCoreL2gic {
        match self.bits {
            false => ClrCoreL2gic::B0,
            true => ClrCoreL2gic::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ClrCoreL2gic::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ClrCoreL2gic::B1
    }
}
#[doc = "Field `CLR_CORE_L_2GIC` writer - hardware send idle request to path from core_l to gic low power interface"]
pub type ClrCoreL2gicW<'a, REG> = crate::BitWriter<'a, REG, ClrCoreL2gic>;
impl<'a, REG> ClrCoreL2gicW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ClrCoreL2gic::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ClrCoreL2gic::B1)
    }
}
#[doc = "hardware send idle request to path from gic to core_l low power interface\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClrGic2CoreL {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<ClrGic2CoreL> for bool {
    #[inline(always)]
    fn from(variant: ClrGic2CoreL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_GIC2_CORE_L` reader - hardware send idle request to path from gic to core_l low power interface"]
pub type ClrGic2CoreLR = crate::BitReader<ClrGic2CoreL>;
impl ClrGic2CoreLR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClrGic2CoreL {
        match self.bits {
            false => ClrGic2CoreL::B0,
            true => ClrGic2CoreL::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ClrGic2CoreL::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ClrGic2CoreL::B1
    }
}
#[doc = "Field `CLR_GIC2_CORE_L` writer - hardware send idle request to path from gic to core_l low power interface"]
pub type ClrGic2CoreLW<'a, REG> = crate::BitWriter<'a, REG, ClrGic2CoreL>;
impl<'a, REG> ClrGic2CoreLW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ClrGic2CoreL::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ClrGic2CoreL::B1)
    }
}
#[doc = "hardware send idle request from core_b to cci low power interface\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClrCoreB {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<ClrCoreB> for bool {
    #[inline(always)]
    fn from(variant: ClrCoreB) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_CORE_B` reader - hardware send idle request from core_b to cci low power interface"]
pub type ClrCoreBR = crate::BitReader<ClrCoreB>;
impl ClrCoreBR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClrCoreB {
        match self.bits {
            false => ClrCoreB::B0,
            true => ClrCoreB::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ClrCoreB::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ClrCoreB::B1
    }
}
#[doc = "Field `CLR_CORE_B` writer - hardware send idle request from core_b to cci low power interface"]
pub type ClrCoreBW<'a, REG> = crate::BitWriter<'a, REG, ClrCoreB>;
impl<'a, REG> ClrCoreBW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ClrCoreB::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ClrCoreB::B1)
    }
}
#[doc = "hardware send idle request to path from core_b to gic low power interface\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClrCoreB2gic {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<ClrCoreB2gic> for bool {
    #[inline(always)]
    fn from(variant: ClrCoreB2gic) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_CORE_B_2GIC` reader - hardware send idle request to path from core_b to gic low power interface"]
pub type ClrCoreB2gicR = crate::BitReader<ClrCoreB2gic>;
impl ClrCoreB2gicR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClrCoreB2gic {
        match self.bits {
            false => ClrCoreB2gic::B0,
            true => ClrCoreB2gic::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ClrCoreB2gic::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ClrCoreB2gic::B1
    }
}
#[doc = "Field `CLR_CORE_B_2GIC` writer - hardware send idle request to path from core_b to gic low power interface"]
pub type ClrCoreB2gicW<'a, REG> = crate::BitWriter<'a, REG, ClrCoreB2gic>;
impl<'a, REG> ClrCoreB2gicW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ClrCoreB2gic::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ClrCoreB2gic::B1)
    }
}
#[doc = "hardware send idle request to path from gic to core_b low power interface\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClrGic2CoreB {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<ClrGic2CoreB> for bool {
    #[inline(always)]
    fn from(variant: ClrGic2CoreB) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_GIC2_CORE_B` reader - hardware send idle request to path from gic to core_b low power interface"]
pub type ClrGic2CoreBR = crate::BitReader<ClrGic2CoreB>;
impl ClrGic2CoreBR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClrGic2CoreB {
        match self.bits {
            false => ClrGic2CoreB::B0,
            true => ClrGic2CoreB::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ClrGic2CoreB::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ClrGic2CoreB::B1
    }
}
#[doc = "Field `CLR_GIC2_CORE_B` writer - hardware send idle request to path from gic to core_b low power interface"]
pub type ClrGic2CoreBW<'a, REG> = crate::BitWriter<'a, REG, ClrGic2CoreB>;
impl<'a, REG> ClrGic2CoreBW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ClrGic2CoreB::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ClrGic2CoreB::B1)
    }
}
#[doc = "Field `WRITE_ENABLE` reader - When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - software send idle request to cxcs low power interface"]
    #[inline(always)]
    pub fn pwrdwn_req_cxcs(&self) -> PwrdwnReqCxcsR {
        PwrdwnReqCxcsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - software send idle request from core_l to cci low power interface"]
    #[inline(always)]
    pub fn pwrdwn_req_core_l(&self) -> PwrdwnReqCoreLR {
        PwrdwnReqCoreLR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - software send idle request to path from core_l to gic low power interface"]
    #[inline(always)]
    pub fn pwrdwn_req_core_l_2gic(&self) -> PwrdwnReqCoreL2gicR {
        PwrdwnReqCoreL2gicR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - send idle request to path from gic to core_l low power interface"]
    #[inline(always)]
    pub fn pwrdwn_req_gic2_core_l(&self) -> PwrdwnReqGic2CoreLR {
        PwrdwnReqGic2CoreLR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - software send idle request from core_b to cci low power interface"]
    #[inline(always)]
    pub fn pwrdwn_req_core_b(&self) -> PwrdwnReqCoreBR {
        PwrdwnReqCoreBR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - software send idle request to path from core_b to gic low power interface"]
    #[inline(always)]
    pub fn pwrdwn_req_core_b_2gic(&self) -> PwrdwnReqCoreB2gicR {
        PwrdwnReqCoreB2gicR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - send idle request to path from gic to core_b low power interface"]
    #[inline(always)]
    pub fn pwrdwn_req_gic2_core_b(&self) -> PwrdwnReqGic2CoreBR {
        PwrdwnReqGic2CoreBR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - hardware send idle request to cxcs low power interface"]
    #[inline(always)]
    pub fn clr_cxcs(&self) -> ClrCxcsR {
        ClrCxcsR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - software send idle request from core_l to cci low power interface"]
    #[inline(always)]
    pub fn clr_core_l(&self) -> ClrCoreLR {
        ClrCoreLR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - hardware send idle request to path from core_l to gic low power interface"]
    #[inline(always)]
    pub fn clr_core_l_2gic(&self) -> ClrCoreL2gicR {
        ClrCoreL2gicR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - hardware send idle request to path from gic to core_l low power interface"]
    #[inline(always)]
    pub fn clr_gic2_core_l(&self) -> ClrGic2CoreLR {
        ClrGic2CoreLR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - hardware send idle request from core_b to cci low power interface"]
    #[inline(always)]
    pub fn clr_core_b(&self) -> ClrCoreBR {
        ClrCoreBR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - hardware send idle request to path from core_b to gic low power interface"]
    #[inline(always)]
    pub fn clr_core_b_2gic(&self) -> ClrCoreB2gicR {
        ClrCoreB2gicR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - hardware send idle request to path from gic to core_b low power interface"]
    #[inline(always)]
    pub fn clr_gic2_core_b(&self) -> ClrGic2CoreBR {
        ClrGic2CoreBR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:31 - When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - software send idle request to cxcs low power interface"]
    #[inline(always)]
    #[must_use]
    pub fn pwrdwn_req_cxcs(&mut self) -> PwrdwnReqCxcsW<PmuAdb400ConSpec> {
        PwrdwnReqCxcsW::new(self, 0)
    }
    #[doc = "Bit 1 - software send idle request from core_l to cci low power interface"]
    #[inline(always)]
    #[must_use]
    pub fn pwrdwn_req_core_l(&mut self) -> PwrdwnReqCoreLW<PmuAdb400ConSpec> {
        PwrdwnReqCoreLW::new(self, 1)
    }
    #[doc = "Bit 2 - software send idle request to path from core_l to gic low power interface"]
    #[inline(always)]
    #[must_use]
    pub fn pwrdwn_req_core_l_2gic(&mut self) -> PwrdwnReqCoreL2gicW<PmuAdb400ConSpec> {
        PwrdwnReqCoreL2gicW::new(self, 2)
    }
    #[doc = "Bit 3 - send idle request to path from gic to core_l low power interface"]
    #[inline(always)]
    #[must_use]
    pub fn pwrdwn_req_gic2_core_l(&mut self) -> PwrdwnReqGic2CoreLW<PmuAdb400ConSpec> {
        PwrdwnReqGic2CoreLW::new(self, 3)
    }
    #[doc = "Bit 4 - software send idle request from core_b to cci low power interface"]
    #[inline(always)]
    #[must_use]
    pub fn pwrdwn_req_core_b(&mut self) -> PwrdwnReqCoreBW<PmuAdb400ConSpec> {
        PwrdwnReqCoreBW::new(self, 4)
    }
    #[doc = "Bit 5 - software send idle request to path from core_b to gic low power interface"]
    #[inline(always)]
    #[must_use]
    pub fn pwrdwn_req_core_b_2gic(&mut self) -> PwrdwnReqCoreB2gicW<PmuAdb400ConSpec> {
        PwrdwnReqCoreB2gicW::new(self, 5)
    }
    #[doc = "Bit 6 - send idle request to path from gic to core_b low power interface"]
    #[inline(always)]
    #[must_use]
    pub fn pwrdwn_req_gic2_core_b(&mut self) -> PwrdwnReqGic2CoreBW<PmuAdb400ConSpec> {
        PwrdwnReqGic2CoreBW::new(self, 6)
    }
    #[doc = "Bit 8 - hardware send idle request to cxcs low power interface"]
    #[inline(always)]
    #[must_use]
    pub fn clr_cxcs(&mut self) -> ClrCxcsW<PmuAdb400ConSpec> {
        ClrCxcsW::new(self, 8)
    }
    #[doc = "Bit 9 - software send idle request from core_l to cci low power interface"]
    #[inline(always)]
    #[must_use]
    pub fn clr_core_l(&mut self) -> ClrCoreLW<PmuAdb400ConSpec> {
        ClrCoreLW::new(self, 9)
    }
    #[doc = "Bit 10 - hardware send idle request to path from core_l to gic low power interface"]
    #[inline(always)]
    #[must_use]
    pub fn clr_core_l_2gic(&mut self) -> ClrCoreL2gicW<PmuAdb400ConSpec> {
        ClrCoreL2gicW::new(self, 10)
    }
    #[doc = "Bit 11 - hardware send idle request to path from gic to core_l low power interface"]
    #[inline(always)]
    #[must_use]
    pub fn clr_gic2_core_l(&mut self) -> ClrGic2CoreLW<PmuAdb400ConSpec> {
        ClrGic2CoreLW::new(self, 11)
    }
    #[doc = "Bit 12 - hardware send idle request from core_b to cci low power interface"]
    #[inline(always)]
    #[must_use]
    pub fn clr_core_b(&mut self) -> ClrCoreBW<PmuAdb400ConSpec> {
        ClrCoreBW::new(self, 12)
    }
    #[doc = "Bit 13 - hardware send idle request to path from core_b to gic low power interface"]
    #[inline(always)]
    #[must_use]
    pub fn clr_core_b_2gic(&mut self) -> ClrCoreB2gicW<PmuAdb400ConSpec> {
        ClrCoreB2gicW::new(self, 13)
    }
    #[doc = "Bit 14 - hardware send idle request to path from gic to core_b low power interface"]
    #[inline(always)]
    #[must_use]
    pub fn clr_gic2_core_b(&mut self) -> ClrGic2CoreBW<PmuAdb400ConSpec> {
        ClrGic2CoreBW::new(self, 14)
    }
    #[doc = "Bits 16:31 - When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<PmuAdb400ConSpec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "adb-400 low power control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_adb400_con::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_adb400_con::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmuAdb400ConSpec;
impl crate::RegisterSpec for PmuAdb400ConSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmu_adb400_con::R`](R) reader structure"]
impl crate::Readable for PmuAdb400ConSpec {}
#[doc = "`write(|w| ..)` method takes [`pmu_adb400_con::W`](W) writer structure"]
impl crate::Writable for PmuAdb400ConSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMU_ADB400_CON to value 0"]
impl crate::Resettable for PmuAdb400ConSpec {
    const RESET_VALUE: u32 = 0;
}
