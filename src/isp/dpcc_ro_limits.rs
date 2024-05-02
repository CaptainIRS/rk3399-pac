#[doc = "Register `DPCC_RO_LIMITS` reader"]
pub type R = crate::R<DpccRoLimitsSpec>;
#[doc = "Register `DPCC_RO_LIMITS` writer"]
pub type W = crate::W<DpccRoLimitsSpec>;
#[doc = "Field `RO_LIM_1_G` reader - Rank order limit for set 1 green"]
pub type RoLim1GR = crate::FieldReader;
#[doc = "Field `RO_LIM_1_G` writer - Rank order limit for set 1 green"]
pub type RoLim1GW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RO_LIM_1_RB` reader - Rank order limit for set 1 red/blue"]
pub type RoLim1RbR = crate::FieldReader;
#[doc = "Field `RO_LIM_1_RB` writer - Rank order limit for set 1 red/blue"]
pub type RoLim1RbW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RO_LIM_2_G` reader - Rank order limit for set 2 green"]
pub type RoLim2GR = crate::FieldReader;
#[doc = "Field `RO_LIM_2_G` writer - Rank order limit for set 2 green"]
pub type RoLim2GW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RO_LIM_2_RB` reader - Rank order limit for set 2 red/blue"]
pub type RoLim2RbR = crate::FieldReader;
#[doc = "Field `RO_LIM_2_RB` writer - Rank order limit for set 2 red/blue"]
pub type RoLim2RbW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RO_LIM_3_G` reader - Rank order limit for set 3 green"]
pub type RoLim3GR = crate::FieldReader;
#[doc = "Field `RO_LIM_3_G` writer - Rank order limit for set 3 green"]
pub type RoLim3GW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RO_LIM_3_RB` reader - Rank order limit for set 3 red/blue"]
pub type RoLim3RbR = crate::FieldReader;
#[doc = "Field `RO_LIM_3_RB` writer - Rank order limit for set 3 red/blue"]
pub type RoLim3RbW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Rank order limit for set 1 green"]
    #[inline(always)]
    pub fn ro_lim_1_g(&self) -> RoLim1GR {
        RoLim1GR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Rank order limit for set 1 red/blue"]
    #[inline(always)]
    pub fn ro_lim_1_rb(&self) -> RoLim1RbR {
        RoLim1RbR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Rank order limit for set 2 green"]
    #[inline(always)]
    pub fn ro_lim_2_g(&self) -> RoLim2GR {
        RoLim2GR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Rank order limit for set 2 red/blue"]
    #[inline(always)]
    pub fn ro_lim_2_rb(&self) -> RoLim2RbR {
        RoLim2RbR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Rank order limit for set 3 green"]
    #[inline(always)]
    pub fn ro_lim_3_g(&self) -> RoLim3GR {
        RoLim3GR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Rank order limit for set 3 red/blue"]
    #[inline(always)]
    pub fn ro_lim_3_rb(&self) -> RoLim3RbR {
        RoLim3RbR::new(((self.bits >> 10) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Rank order limit for set 1 green"]
    #[inline(always)]
    #[must_use]
    pub fn ro_lim_1_g(&mut self) -> RoLim1GW<DpccRoLimitsSpec> {
        RoLim1GW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Rank order limit for set 1 red/blue"]
    #[inline(always)]
    #[must_use]
    pub fn ro_lim_1_rb(&mut self) -> RoLim1RbW<DpccRoLimitsSpec> {
        RoLim1RbW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Rank order limit for set 2 green"]
    #[inline(always)]
    #[must_use]
    pub fn ro_lim_2_g(&mut self) -> RoLim2GW<DpccRoLimitsSpec> {
        RoLim2GW::new(self, 4)
    }
    #[doc = "Bits 6:7 - Rank order limit for set 2 red/blue"]
    #[inline(always)]
    #[must_use]
    pub fn ro_lim_2_rb(&mut self) -> RoLim2RbW<DpccRoLimitsSpec> {
        RoLim2RbW::new(self, 6)
    }
    #[doc = "Bits 8:9 - Rank order limit for set 3 green"]
    #[inline(always)]
    #[must_use]
    pub fn ro_lim_3_g(&mut self) -> RoLim3GW<DpccRoLimitsSpec> {
        RoLim3GW::new(self, 8)
    }
    #[doc = "Bits 10:11 - Rank order limit for set 3 red/blue"]
    #[inline(always)]
    #[must_use]
    pub fn ro_lim_3_rb(&mut self) -> RoLim3RbW<DpccRoLimitsSpec> {
        RoLim3RbW::new(self, 10)
    }
}
#[doc = "Rank Order Limits\n\nNote: all values are unsigned integer \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpcc_ro_limits::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpcc_ro_limits::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpccRoLimitsSpec;
impl crate::RegisterSpec for DpccRoLimitsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpcc_ro_limits::R`](R) reader structure"]
impl crate::Readable for DpccRoLimitsSpec {}
#[doc = "`write(|w| ..)` method takes [`dpcc_ro_limits::W`](W) writer structure"]
impl crate::Writable for DpccRoLimitsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DPCC_RO_LIMITS to value 0"]
impl crate::Resettable for DpccRoLimitsSpec {
    const RESET_VALUE: u32 = 0;
}
