#[doc = "Register `PCIE_RC_HEADER_LOG_1` reader"]
pub type R = crate::R<PcieRcHeaderLog1Spec>;
#[doc = "Field `HD1` reader - Header Dword 1 \\[HD1\\]
Second Dword of captured TLP header. STICKY."]
pub type Hd1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Header Dword 1 \\[HD1\\]
Second Dword of captured TLP header. STICKY."]
    #[inline(always)]
    pub fn hd1(&self) -> Hd1R {
        Hd1R::new(self.bits)
    }
}
#[doc = "Header Log Register 1 Second Dword of captured TLP header. STICKY.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_header_log_1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieRcHeaderLog1Spec;
impl crate::RegisterSpec for PcieRcHeaderLog1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_rc_header_log_1::R`](R) reader structure"]
impl crate::Readable for PcieRcHeaderLog1Spec {}
#[doc = "`reset()` method sets PCIE_RC_HEADER_LOG_1 to value 0"]
impl crate::Resettable for PcieRcHeaderLog1Spec {
    const RESET_VALUE: u32 = 0;
}
