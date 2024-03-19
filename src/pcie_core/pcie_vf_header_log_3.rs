#[doc = "Register `PCIE_VF_HEADER_LOG_3` reader"]
pub type R = crate::R<PcieVfHeaderLog3Spec>;
#[doc = "Field `HD3` reader - Header DWORD 3 \\[HD3\\]
Fourth DWORD of captured TLP header STICKY."]
pub type Hd3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Header DWORD 3 \\[HD3\\]
Fourth DWORD of captured TLP header STICKY."]
    #[inline(always)]
    pub fn hd3(&self) -> Hd3R {
        Hd3R::new(self.bits)
    }
}
#[doc = "Header Log Register 3 Fourth DWORD of captured TLP header STICKY.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_vf_header_log_3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieVfHeaderLog3Spec;
impl crate::RegisterSpec for PcieVfHeaderLog3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_vf_header_log_3::R`](R) reader structure"]
impl crate::Readable for PcieVfHeaderLog3Spec {}
#[doc = "`reset()` method sets PCIE_VF_HEADER_LOG_3 to value 0"]
impl crate::Resettable for PcieVfHeaderLog3Spec {
    const RESET_VALUE: u32 = 0;
}
