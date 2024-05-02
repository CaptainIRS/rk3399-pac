#[doc = "Register `DPCC_RG_FAC_1` reader"]
pub type R = crate::R<DpccRgFac1Spec>;
#[doc = "Register `DPCC_RG_FAC_1` writer"]
pub type W = crate::W<DpccRgFac1Spec>;
#[doc = "Field `RG_FAC_1_G` reader - green"]
pub type RgFac1GR = crate::FieldReader;
#[doc = "Field `RG_FAC_1_G` writer - green"]
pub type RgFac1GW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RG_FAC_1_RB` reader - red/blue"]
pub type RgFac1RbR = crate::FieldReader;
#[doc = "Field `RG_FAC_1_RB` writer - red/blue"]
pub type RgFac1RbW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - green"]
    #[inline(always)]
    pub fn rg_fac_1_g(&self) -> RgFac1GR {
        RgFac1GR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - red/blue"]
    #[inline(always)]
    pub fn rg_fac_1_rb(&self) -> RgFac1RbR {
        RgFac1RbR::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - green"]
    #[inline(always)]
    #[must_use]
    pub fn rg_fac_1_g(&mut self) -> RgFac1GW<DpccRgFac1Spec> {
        RgFac1GW::new(self, 0)
    }
    #[doc = "Bits 8:13 - red/blue"]
    #[inline(always)]
    #[must_use]
    pub fn rg_fac_1_rb(&mut self) -> RgFac1RbW<DpccRgFac1Spec> {
        RgFac1RbW::new(self, 8)
    }
}
#[doc = "Rank gradient factor for set 1\n\nNote: all values are unsigned integer \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpcc_rg_fac_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpcc_rg_fac_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpccRgFac1Spec;
impl crate::RegisterSpec for DpccRgFac1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpcc_rg_fac_1::R`](R) reader structure"]
impl crate::Readable for DpccRgFac1Spec {}
#[doc = "`write(|w| ..)` method takes [`dpcc_rg_fac_1::W`](W) writer structure"]
impl crate::Writable for DpccRgFac1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DPCC_RG_FAC_1 to value 0"]
impl crate::Resettable for DpccRgFac1Spec {
    const RESET_VALUE: u32 = 0;
}
