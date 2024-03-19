#[doc = "Register `PHY_TST0` reader"]
pub type R = crate::R<PhyTst0Spec>;
#[doc = "Register `PHY_TST0` writer"]
pub type W = crate::W<PhyTst0Spec>;
#[doc = "Field `TESTCLK` reader - Test Clock signal. Otherwise, this field is a \"spare\"\n\nbit with no associated functionality."]
pub type TestclkR = crate::BitReader;
#[doc = "Field `TESTCLK` writer - Test Clock signal. Otherwise, this field is a \"spare\"\n\nbit with no associated functionality."]
pub type TestclkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPARE_1` reader - Reserved as \"spare\" bit with no associated\n\nfunctionality."]
pub type Spare1R = crate::FieldReader;
#[doc = "Field `SPARE_1` writer - Reserved as \"spare\" bit with no associated\n\nfunctionality."]
pub type Spare1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TESTEN` reader - Test Enable signal. Otherwise, this field is a \"spare\"\n\nbit with no associated functionality."]
pub type TestenR = crate::BitReader;
#[doc = "Field `TESTEN` writer - Test Enable signal. Otherwise, this field is a \"spare\"\n\nbit with no associated functionality."]
pub type TestenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TESTCLR` reader - Test Clear signal. Otherwise, this field is a \"spare\"\n\nbit with no associated functionality."]
pub type TestclrR = crate::BitReader;
#[doc = "Field `TESTCLR` writer - Test Clear signal. Otherwise, this field is a \"spare\"\n\nbit with no associated functionality."]
pub type TestclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPARE_2` reader - Reserved as \"spare\" bit with no associated\n\nfunctionality."]
pub type Spare2R = crate::FieldReader;
#[doc = "Field `SPARE_2` writer - Reserved as \"spare\" bit with no associated\n\nfunctionality."]
pub type Spare2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Test Clock signal. Otherwise, this field is a \"spare\"\n\nbit with no associated functionality."]
    #[inline(always)]
    pub fn testclk(&self) -> TestclkR {
        TestclkR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Reserved as \"spare\" bit with no associated\n\nfunctionality."]
    #[inline(always)]
    pub fn spare_1(&self) -> Spare1R {
        Spare1R::new((self.bits >> 1) & 7)
    }
    #[doc = "Bit 4 - Test Enable signal. Otherwise, this field is a \"spare\"\n\nbit with no associated functionality."]
    #[inline(always)]
    pub fn testen(&self) -> TestenR {
        TestenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Test Clear signal. Otherwise, this field is a \"spare\"\n\nbit with no associated functionality."]
    #[inline(always)]
    pub fn testclr(&self) -> TestclrR {
        TestclrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Reserved as \"spare\" bit with no associated\n\nfunctionality."]
    #[inline(always)]
    pub fn spare_2(&self) -> Spare2R {
        Spare2R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bit 0 - Test Clock signal. Otherwise, this field is a \"spare\"\n\nbit with no associated functionality."]
    #[inline(always)]
    #[must_use]
    pub fn testclk(&mut self) -> TestclkW<PhyTst0Spec> {
        TestclkW::new(self, 0)
    }
    #[doc = "Bits 1:3 - Reserved as \"spare\" bit with no associated\n\nfunctionality."]
    #[inline(always)]
    #[must_use]
    pub fn spare_1(&mut self) -> Spare1W<PhyTst0Spec> {
        Spare1W::new(self, 1)
    }
    #[doc = "Bit 4 - Test Enable signal. Otherwise, this field is a \"spare\"\n\nbit with no associated functionality."]
    #[inline(always)]
    #[must_use]
    pub fn testen(&mut self) -> TestenW<PhyTst0Spec> {
        TestenW::new(self, 4)
    }
    #[doc = "Bit 5 - Test Clear signal. Otherwise, this field is a \"spare\"\n\nbit with no associated functionality."]
    #[inline(always)]
    #[must_use]
    pub fn testclr(&mut self) -> TestclrW<PhyTst0Spec> {
        TestclrW::new(self, 5)
    }
    #[doc = "Bits 6:7 - Reserved as \"spare\" bit with no associated\n\nfunctionality."]
    #[inline(always)]
    #[must_use]
    pub fn spare_2(&mut self) -> Spare2W<PhyTst0Spec> {
        Spare2W::new(self, 6)
    }
}
#[doc = "PHY Test Interface Register 0\n\nPHY TX mapped test interface (control).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_tst0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_tst0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PhyTst0Spec;
impl crate::RegisterSpec for PhyTst0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`phy_tst0::R`](R) reader structure"]
impl crate::Readable for PhyTst0Spec {}
#[doc = "`write(|w| ..)` method takes [`phy_tst0::W`](W) writer structure"]
impl crate::Writable for PhyTst0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PHY_TST0 to value 0"]
impl crate::Resettable for PhyTst0Spec {
    const RESET_VALUE: u8 = 0;
}
