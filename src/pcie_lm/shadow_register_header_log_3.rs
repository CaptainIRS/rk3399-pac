#[doc = "Register `SHADOW_REGISTER_HEADER_LOG_3` reader"]
pub type R = crate::R<ShadowRegisterHeaderLog3Spec>;
#[doc = "Register `SHADOW_REGISTER_HEADER_LOG_3` writer"]
pub type W = crate::W<ShadowRegisterHeaderLog3Spec>;
#[doc = "Field `SHDW_HDR_LOG_3` reader - Shadow header log 3 \\[SHDW_HDR_LOG_3\\]
The value here will be reflected in the target function's header log register when f/w sets any bit In the shadow error register. If the header log is already set in the function's AER space, the value here may not get written and a header log overflow bit would get set. This register holds \\[127:96\\]
value of the TLP header."]
pub type ShdwHdrLog3R = crate::FieldReader<u32>;
#[doc = "Field `SHDW_HDR_LOG_3` writer - Shadow header log 3 \\[SHDW_HDR_LOG_3\\]
The value here will be reflected in the target function's header log register when f/w sets any bit In the shadow error register. If the header log is already set in the function's AER space, the value here may not get written and a header log overflow bit would get set. This register holds \\[127:96\\]
value of the TLP header."]
pub type ShdwHdrLog3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Shadow header log 3 \\[SHDW_HDR_LOG_3\\]
The value here will be reflected in the target function's header log register when f/w sets any bit In the shadow error register. If the header log is already set in the function's AER space, the value here may not get written and a header log overflow bit would get set. This register holds \\[127:96\\]
value of the TLP header."]
    #[inline(always)]
    pub fn shdw_hdr_log_3(&self) -> ShdwHdrLog3R {
        ShdwHdrLog3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Shadow header log 3 \\[SHDW_HDR_LOG_3\\]
The value here will be reflected in the target function's header log register when f/w sets any bit In the shadow error register. If the header log is already set in the function's AER space, the value here may not get written and a header log overflow bit would get set. This register holds \\[127:96\\]
value of the TLP header."]
    #[inline(always)]
    #[must_use]
    pub fn shdw_hdr_log_3(&mut self) -> ShdwHdrLog3W<ShadowRegisterHeaderLog3Spec> {
        ShdwHdrLog3W::new(self, 0)
    }
}
#[doc = "Shadow register header log 3 The value here will be reflected in the target function's header log register when f/w sets any bit In the shadow error register. If the header log is already set in the function's AER space, the value here may not get written and a header log overflow bit would get set. This register holds \\[127:96\\]
value of the TLP header.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shadow_register_header_log_3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shadow_register_header_log_3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ShadowRegisterHeaderLog3Spec;
impl crate::RegisterSpec for ShadowRegisterHeaderLog3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shadow_register_header_log_3::R`](R) reader structure"]
impl crate::Readable for ShadowRegisterHeaderLog3Spec {}
#[doc = "`write(|w| ..)` method takes [`shadow_register_header_log_3::W`](W) writer structure"]
impl crate::Writable for ShadowRegisterHeaderLog3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SHADOW_REGISTER_HEADER_LOG_3 to value 0"]
impl crate::Resettable for ShadowRegisterHeaderLog3Spec {
    const RESET_VALUE: u32 = 0;
}
