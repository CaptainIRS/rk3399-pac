#[doc = "Register `PCIE_LM_SHADOW_REGISTER_HEADER_LOG_3` reader"]
pub type R = crate::R<PcieLmShadowRegisterHeaderLog3Spec>;
#[doc = "Register `PCIE_LM_SHADOW_REGISTER_HEADER_LOG_3` writer"]
pub type W = crate::W<PcieLmShadowRegisterHeaderLog3Spec>;
#[doc = "Field `SHDW_HDR_LOG_3` reader - Shadow header log 3 \\[SHDW_HDR_LOG_3\\]\n\nThe value here will be reflected in\n\nthe target function's header log\n\nregister when f/w sets any bit In\n\nthe shadow error register. If the\n\nheader log is already set in the\n\nfunction's AER space, the value\n\nhere may not get written and a\n\nheader log overflow bit would get\n\nset. This register holds \\[127:96\\]\n\nvalue of the TLP header."]
pub type ShdwHdrLog3R = crate::FieldReader<u32>;
#[doc = "Field `SHDW_HDR_LOG_3` writer - Shadow header log 3 \\[SHDW_HDR_LOG_3\\]\n\nThe value here will be reflected in\n\nthe target function's header log\n\nregister when f/w sets any bit In\n\nthe shadow error register. If the\n\nheader log is already set in the\n\nfunction's AER space, the value\n\nhere may not get written and a\n\nheader log overflow bit would get\n\nset. This register holds \\[127:96\\]\n\nvalue of the TLP header."]
pub type ShdwHdrLog3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Shadow header log 3 \\[SHDW_HDR_LOG_3\\]\n\nThe value here will be reflected in\n\nthe target function's header log\n\nregister when f/w sets any bit In\n\nthe shadow error register. If the\n\nheader log is already set in the\n\nfunction's AER space, the value\n\nhere may not get written and a\n\nheader log overflow bit would get\n\nset. This register holds \\[127:96\\]\n\nvalue of the TLP header."]
    #[inline(always)]
    pub fn shdw_hdr_log_3(&self) -> ShdwHdrLog3R {
        ShdwHdrLog3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Shadow header log 3 \\[SHDW_HDR_LOG_3\\]\n\nThe value here will be reflected in\n\nthe target function's header log\n\nregister when f/w sets any bit In\n\nthe shadow error register. If the\n\nheader log is already set in the\n\nfunction's AER space, the value\n\nhere may not get written and a\n\nheader log overflow bit would get\n\nset. This register holds \\[127:96\\]\n\nvalue of the TLP header."]
    #[inline(always)]
    #[must_use]
    pub fn shdw_hdr_log_3(&mut self) -> ShdwHdrLog3W<PcieLmShadowRegisterHeaderLog3Spec> {
        ShdwHdrLog3W::new(self, 0)
    }
}
#[doc = "Shadow register header log 3\n\nThe value here will be reflected in\n\nthe target function's header log\n\nregister when f/w sets any bit In\n\nthe shadow error register. If the\n\nheader log is already set in the\n\nfunction's AER space, the value\n\nhere may not get written and a\n\nheader log overflow bit would get\n\nset. This register holds \\[127:96\\]\n\nvalue of the TLP header.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_lm_shadow_register_header_log_3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_lm_shadow_register_header_log_3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieLmShadowRegisterHeaderLog3Spec;
impl crate::RegisterSpec for PcieLmShadowRegisterHeaderLog3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_lm_shadow_register_header_log_3::R`](R) reader structure"]
impl crate::Readable for PcieLmShadowRegisterHeaderLog3Spec {}
#[doc = "`write(|w| ..)` method takes [`pcie_lm_shadow_register_header_log_3::W`](W) writer structure"]
impl crate::Writable for PcieLmShadowRegisterHeaderLog3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCIE_LM_SHADOW_REGISTER_HEADER_LOG_3 to value 0"]
impl crate::Resettable for PcieLmShadowRegisterHeaderLog3Spec {
    const RESET_VALUE: u32 = 0;
}
