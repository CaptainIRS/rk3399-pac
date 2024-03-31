#[doc = "Register `SOC_STATUS1` reader"]
pub type R = crate::R<SocStatus1Spec>;
#[doc = "Register `SOC_STATUS1` writer"]
pub type W = crate::W<SocStatus1Spec>;
#[doc = "Field `DPHY_RX0_TESTDOUT` reader - status bit of dphy_rx0_testdout"]
pub type DphyRx0TestdoutR = crate::FieldReader;
#[doc = "Field `DPHY_RX0_TESTDOUT` writer - status bit of dphy_rx0_testdout"]
pub type DphyRx0TestdoutW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GRF_PCIE_TEST_O` reader - status bit of grf_pcie_test_o"]
pub type GrfPcieTestOR = crate::FieldReader;
#[doc = "Field `GRF_PCIE_TEST_O` writer - status bit of grf_pcie_test_o"]
pub type GrfPcieTestOW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:7 - status bit of dphy_rx0_testdout"]
    #[inline(always)]
    pub fn dphy_rx0_testdout(&self) -> DphyRx0TestdoutR {
        DphyRx0TestdoutR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - status bit of grf_pcie_test_o"]
    #[inline(always)]
    pub fn grf_pcie_test_o(&self) -> GrfPcieTestOR {
        GrfPcieTestOR::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - status bit of dphy_rx0_testdout"]
    #[inline(always)]
    #[must_use]
    pub fn dphy_rx0_testdout(&mut self) -> DphyRx0TestdoutW<SocStatus1Spec> {
        DphyRx0TestdoutW::new(self, 0)
    }
    #[doc = "Bits 8:11 - status bit of grf_pcie_test_o"]
    #[inline(always)]
    #[must_use]
    pub fn grf_pcie_test_o(&mut self) -> GrfPcieTestOW<SocStatus1Spec> {
        GrfPcieTestOW::new(self, 8)
    }
}
#[doc = "SOC status register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`soc_status1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`soc_status1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SocStatus1Spec;
impl crate::RegisterSpec for SocStatus1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`soc_status1::R`](R) reader structure"]
impl crate::Readable for SocStatus1Spec {}
#[doc = "`write(|w| ..)` method takes [`soc_status1::W`](W) writer structure"]
impl crate::Writable for SocStatus1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SOC_STATUS1 to value 0"]
impl crate::Resettable for SocStatus1Spec {
    const RESET_VALUE: u32 = 0;
}
