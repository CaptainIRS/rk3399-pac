#[doc = "Register `DPCC_RG_FAC_3` reader"]
pub type R = crate::R<DpccRgFac3Spec>;
#[doc = "Register `DPCC_RG_FAC_3` writer"]
pub type W = crate::W<DpccRgFac3Spec>;
#[doc = "Field `RG_FAC_3_G` reader - green"]
pub type RgFac3GR = crate::FieldReader;
#[doc = "Field `RG_FAC_3_G` writer - green"]
pub type RgFac3GW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RG_FAC_3_RB` reader - red/blue"]
pub type RgFac3RbR = crate::FieldReader;
#[doc = "Field `RG_FAC_3_RB` writer - red/blue"]
pub type RgFac3RbW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - green"]
    #[inline(always)]
    pub fn rg_fac_3_g(&self) -> RgFac3GR {
        RgFac3GR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - red/blue"]
    #[inline(always)]
    pub fn rg_fac_3_rb(&self) -> RgFac3RbR {
        RgFac3RbR::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - green"]
    #[inline(always)]
    #[must_use]
    pub fn rg_fac_3_g(&mut self) -> RgFac3GW<DpccRgFac3Spec> {
        RgFac3GW::new(self, 0)
    }
    #[doc = "Bits 8:13 - red/blue"]
    #[inline(always)]
    #[must_use]
    pub fn rg_fac_3_rb(&mut self) -> RgFac3RbW<DpccRgFac3Spec> {
        RgFac3RbW::new(self, 8)
    }
}
#[doc = "Rank gradient factor for set 3\n\nNote: all values are unsigned integer \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpcc_rg_fac_3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpcc_rg_fac_3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpccRgFac3Spec;
impl crate::RegisterSpec for DpccRgFac3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpcc_rg_fac_3::R`](R) reader structure"]
impl crate::Readable for DpccRgFac3Spec {}
#[doc = "`write(|w| ..)` method takes [`dpcc_rg_fac_3::W`](W) writer structure"]
impl crate::Writable for DpccRgFac3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DPCC_RG_FAC_3 to value 0"]
impl crate::Resettable for DpccRgFac3Spec {
    const RESET_VALUE: u32 = 0;
}
