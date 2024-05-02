#[doc = "Register `DPCC_LINE_MAD_FAC_2` reader"]
pub type R = crate::R<DpccLineMadFac2Spec>;
#[doc = "Register `DPCC_LINE_MAD_FAC_2` writer"]
pub type W = crate::W<DpccLineMadFac2Spec>;
#[doc = "Field `LINE_MAD_FAC_2_G` reader - line MAD factor for set 2 green"]
pub type LineMadFac2GR = crate::FieldReader;
#[doc = "Field `LINE_MAD_FAC_2_G` writer - line MAD factor for set 2 green"]
pub type LineMadFac2GW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `LINE_MAD_FAC_2_RB` reader - line MAD factor for set 2 red/blue"]
pub type LineMadFac2RbR = crate::FieldReader;
#[doc = "Field `LINE_MAD_FAC_2_RB` writer - line MAD factor for set 2 red/blue"]
pub type LineMadFac2RbW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - line MAD factor for set 2 green"]
    #[inline(always)]
    pub fn line_mad_fac_2_g(&self) -> LineMadFac2GR {
        LineMadFac2GR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - line MAD factor for set 2 red/blue"]
    #[inline(always)]
    pub fn line_mad_fac_2_rb(&self) -> LineMadFac2RbR {
        LineMadFac2RbR::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - line MAD factor for set 2 green"]
    #[inline(always)]
    #[must_use]
    pub fn line_mad_fac_2_g(&mut self) -> LineMadFac2GW<DpccLineMadFac2Spec> {
        LineMadFac2GW::new(self, 0)
    }
    #[doc = "Bits 8:13 - line MAD factor for set 2 red/blue"]
    #[inline(always)]
    #[must_use]
    pub fn line_mad_fac_2_rb(&mut self) -> LineMadFac2RbW<DpccLineMadFac2Spec> {
        LineMadFac2RbW::new(self, 8)
    }
}
#[doc = "Mean Absolute Difference (MAD) factor for Line check set 2\n\nNote: all values are unsigned integer \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpcc_line_mad_fac_2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpcc_line_mad_fac_2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpccLineMadFac2Spec;
impl crate::RegisterSpec for DpccLineMadFac2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpcc_line_mad_fac_2::R`](R) reader structure"]
impl crate::Readable for DpccLineMadFac2Spec {}
#[doc = "`write(|w| ..)` method takes [`dpcc_line_mad_fac_2::W`](W) writer structure"]
impl crate::Writable for DpccLineMadFac2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DPCC_LINE_MAD_FAC_2 to value 0"]
impl crate::Resettable for DpccLineMadFac2Spec {
    const RESET_VALUE: u32 = 0;
}
