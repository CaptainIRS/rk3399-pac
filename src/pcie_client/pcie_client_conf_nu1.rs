#[doc = "Register `PCIE_CLIENT_CONF_NU1` reader"]
pub type R = crate::R<PcieClientConfNu1Spec>;
#[doc = "Configuration no used\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_client_conf_nu1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieClientConfNu1Spec;
impl crate::RegisterSpec for PcieClientConfNu1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_client_conf_nu1::R`](R) reader structure"]
impl crate::Readable for PcieClientConfNu1Spec {}
#[doc = "`reset()` method sets PCIE_CLIENT_CONF_NU1 to value 0"]
impl crate::Resettable for PcieClientConfNu1Spec {
    const RESET_VALUE: u32 = 0;
}
