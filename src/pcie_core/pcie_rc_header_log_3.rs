#[doc = "Register `PCIE_RC_HEADER_LOG_3` reader"]
pub type R = crate::R<PcieRcHeaderLog3Spec>;
#[doc = "Field `HD3` reader - Header Dword 3 \\[HD3\\]\n\nFourth Dword of captured TLP\n\nheader. STICKY."]
pub type Hd3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Header Dword 3 \\[HD3\\]\n\nFourth Dword of captured TLP\n\nheader. STICKY."]
    #[inline(always)]
    pub fn hd3(&self) -> Hd3R {
        Hd3R::new(self.bits)
    }
}
#[doc = "Header Log Register 3\n\nFourth Dword of captured TLP\n\nheader. STICKY.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_header_log_3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieRcHeaderLog3Spec;
impl crate::RegisterSpec for PcieRcHeaderLog3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_rc_header_log_3::R`](R) reader structure"]
impl crate::Readable for PcieRcHeaderLog3Spec {}
#[doc = "`reset()` method sets PCIE_RC_HEADER_LOG_3 to value 0"]
impl crate::Resettable for PcieRcHeaderLog3Spec {
    const RESET_VALUE: u32 = 0;
}
