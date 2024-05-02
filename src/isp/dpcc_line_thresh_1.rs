#[doc = "Register `DPCC_LINE_THRESH_1` reader"]
pub type R = crate::R<DpccLineThresh1Spec>;
#[doc = "Register `DPCC_LINE_THRESH_1` writer"]
pub type W = crate::W<DpccLineThresh1Spec>;
#[doc = "Field `LINE_THR_1_G` reader - line threshold for set 1 green"]
pub type LineThr1GR = crate::FieldReader;
#[doc = "Field `LINE_THR_1_G` writer - line threshold for set 1 green"]
pub type LineThr1GW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LINE_THR_1_RB` reader - line threshold for set 1 red/blue"]
pub type LineThr1RbR = crate::FieldReader;
#[doc = "Field `LINE_THR_1_RB` writer - line threshold for set 1 red/blue"]
pub type LineThr1RbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - line threshold for set 1 green"]
    #[inline(always)]
    pub fn line_thr_1_g(&self) -> LineThr1GR {
        LineThr1GR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - line threshold for set 1 red/blue"]
    #[inline(always)]
    pub fn line_thr_1_rb(&self) -> LineThr1RbR {
        LineThr1RbR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - line threshold for set 1 green"]
    #[inline(always)]
    #[must_use]
    pub fn line_thr_1_g(&mut self) -> LineThr1GW<DpccLineThresh1Spec> {
        LineThr1GW::new(self, 0)
    }
    #[doc = "Bits 8:15 - line threshold for set 1 red/blue"]
    #[inline(always)]
    #[must_use]
    pub fn line_thr_1_rb(&mut self) -> LineThr1RbW<DpccLineThresh1Spec> {
        LineThr1RbW::new(self, 8)
    }
}
#[doc = "Line threshold SET_1\n\nNote: all values are unsigned integer \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpcc_line_thresh_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpcc_line_thresh_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpccLineThresh1Spec;
impl crate::RegisterSpec for DpccLineThresh1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpcc_line_thresh_1::R`](R) reader structure"]
impl crate::Readable for DpccLineThresh1Spec {}
#[doc = "`write(|w| ..)` method takes [`dpcc_line_thresh_1::W`](W) writer structure"]
impl crate::Writable for DpccLineThresh1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DPCC_LINE_THRESH_1 to value 0"]
impl crate::Resettable for DpccLineThresh1Spec {
    const RESET_VALUE: u32 = 0;
}
