#[doc = "Register `PCIE_RC_HEADER_LOG_2` reader"]
pub type R = crate::R<PcieRcHeaderLog2Spec>;
#[doc = "Field `HD2` reader - Header Dword 2 \\[HD2\\]
Third Dword of captured TLP header. STICKY."]
pub type Hd2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Header Dword 2 \\[HD2\\]
Third Dword of captured TLP header. STICKY."]
    #[inline(always)]
    pub fn hd2(&self) -> Hd2R {
        Hd2R::new(self.bits)
    }
}
#[doc = "Header Log Register 2 Third Dword of captured TLP header. STICKY.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_header_log_2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieRcHeaderLog2Spec;
impl crate::RegisterSpec for PcieRcHeaderLog2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_rc_header_log_2::R`](R) reader structure"]
impl crate::Readable for PcieRcHeaderLog2Spec {}
#[doc = "`reset()` method sets PCIE_RC_HEADER_LOG_2 to value 0"]
impl crate::Resettable for PcieRcHeaderLog2Spec {
    const RESET_VALUE: u32 = 0;
}
