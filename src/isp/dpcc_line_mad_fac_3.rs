#[doc = "Register `DPCC_LINE_MAD_FAC_3` reader"]
pub type R = crate::R<DpccLineMadFac3Spec>;
#[doc = "Register `DPCC_LINE_MAD_FAC_3` writer"]
pub type W = crate::W<DpccLineMadFac3Spec>;
#[doc = "Field `LINE_MAD_FAC_3_G` reader - line MAD factor for set 3 green"]
pub type LineMadFac3GR = crate::FieldReader;
#[doc = "Field `LINE_MAD_FAC_3_G` writer - line MAD factor for set 3 green"]
pub type LineMadFac3GW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `LINE_MAD_FAC_3_RB` reader - line MAD factor for set 3 red/blue"]
pub type LineMadFac3RbR = crate::FieldReader;
#[doc = "Field `LINE_MAD_FAC_3_RB` writer - line MAD factor for set 3 red/blue"]
pub type LineMadFac3RbW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - line MAD factor for set 3 green"]
    #[inline(always)]
    pub fn line_mad_fac_3_g(&self) -> LineMadFac3GR {
        LineMadFac3GR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - line MAD factor for set 3 red/blue"]
    #[inline(always)]
    pub fn line_mad_fac_3_rb(&self) -> LineMadFac3RbR {
        LineMadFac3RbR::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - line MAD factor for set 3 green"]
    #[inline(always)]
    #[must_use]
    pub fn line_mad_fac_3_g(&mut self) -> LineMadFac3GW<DpccLineMadFac3Spec> {
        LineMadFac3GW::new(self, 0)
    }
    #[doc = "Bits 8:13 - line MAD factor for set 3 red/blue"]
    #[inline(always)]
    #[must_use]
    pub fn line_mad_fac_3_rb(&mut self) -> LineMadFac3RbW<DpccLineMadFac3Spec> {
        LineMadFac3RbW::new(self, 8)
    }
}
#[doc = "Mean Absolute Difference (MAD) factor for Line check set 3\n\nNote: all values are unsigned integer \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpcc_line_mad_fac_3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpcc_line_mad_fac_3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpccLineMadFac3Spec;
impl crate::RegisterSpec for DpccLineMadFac3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpcc_line_mad_fac_3::R`](R) reader structure"]
impl crate::Readable for DpccLineMadFac3Spec {}
#[doc = "`write(|w| ..)` method takes [`dpcc_line_mad_fac_3::W`](W) writer structure"]
impl crate::Writable for DpccLineMadFac3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DPCC_LINE_MAD_FAC_3 to value 0"]
impl crate::Resettable for DpccLineMadFac3Spec {
    const RESET_VALUE: u32 = 0;
}
