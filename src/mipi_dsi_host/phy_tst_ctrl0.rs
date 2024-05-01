#[doc = "Register `PHY_TST_CTRL0` reader"]
pub type R = crate::R<PhyTstCtrl0Spec>;
#[doc = "Register `PHY_TST_CTRL0` writer"]
pub type W = crate::W<PhyTstCtrl0Spec>;
#[doc = "Field `PHY_TESTCLR` reader - phy_testclr\n\nPHY test interface clear (active high)."]
pub type PhyTestclrR = crate::BitReader;
#[doc = "Field `PHY_TESTCLR` writer - phy_testclr\n\nPHY test interface clear (active high)."]
pub type PhyTestclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_TESTCLK` reader - phy_testclk\n\nThis bit is used to clock the TESTDIN bus into the D-PHY."]
pub type PhyTestclkR = crate::BitReader;
#[doc = "Field `PHY_TESTCLK` writer - phy_testclk\n\nThis bit is used to clock the TESTDIN bus into the D-PHY."]
pub type PhyTestclkW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - phy_testclr\n\nPHY test interface clear (active high)."]
    #[inline(always)]
    pub fn phy_testclr(&self) -> PhyTestclrR {
        PhyTestclrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - phy_testclk\n\nThis bit is used to clock the TESTDIN bus into the D-PHY."]
    #[inline(always)]
    pub fn phy_testclk(&self) -> PhyTestclkR {
        PhyTestclkR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - phy_testclr\n\nPHY test interface clear (active high)."]
    #[inline(always)]
    #[must_use]
    pub fn phy_testclr(&mut self) -> PhyTestclrW<PhyTstCtrl0Spec> {
        PhyTestclrW::new(self, 0)
    }
    #[doc = "Bit 1 - phy_testclk\n\nThis bit is used to clock the TESTDIN bus into the D-PHY."]
    #[inline(always)]
    #[must_use]
    pub fn phy_testclk(&mut self) -> PhyTestclkW<PhyTstCtrl0Spec> {
        PhyTestclkW::new(self, 1)
    }
}
#[doc = "D-PHY Test Interface Control 0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_tst_ctrl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_tst_ctrl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PhyTstCtrl0Spec;
impl crate::RegisterSpec for PhyTstCtrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phy_tst_ctrl0::R`](R) reader structure"]
impl crate::Readable for PhyTstCtrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`phy_tst_ctrl0::W`](W) writer structure"]
impl crate::Writable for PhyTstCtrl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PHY_TST_CTRL0 to value 0"]
impl crate::Resettable for PhyTstCtrl0Spec {
    const RESET_VALUE: u32 = 0;
}
