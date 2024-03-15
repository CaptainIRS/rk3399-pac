#[doc = "Register `SHADOW_REGISTER_HEADER_LOG_2` reader"]
pub type R = crate::R<ShadowRegisterHeaderLog2Spec>;
#[doc = "Register `SHADOW_REGISTER_HEADER_LOG_2` writer"]
pub type W = crate::W<ShadowRegisterHeaderLog2Spec>;
#[doc = "Field `SHDW_HDR_LOG_2` reader - Shadow header log 2 \\[SHDW_HDR_LOG_2\\]
The value here will be reflected in the target function's header log register when f/w sets any bit In the shadow error register. If the header log is already set in the function's AER space, the value here may not get written and a header log overflow bit would get set. This register holds \\[95:64\\]
value of the TLP header."]
pub type ShdwHdrLog2R = crate::FieldReader<u32>;
#[doc = "Field `SHDW_HDR_LOG_2` writer - Shadow header log 2 \\[SHDW_HDR_LOG_2\\]
The value here will be reflected in the target function's header log register when f/w sets any bit In the shadow error register. If the header log is already set in the function's AER space, the value here may not get written and a header log overflow bit would get set. This register holds \\[95:64\\]
value of the TLP header."]
pub type ShdwHdrLog2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Shadow header log 2 \\[SHDW_HDR_LOG_2\\]
The value here will be reflected in the target function's header log register when f/w sets any bit In the shadow error register. If the header log is already set in the function's AER space, the value here may not get written and a header log overflow bit would get set. This register holds \\[95:64\\]
value of the TLP header."]
    #[inline(always)]
    pub fn shdw_hdr_log_2(&self) -> ShdwHdrLog2R {
        ShdwHdrLog2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Shadow header log 2 \\[SHDW_HDR_LOG_2\\]
The value here will be reflected in the target function's header log register when f/w sets any bit In the shadow error register. If the header log is already set in the function's AER space, the value here may not get written and a header log overflow bit would get set. This register holds \\[95:64\\]
value of the TLP header."]
    #[inline(always)]
    #[must_use]
    pub fn shdw_hdr_log_2(&mut self) -> ShdwHdrLog2W<ShadowRegisterHeaderLog2Spec> {
        ShdwHdrLog2W::new(self, 0)
    }
}
#[doc = "Shadow register header log 2 The value here will be reflected in the target function's header log register when f/w sets any bit In the shadow error register. If the header log is already set in the function's AER space, the value here may not get written and a header log overflow bit would get set. This register holds \\[95:64\\]
value of the TLP header.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shadow_register_header_log_2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shadow_register_header_log_2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ShadowRegisterHeaderLog2Spec;
impl crate::RegisterSpec for ShadowRegisterHeaderLog2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shadow_register_header_log_2::R`](R) reader structure"]
impl crate::Readable for ShadowRegisterHeaderLog2Spec {}
#[doc = "`write(|w| ..)` method takes [`shadow_register_header_log_2::W`](W) writer structure"]
impl crate::Writable for ShadowRegisterHeaderLog2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SHADOW_REGISTER_HEADER_LOG_2 to value 0"]
impl crate::Resettable for ShadowRegisterHeaderLog2Spec {
    const RESET_VALUE: u32 = 0;
}
