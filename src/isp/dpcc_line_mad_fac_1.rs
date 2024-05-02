#[doc = "Register `DPCC_LINE_MAD_FAC_1` reader"]
pub type R = crate::R<DpccLineMadFac1Spec>;
#[doc = "Register `DPCC_LINE_MAD_FAC_1` writer"]
pub type W = crate::W<DpccLineMadFac1Spec>;
#[doc = "Field `LINE_MAD_FAC_1_G` reader - line MAD factor for set 1 green"]
pub type LineMadFac1GR = crate::FieldReader;
#[doc = "Field `LINE_MAD_FAC_1_G` writer - line MAD factor for set 1 green"]
pub type LineMadFac1GW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `LINE_MAD_FAC_1_RB` reader - line MAD factor for set 1 red/blue"]
pub type LineMadFac1RbR = crate::FieldReader;
#[doc = "Field `LINE_MAD_FAC_1_RB` writer - line MAD factor for set 1 red/blue"]
pub type LineMadFac1RbW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - line MAD factor for set 1 green"]
    #[inline(always)]
    pub fn line_mad_fac_1_g(&self) -> LineMadFac1GR {
        LineMadFac1GR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - line MAD factor for set 1 red/blue"]
    #[inline(always)]
    pub fn line_mad_fac_1_rb(&self) -> LineMadFac1RbR {
        LineMadFac1RbR::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - line MAD factor for set 1 green"]
    #[inline(always)]
    #[must_use]
    pub fn line_mad_fac_1_g(&mut self) -> LineMadFac1GW<DpccLineMadFac1Spec> {
        LineMadFac1GW::new(self, 0)
    }
    #[doc = "Bits 8:13 - line MAD factor for set 1 red/blue"]
    #[inline(always)]
    #[must_use]
    pub fn line_mad_fac_1_rb(&mut self) -> LineMadFac1RbW<DpccLineMadFac1Spec> {
        LineMadFac1RbW::new(self, 8)
    }
}
#[doc = "Mean Absolute Difference (MAD) factor for Line check set 1\n\nNote: all values are unsigned integer \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpcc_line_mad_fac_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpcc_line_mad_fac_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpccLineMadFac1Spec;
impl crate::RegisterSpec for DpccLineMadFac1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpcc_line_mad_fac_1::R`](R) reader structure"]
impl crate::Readable for DpccLineMadFac1Spec {}
#[doc = "`write(|w| ..)` method takes [`dpcc_line_mad_fac_1::W`](W) writer structure"]
impl crate::Writable for DpccLineMadFac1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DPCC_LINE_MAD_FAC_1 to value 0"]
impl crate::Resettable for DpccLineMadFac1Spec {
    const RESET_VALUE: u32 = 0;
}
