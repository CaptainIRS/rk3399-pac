#[doc = "Register `DPCC_LINE_THRESH_3` reader"]
pub type R = crate::R<DpccLineThresh3Spec>;
#[doc = "Register `DPCC_LINE_THRESH_3` writer"]
pub type W = crate::W<DpccLineThresh3Spec>;
#[doc = "Field `LINE_THR_3_G` reader - line threshold for set 3 green"]
pub type LineThr3GR = crate::FieldReader;
#[doc = "Field `LINE_THR_3_G` writer - line threshold for set 3 green"]
pub type LineThr3GW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LINE_THR_3_RB` reader - line threshold for set 3 red/blue"]
pub type LineThr3RbR = crate::FieldReader;
#[doc = "Field `LINE_THR_3_RB` writer - line threshold for set 3 red/blue"]
pub type LineThr3RbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - line threshold for set 3 green"]
    #[inline(always)]
    pub fn line_thr_3_g(&self) -> LineThr3GR {
        LineThr3GR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - line threshold for set 3 red/blue"]
    #[inline(always)]
    pub fn line_thr_3_rb(&self) -> LineThr3RbR {
        LineThr3RbR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - line threshold for set 3 green"]
    #[inline(always)]
    #[must_use]
    pub fn line_thr_3_g(&mut self) -> LineThr3GW<DpccLineThresh3Spec> {
        LineThr3GW::new(self, 0)
    }
    #[doc = "Bits 8:15 - line threshold for set 3 red/blue"]
    #[inline(always)]
    #[must_use]
    pub fn line_thr_3_rb(&mut self) -> LineThr3RbW<DpccLineThresh3Spec> {
        LineThr3RbW::new(self, 8)
    }
}
#[doc = "Line threshold set 3\n\nNote: all values are unsigned integer \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpcc_line_thresh_3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpcc_line_thresh_3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpccLineThresh3Spec;
impl crate::RegisterSpec for DpccLineThresh3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpcc_line_thresh_3::R`](R) reader structure"]
impl crate::Readable for DpccLineThresh3Spec {}
#[doc = "`write(|w| ..)` method takes [`dpcc_line_thresh_3::W`](W) writer structure"]
impl crate::Writable for DpccLineThresh3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DPCC_LINE_THRESH_3 to value 0"]
impl crate::Resettable for DpccLineThresh3Spec {
    const RESET_VALUE: u32 = 0;
}
