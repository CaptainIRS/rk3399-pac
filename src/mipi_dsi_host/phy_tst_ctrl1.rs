#[doc = "Register `PHY_TST_CTRL1` reader"]
pub type R = crate::R<PhyTstCtrl1Spec>;
#[doc = "Register `PHY_TST_CTRL1` writer"]
pub type W = crate::W<PhyTstCtrl1Spec>;
#[doc = "Field `PHY_TESTDIN` reader - phy_testdin\n\nPHY test interface input 8-bit data bus for internal register\n\nprogramming and test functionalities access."]
pub type PhyTestdinR = crate::FieldReader;
#[doc = "Field `PHY_TESTDIN` writer - phy_testdin\n\nPHY test interface input 8-bit data bus for internal register\n\nprogramming and test functionalities access."]
pub type PhyTestdinW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHT_TESTDOUT` reader - pht_testdout\n\nPHY output 8-bit data bus for read-back and internal probing\n\nfunctionalities."]
pub type PhtTestdoutR = crate::FieldReader;
#[doc = "Field `PHY_TESTEN` reader - phy_testen\n\nPHY test interface operation selector:\n\n■1: The address write operation is set on the falling edge of the\n\ntestclk signal.\n\n■0: The data write operation is set on the rising edge of the testclk\n\nsignal."]
pub type PhyTestenR = crate::BitReader;
#[doc = "Field `PHY_TESTEN` writer - phy_testen\n\nPHY test interface operation selector:\n\n■1: The address write operation is set on the falling edge of the\n\ntestclk signal.\n\n■0: The data write operation is set on the rising edge of the testclk\n\nsignal."]
pub type PhyTestenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - phy_testdin\n\nPHY test interface input 8-bit data bus for internal register\n\nprogramming and test functionalities access."]
    #[inline(always)]
    pub fn phy_testdin(&self) -> PhyTestdinR {
        PhyTestdinR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - pht_testdout\n\nPHY output 8-bit data bus for read-back and internal probing\n\nfunctionalities."]
    #[inline(always)]
    pub fn pht_testdout(&self) -> PhtTestdoutR {
        PhtTestdoutR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - phy_testen\n\nPHY test interface operation selector:\n\n■1: The address write operation is set on the falling edge of the\n\ntestclk signal.\n\n■0: The data write operation is set on the rising edge of the testclk\n\nsignal."]
    #[inline(always)]
    pub fn phy_testen(&self) -> PhyTestenR {
        PhyTestenR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - phy_testdin\n\nPHY test interface input 8-bit data bus for internal register\n\nprogramming and test functionalities access."]
    #[inline(always)]
    #[must_use]
    pub fn phy_testdin(&mut self) -> PhyTestdinW<PhyTstCtrl1Spec> {
        PhyTestdinW::new(self, 0)
    }
    #[doc = "Bit 16 - phy_testen\n\nPHY test interface operation selector:\n\n■1: The address write operation is set on the falling edge of the\n\ntestclk signal.\n\n■0: The data write operation is set on the rising edge of the testclk\n\nsignal."]
    #[inline(always)]
    #[must_use]
    pub fn phy_testen(&mut self) -> PhyTestenW<PhyTstCtrl1Spec> {
        PhyTestenW::new(self, 16)
    }
}
#[doc = "D-PHY Test Interface Control 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_tst_ctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_tst_ctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PhyTstCtrl1Spec;
impl crate::RegisterSpec for PhyTstCtrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phy_tst_ctrl1::R`](R) reader structure"]
impl crate::Readable for PhyTstCtrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`phy_tst_ctrl1::W`](W) writer structure"]
impl crate::Writable for PhyTstCtrl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PHY_TST_CTRL1 to value 0"]
impl crate::Resettable for PhyTstCtrl1Spec {
    const RESET_VALUE: u32 = 0;
}
