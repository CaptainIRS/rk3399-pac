#[doc = "Register `PHY_TST1` reader"]
pub type R = crate::R<PhyTst1Spec>;
#[doc = "Register `PHY_TST1` writer"]
pub type W = crate::W<PhyTst1Spec>;
#[doc = "Field `TESTDIN` reader - Test Data input Otherwise, this field is a \"spare\" bit\n\nwith no associated functionality."]
pub type TestdinR = crate::FieldReader;
#[doc = "Field `TESTDIN` writer - Test Data input Otherwise, this field is a \"spare\" bit\n\nwith no associated functionality."]
pub type TestdinW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Test Data input Otherwise, this field is a \"spare\" bit\n\nwith no associated functionality."]
    #[inline(always)]
    pub fn testdin(&self) -> TestdinR {
        TestdinR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Test Data input Otherwise, this field is a \"spare\" bit\n\nwith no associated functionality."]
    #[inline(always)]
    #[must_use]
    pub fn testdin(&mut self) -> TestdinW<PhyTst1Spec> {
        TestdinW::new(self, 0)
    }
}
#[doc = "PHY Test Interface Register 1\n\nPHY TX mapped text interface (data in).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_tst1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_tst1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PhyTst1Spec;
impl crate::RegisterSpec for PhyTst1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`phy_tst1::R`](R) reader structure"]
impl crate::Readable for PhyTst1Spec {}
#[doc = "`write(|w| ..)` method takes [`phy_tst1::W`](W) writer structure"]
impl crate::Writable for PhyTst1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PHY_TST1 to value 0"]
impl crate::Resettable for PhyTst1Spec {
    const RESET_VALUE: u8 = 0;
}
