#[doc = "Register `PHY_TST2` reader"]
pub type R = crate::R<PhyTst2Spec>;
#[doc = "Field `TESTDOUT` reader - Test Data output. Otherwise, this field is a \"spare\"\n\nbit with no associated functionality."]
pub type TestdoutR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Test Data output. Otherwise, this field is a \"spare\"\n\nbit with no associated functionality."]
    #[inline(always)]
    pub fn testdout(&self) -> TestdoutR {
        TestdoutR::new(self.bits)
    }
}
#[doc = "PHY Test Interface Register 2\n\nPHY TX mapped text interface (data out).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_tst2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PhyTst2Spec;
impl crate::RegisterSpec for PhyTst2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`phy_tst2::R`](R) reader structure"]
impl crate::Readable for PhyTst2Spec {}
#[doc = "`reset()` method sets PHY_TST2 to value 0"]
impl crate::Resettable for PhyTst2Spec {
    const RESET_VALUE: u8 = 0;
}
