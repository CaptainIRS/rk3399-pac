#[doc = "Register `DPCC_RG_FAC_2` reader"]
pub type R = crate::R<DpccRgFac2Spec>;
#[doc = "Register `DPCC_RG_FAC_2` writer"]
pub type W = crate::W<DpccRgFac2Spec>;
#[doc = "Field `RG_FAC_2_G` reader - green"]
pub type RgFac2GR = crate::FieldReader;
#[doc = "Field `RG_FAC_2_G` writer - green"]
pub type RgFac2GW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RG_FAC_2_RB` reader - red/blue"]
pub type RgFac2RbR = crate::FieldReader;
#[doc = "Field `RG_FAC_2_RB` writer - red/blue"]
pub type RgFac2RbW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - green"]
    #[inline(always)]
    pub fn rg_fac_2_g(&self) -> RgFac2GR {
        RgFac2GR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - red/blue"]
    #[inline(always)]
    pub fn rg_fac_2_rb(&self) -> RgFac2RbR {
        RgFac2RbR::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - green"]
    #[inline(always)]
    #[must_use]
    pub fn rg_fac_2_g(&mut self) -> RgFac2GW<DpccRgFac2Spec> {
        RgFac2GW::new(self, 0)
    }
    #[doc = "Bits 8:13 - red/blue"]
    #[inline(always)]
    #[must_use]
    pub fn rg_fac_2_rb(&mut self) -> RgFac2RbW<DpccRgFac2Spec> {
        RgFac2RbW::new(self, 8)
    }
}
#[doc = "Rank gradient factor for set 2\n\nNote: all values are unsigned integer \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpcc_rg_fac_2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpcc_rg_fac_2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpccRgFac2Spec;
impl crate::RegisterSpec for DpccRgFac2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpcc_rg_fac_2::R`](R) reader structure"]
impl crate::Readable for DpccRgFac2Spec {}
#[doc = "`write(|w| ..)` method takes [`dpcc_rg_fac_2::W`](W) writer structure"]
impl crate::Writable for DpccRgFac2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DPCC_RG_FAC_2 to value 0"]
impl crate::Resettable for DpccRgFac2Spec {
    const RESET_VALUE: u32 = 0;
}
