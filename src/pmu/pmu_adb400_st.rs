#[doc = "Register `PMU_ADB400_ST` reader"]
pub type R = crate::R<PmuAdb400StSpec>;
#[doc = "Register `PMU_ADB400_ST` writer"]
pub type W = crate::W<PmuAdb400StSpec>;
#[doc = "idle acknowledge status from cxcs\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PwrdwnAckCxcs {
    #[doc = "0: idle acknowledge status of adb is 1"]
    B0 = 0,
    #[doc = "1: idle acknowledge status of adb is 1"]
    B1 = 1,
}
impl From<PwrdwnAckCxcs> for bool {
    #[inline(always)]
    fn from(variant: PwrdwnAckCxcs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRDWN_ACK_CXCS` reader - idle acknowledge status from cxcs"]
pub type PwrdwnAckCxcsR = crate::BitReader<PwrdwnAckCxcs>;
impl PwrdwnAckCxcsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PwrdwnAckCxcs {
        match self.bits {
            false => PwrdwnAckCxcs::B0,
            true => PwrdwnAckCxcs::B1,
        }
    }
    #[doc = "idle acknowledge status of adb is 1"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PwrdwnAckCxcs::B0
    }
    #[doc = "idle acknowledge status of adb is 1"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PwrdwnAckCxcs::B1
    }
}
#[doc = "idle acknowledge status from cxcs\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PwrdwnAckCoreL {
    #[doc = "0: idle acknowledge status of adb is 1"]
    B0 = 0,
    #[doc = "1: idle acknowledge status of adb is 1"]
    B1 = 1,
}
impl From<PwrdwnAckCoreL> for bool {
    #[inline(always)]
    fn from(variant: PwrdwnAckCoreL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRDWN_ACK_CORE_L` reader - idle acknowledge status from cxcs"]
pub type PwrdwnAckCoreLR = crate::BitReader<PwrdwnAckCoreL>;
impl PwrdwnAckCoreLR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PwrdwnAckCoreL {
        match self.bits {
            false => PwrdwnAckCoreL::B0,
            true => PwrdwnAckCoreL::B1,
        }
    }
    #[doc = "idle acknowledge status of adb is 1"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PwrdwnAckCoreL::B0
    }
    #[doc = "idle acknowledge status of adb is 1"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PwrdwnAckCoreL::B1
    }
}
#[doc = "idle acknowledge status from cxcs\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PwrdwnAckCoreL2gic {
    #[doc = "0: idle acknowledge status of adb is 1"]
    B0 = 0,
    #[doc = "1: idle acknowledge status of adb is 1"]
    B1 = 1,
}
impl From<PwrdwnAckCoreL2gic> for bool {
    #[inline(always)]
    fn from(variant: PwrdwnAckCoreL2gic) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRDWN_ACK_CORE_L_2GIC` reader - idle acknowledge status from cxcs"]
pub type PwrdwnAckCoreL2gicR = crate::BitReader<PwrdwnAckCoreL2gic>;
impl PwrdwnAckCoreL2gicR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PwrdwnAckCoreL2gic {
        match self.bits {
            false => PwrdwnAckCoreL2gic::B0,
            true => PwrdwnAckCoreL2gic::B1,
        }
    }
    #[doc = "idle acknowledge status of adb is 1"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PwrdwnAckCoreL2gic::B0
    }
    #[doc = "idle acknowledge status of adb is 1"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PwrdwnAckCoreL2gic::B1
    }
}
#[doc = "idle acknowledge status from cxcs\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PwrdwnAckGic2CoreL {
    #[doc = "0: idle acknowledge status of adb is 1"]
    B0 = 0,
    #[doc = "1: idle acknowledge status of adb is 1"]
    B1 = 1,
}
impl From<PwrdwnAckGic2CoreL> for bool {
    #[inline(always)]
    fn from(variant: PwrdwnAckGic2CoreL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRDWN_ACK_GIC2_CORE_L` reader - idle acknowledge status from cxcs"]
pub type PwrdwnAckGic2CoreLR = crate::BitReader<PwrdwnAckGic2CoreL>;
impl PwrdwnAckGic2CoreLR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PwrdwnAckGic2CoreL {
        match self.bits {
            false => PwrdwnAckGic2CoreL::B0,
            true => PwrdwnAckGic2CoreL::B1,
        }
    }
    #[doc = "idle acknowledge status of adb is 1"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PwrdwnAckGic2CoreL::B0
    }
    #[doc = "idle acknowledge status of adb is 1"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PwrdwnAckGic2CoreL::B1
    }
}
#[doc = "idle acknowledge status from cxcs\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PwrdwnAckCoreB {
    #[doc = "0: idle acknowledge status of adb is 1"]
    B0 = 0,
    #[doc = "1: idle acknowledge status of adb is 1"]
    B1 = 1,
}
impl From<PwrdwnAckCoreB> for bool {
    #[inline(always)]
    fn from(variant: PwrdwnAckCoreB) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRDWN_ACK_CORE_B` reader - idle acknowledge status from cxcs"]
pub type PwrdwnAckCoreBR = crate::BitReader<PwrdwnAckCoreB>;
impl PwrdwnAckCoreBR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PwrdwnAckCoreB {
        match self.bits {
            false => PwrdwnAckCoreB::B0,
            true => PwrdwnAckCoreB::B1,
        }
    }
    #[doc = "idle acknowledge status of adb is 1"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PwrdwnAckCoreB::B0
    }
    #[doc = "idle acknowledge status of adb is 1"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PwrdwnAckCoreB::B1
    }
}
#[doc = "idle acknowledge status from cxcs\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PwrdwnAckCoreB2gic {
    #[doc = "0: idle acknowledge status of adb is 1"]
    B0 = 0,
    #[doc = "1: idle acknowledge status of adb is 1"]
    B1 = 1,
}
impl From<PwrdwnAckCoreB2gic> for bool {
    #[inline(always)]
    fn from(variant: PwrdwnAckCoreB2gic) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRDWN_ACK_CORE_B_2GIC` reader - idle acknowledge status from cxcs"]
pub type PwrdwnAckCoreB2gicR = crate::BitReader<PwrdwnAckCoreB2gic>;
impl PwrdwnAckCoreB2gicR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PwrdwnAckCoreB2gic {
        match self.bits {
            false => PwrdwnAckCoreB2gic::B0,
            true => PwrdwnAckCoreB2gic::B1,
        }
    }
    #[doc = "idle acknowledge status of adb is 1"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PwrdwnAckCoreB2gic::B0
    }
    #[doc = "idle acknowledge status of adb is 1"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PwrdwnAckCoreB2gic::B1
    }
}
#[doc = "idle acknowledge status from cxcs\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PwrdwnAckGic2CoreB {
    #[doc = "0: idle acknowledge status of adb is 1"]
    B0 = 0,
    #[doc = "1: idle acknowledge status of adb is 1"]
    B1 = 1,
}
impl From<PwrdwnAckGic2CoreB> for bool {
    #[inline(always)]
    fn from(variant: PwrdwnAckGic2CoreB) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRDWN_ACK_GIC2_CORE_B` reader - idle acknowledge status from cxcs"]
pub type PwrdwnAckGic2CoreBR = crate::BitReader<PwrdwnAckGic2CoreB>;
impl PwrdwnAckGic2CoreBR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PwrdwnAckGic2CoreB {
        match self.bits {
            false => PwrdwnAckGic2CoreB::B0,
            true => PwrdwnAckGic2CoreB::B1,
        }
    }
    #[doc = "idle acknowledge status of adb is 1"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PwrdwnAckGic2CoreB::B0
    }
    #[doc = "idle acknowledge status of adb is 1"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PwrdwnAckGic2CoreB::B1
    }
}
#[doc = "active status of cxcs low power interface\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActiveCxcs {
    #[doc = "0: active status is 1 (active)"]
    B0 = 0,
    #[doc = "1: active status is 1 (active)"]
    B1 = 1,
}
impl From<ActiveCxcs> for bool {
    #[inline(always)]
    fn from(variant: ActiveCxcs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACTIVE_CXCS` reader - active status of cxcs low power interface"]
pub type ActiveCxcsR = crate::BitReader<ActiveCxcs>;
impl ActiveCxcsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ActiveCxcs {
        match self.bits {
            false => ActiveCxcs::B0,
            true => ActiveCxcs::B1,
        }
    }
    #[doc = "active status is 1 (active)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ActiveCxcs::B0
    }
    #[doc = "active status is 1 (active)"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ActiveCxcs::B1
    }
}
#[doc = "active status of cxcs low power interface\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleCoreL {
    #[doc = "0: active status is 1 (active)"]
    B0 = 0,
    #[doc = "1: active status is 1 (active)"]
    B1 = 1,
}
impl From<IdleCoreL> for bool {
    #[inline(always)]
    fn from(variant: IdleCoreL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_CORE_L` reader - active status of cxcs low power interface"]
pub type IdleCoreLR = crate::BitReader<IdleCoreL>;
impl IdleCoreLR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleCoreL {
        match self.bits {
            false => IdleCoreL::B0,
            true => IdleCoreL::B1,
        }
    }
    #[doc = "active status is 1 (active)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleCoreL::B0
    }
    #[doc = "active status is 1 (active)"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleCoreL::B1
    }
}
#[doc = "active status of cxcs low power interface\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleCoreL2gic {
    #[doc = "0: active status is 1 (active)"]
    B0 = 0,
    #[doc = "1: active status is 1 (active)"]
    B1 = 1,
}
impl From<IdleCoreL2gic> for bool {
    #[inline(always)]
    fn from(variant: IdleCoreL2gic) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_CORE_L_2GIC` reader - active status of cxcs low power interface"]
pub type IdleCoreL2gicR = crate::BitReader<IdleCoreL2gic>;
impl IdleCoreL2gicR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleCoreL2gic {
        match self.bits {
            false => IdleCoreL2gic::B0,
            true => IdleCoreL2gic::B1,
        }
    }
    #[doc = "active status is 1 (active)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleCoreL2gic::B0
    }
    #[doc = "active status is 1 (active)"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleCoreL2gic::B1
    }
}
#[doc = "active status of cxcs low power interface\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleGic2CoreL {
    #[doc = "0: active status is 1 (active)"]
    B0 = 0,
    #[doc = "1: active status is 1 (active)"]
    B1 = 1,
}
impl From<IdleGic2CoreL> for bool {
    #[inline(always)]
    fn from(variant: IdleGic2CoreL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_GIC2_CORE_L` writer - active status of cxcs low power interface"]
pub type IdleGic2CoreLW<'a, REG> = crate::BitWriter<'a, REG, IdleGic2CoreL>;
impl<'a, REG> IdleGic2CoreLW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "active status is 1 (active)"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IdleGic2CoreL::B0)
    }
    #[doc = "active status is 1 (active)"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IdleGic2CoreL::B1)
    }
}
#[doc = "active status of cxcs low power interface\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleCoreB {
    #[doc = "0: active status is 1 (active)"]
    B0 = 0,
    #[doc = "1: active status is 1 (active)"]
    B1 = 1,
}
impl From<IdleCoreB> for bool {
    #[inline(always)]
    fn from(variant: IdleCoreB) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_CORE_B` reader - active status of cxcs low power interface"]
pub type IdleCoreBR = crate::BitReader<IdleCoreB>;
impl IdleCoreBR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleCoreB {
        match self.bits {
            false => IdleCoreB::B0,
            true => IdleCoreB::B1,
        }
    }
    #[doc = "active status is 1 (active)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleCoreB::B0
    }
    #[doc = "active status is 1 (active)"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleCoreB::B1
    }
}
#[doc = "active status of cxcs low power interface\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleCoreB2gic {
    #[doc = "0: active status is 1 (active)"]
    B0 = 0,
    #[doc = "1: active status is 1 (active)"]
    B1 = 1,
}
impl From<IdleCoreB2gic> for bool {
    #[inline(always)]
    fn from(variant: IdleCoreB2gic) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_CORE_B_2GIC` reader - active status of cxcs low power interface"]
pub type IdleCoreB2gicR = crate::BitReader<IdleCoreB2gic>;
impl IdleCoreB2gicR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleCoreB2gic {
        match self.bits {
            false => IdleCoreB2gic::B0,
            true => IdleCoreB2gic::B1,
        }
    }
    #[doc = "active status is 1 (active)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleCoreB2gic::B0
    }
    #[doc = "active status is 1 (active)"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleCoreB2gic::B1
    }
}
#[doc = "active status of cxcs low power interface\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleGic2CoreB {
    #[doc = "0: active status is 1 (active)"]
    B0 = 0,
    #[doc = "1: active status is 1 (active)"]
    B1 = 1,
}
impl From<IdleGic2CoreB> for bool {
    #[inline(always)]
    fn from(variant: IdleGic2CoreB) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE_GIC2_CORE_B` reader - active status of cxcs low power interface"]
pub type IdleGic2CoreBR = crate::BitReader<IdleGic2CoreB>;
impl IdleGic2CoreBR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IdleGic2CoreB {
        match self.bits {
            false => IdleGic2CoreB::B0,
            true => IdleGic2CoreB::B1,
        }
    }
    #[doc = "active status is 1 (active)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IdleGic2CoreB::B0
    }
    #[doc = "active status is 1 (active)"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IdleGic2CoreB::B1
    }
}
impl R {
    #[doc = "Bit 0 - idle acknowledge status from cxcs"]
    #[inline(always)]
    pub fn pwrdwn_ack_cxcs(&self) -> PwrdwnAckCxcsR {
        PwrdwnAckCxcsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - idle acknowledge status from cxcs"]
    #[inline(always)]
    pub fn pwrdwn_ack_core_l(&self) -> PwrdwnAckCoreLR {
        PwrdwnAckCoreLR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - idle acknowledge status from cxcs"]
    #[inline(always)]
    pub fn pwrdwn_ack_core_l_2gic(&self) -> PwrdwnAckCoreL2gicR {
        PwrdwnAckCoreL2gicR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - idle acknowledge status from cxcs"]
    #[inline(always)]
    pub fn pwrdwn_ack_gic2_core_l(&self) -> PwrdwnAckGic2CoreLR {
        PwrdwnAckGic2CoreLR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - idle acknowledge status from cxcs"]
    #[inline(always)]
    pub fn pwrdwn_ack_core_b(&self) -> PwrdwnAckCoreBR {
        PwrdwnAckCoreBR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - idle acknowledge status from cxcs"]
    #[inline(always)]
    pub fn pwrdwn_ack_core_b_2gic(&self) -> PwrdwnAckCoreB2gicR {
        PwrdwnAckCoreB2gicR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - idle acknowledge status from cxcs"]
    #[inline(always)]
    pub fn pwrdwn_ack_gic2_core_b(&self) -> PwrdwnAckGic2CoreBR {
        PwrdwnAckGic2CoreBR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - active status of cxcs low power interface"]
    #[inline(always)]
    pub fn active_cxcs(&self) -> ActiveCxcsR {
        ActiveCxcsR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - active status of cxcs low power interface"]
    #[inline(always)]
    pub fn idle_core_l(&self) -> IdleCoreLR {
        IdleCoreLR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - active status of cxcs low power interface"]
    #[inline(always)]
    pub fn idle_core_l_2gic(&self) -> IdleCoreL2gicR {
        IdleCoreL2gicR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - active status of cxcs low power interface"]
    #[inline(always)]
    pub fn idle_core_b(&self) -> IdleCoreBR {
        IdleCoreBR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - active status of cxcs low power interface"]
    #[inline(always)]
    pub fn idle_core_b_2gic(&self) -> IdleCoreB2gicR {
        IdleCoreB2gicR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - active status of cxcs low power interface"]
    #[inline(always)]
    pub fn idle_gic2_core_b(&self) -> IdleGic2CoreBR {
        IdleGic2CoreBR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 11 - active status of cxcs low power interface"]
    #[inline(always)]
    #[must_use]
    pub fn idle_gic2_core_l(&mut self) -> IdleGic2CoreLW<PmuAdb400StSpec> {
        IdleGic2CoreLW::new(self, 11)
    }
}
#[doc = "adb-400 low power status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_adb400_st::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_adb400_st::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmuAdb400StSpec;
impl crate::RegisterSpec for PmuAdb400StSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmu_adb400_st::R`](R) reader structure"]
impl crate::Readable for PmuAdb400StSpec {}
#[doc = "`write(|w| ..)` method takes [`pmu_adb400_st::W`](W) writer structure"]
impl crate::Writable for PmuAdb400StSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMU_ADB400_ST to value 0"]
impl crate::Resettable for PmuAdb400StSpec {
    const RESET_VALUE: u32 = 0;
}
