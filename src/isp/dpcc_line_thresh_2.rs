#[doc = "Register `DPCC_LINE_THRESH_2` reader"]
pub type R = crate::R<DpccLineThresh2Spec>;
#[doc = "Register `DPCC_LINE_THRESH_2` writer"]
pub type W = crate::W<DpccLineThresh2Spec>;
#[doc = "Field `LINE_THR_2_G` reader - line threshold for set 2 green"]
pub type LineThr2GR = crate::FieldReader;
#[doc = "Field `LINE_THR_2_G` writer - line threshold for set 2 green"]
pub type LineThr2GW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LINE_THR_2_RB` reader - line threshold for set 2 red/blue"]
pub type LineThr2RbR = crate::FieldReader;
#[doc = "Field `LINE_THR_2_RB` writer - line threshold for set 2 red/blue"]
pub type LineThr2RbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - line threshold for set 2 green"]
    #[inline(always)]
    pub fn line_thr_2_g(&self) -> LineThr2GR {
        LineThr2GR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - line threshold for set 2 red/blue"]
    #[inline(always)]
    pub fn line_thr_2_rb(&self) -> LineThr2RbR {
        LineThr2RbR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - line threshold for set 2 green"]
    #[inline(always)]
    #[must_use]
    pub fn line_thr_2_g(&mut self) -> LineThr2GW<DpccLineThresh2Spec> {
        LineThr2GW::new(self, 0)
    }
    #[doc = "Bits 8:15 - line threshold for set 2 red/blue"]
    #[inline(always)]
    #[must_use]
    pub fn line_thr_2_rb(&mut self) -> LineThr2RbW<DpccLineThresh2Spec> {
        LineThr2RbW::new(self, 8)
    }
}
#[doc = "Line threshold set 2\n\nNote: all values are unsigned integer \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpcc_line_thresh_2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpcc_line_thresh_2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpccLineThresh2Spec;
impl crate::RegisterSpec for DpccLineThresh2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpcc_line_thresh_2::R`](R) reader structure"]
impl crate::Readable for DpccLineThresh2Spec {}
#[doc = "`write(|w| ..)` method takes [`dpcc_line_thresh_2::W`](W) writer structure"]
impl crate::Writable for DpccLineThresh2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DPCC_LINE_THRESH_2 to value 0"]
impl crate::Resettable for DpccLineThresh2Spec {
    const RESET_VALUE: u32 = 0;
}
