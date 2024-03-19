#[doc = "Register `PCIE_RC_HEADER_LOG_0` reader"]
pub type R = crate::R<PcieRcHeaderLog0Spec>;
#[doc = "Field `HD0` reader - Header Dword 0 \\[HD0\\]\n\nFirst Dword of captured TLP header.\n\nSTICKY."]
pub type Hd0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Header Dword 0 \\[HD0\\]\n\nFirst Dword of captured TLP header.\n\nSTICKY."]
    #[inline(always)]
    pub fn hd0(&self) -> Hd0R {
        Hd0R::new(self.bits)
    }
}
#[doc = "Header Log Register 0\n\nFirst Dword of captured TLP header.\n\nSTICKY.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_header_log_0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieRcHeaderLog0Spec;
impl crate::RegisterSpec for PcieRcHeaderLog0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_rc_header_log_0::R`](R) reader structure"]
impl crate::Readable for PcieRcHeaderLog0Spec {}
#[doc = "`reset()` method sets PCIE_RC_HEADER_LOG_0 to value 0"]
impl crate::Resettable for PcieRcHeaderLog0Spec {
    const RESET_VALUE: u32 = 0;
}
