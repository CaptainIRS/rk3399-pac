#[doc = "Register `PCIE_LM_SHADOW_REGISTER_HEADER_LOG_0` reader"]
pub type R = crate::R<PcieLmShadowRegisterHeaderLog0Spec>;
#[doc = "Register `PCIE_LM_SHADOW_REGISTER_HEADER_LOG_0` writer"]
pub type W = crate::W<PcieLmShadowRegisterHeaderLog0Spec>;
#[doc = "Field `SHDW_HDR_LOG_0` reader - Shadow header log 0 \\[SHDW_HDR_LOG_0\\]
The value here will be reflected in the target function's header log register when f/w sets any bit In the shadow error register. If the header log is already set in the function's AER space, the value here may not get written and a header log overflow bit would get set. This register holds \\[31:0\\]
value of the TLP header."]
pub type ShdwHdrLog0R = crate::FieldReader<u32>;
#[doc = "Field `SHDW_HDR_LOG_0` writer - Shadow header log 0 \\[SHDW_HDR_LOG_0\\]
The value here will be reflected in the target function's header log register when f/w sets any bit In the shadow error register. If the header log is already set in the function's AER space, the value here may not get written and a header log overflow bit would get set. This register holds \\[31:0\\]
value of the TLP header."]
pub type ShdwHdrLog0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Shadow header log 0 \\[SHDW_HDR_LOG_0\\]
The value here will be reflected in the target function's header log register when f/w sets any bit In the shadow error register. If the header log is already set in the function's AER space, the value here may not get written and a header log overflow bit would get set. This register holds \\[31:0\\]
value of the TLP header."]
    #[inline(always)]
    pub fn shdw_hdr_log_0(&self) -> ShdwHdrLog0R {
        ShdwHdrLog0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Shadow header log 0 \\[SHDW_HDR_LOG_0\\]
The value here will be reflected in the target function's header log register when f/w sets any bit In the shadow error register. If the header log is already set in the function's AER space, the value here may not get written and a header log overflow bit would get set. This register holds \\[31:0\\]
value of the TLP header."]
    #[inline(always)]
    #[must_use]
    pub fn shdw_hdr_log_0(&mut self) -> ShdwHdrLog0W<PcieLmShadowRegisterHeaderLog0Spec> {
        ShdwHdrLog0W::new(self, 0)
    }
}
#[doc = "Shadow register header log 0 The value here will be reflected in the target function's header log register when f/w sets any bit In the shadow error register. If the header log is already set in the function's AER space, the value here may not get written and a header log overflow bit would get set. This register holds \\[31:0\\]
value of the TLP header.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_lm_shadow_register_header_log_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_lm_shadow_register_header_log_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieLmShadowRegisterHeaderLog0Spec;
impl crate::RegisterSpec for PcieLmShadowRegisterHeaderLog0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_lm_shadow_register_header_log_0::R`](R) reader structure"]
impl crate::Readable for PcieLmShadowRegisterHeaderLog0Spec {}
#[doc = "`write(|w| ..)` method takes [`pcie_lm_shadow_register_header_log_0::W`](W) writer structure"]
impl crate::Writable for PcieLmShadowRegisterHeaderLog0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCIE_LM_SHADOW_REGISTER_HEADER_LOG_0 to value 0"]
impl crate::Resettable for PcieLmShadowRegisterHeaderLog0Spec {
    const RESET_VALUE: u32 = 0;
}
