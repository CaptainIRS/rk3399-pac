#[doc = "Register `PMA_ISO_PLL_CTRL1` reader"]
pub type R = crate::R<PmaIsoPllCtrl1Spec>;
#[doc = "Register `PMA_ISO_PLL_CTRL1` writer"]
pub type W = crate::W<PmaIsoPllCtrl1Spec>;
#[doc = "Field `FIELD3` reader - Drives cmn_pll0_clk_datart0_div PMA input when in PMA isolation \n\nmode"]
pub type Field3R = crate::FieldReader;
#[doc = "Field `FIELD3` writer - Drives cmn_pll0_clk_datart0_div PMA input when in PMA isolation \n\nmode"]
pub type Field3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FIELD2` reader - Drives cmn_pll0_clk_datart1_div PMA input when in PMA isolation \n\nmode"]
pub type Field2R = crate::FieldReader;
#[doc = "Field `FIELD2` writer - Drives cmn_pll0_clk_datart1_div PMA input when in PMA isolation \n\nmode"]
pub type Field2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FIELD1` reader - Drives cmn_pll1_clk_datart0_div PMA input when in PMA isolation \n\nmode"]
pub type Field1R = crate::FieldReader;
#[doc = "Field `FIELD1` writer - Drives cmn_pll1_clk_datart0_div PMA input when in PMA isolation \n\nmode"]
pub type Field1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FIELD0` reader - Drives cmn_pll1_clk_datart1_div PMA input when in PMA isolation \n\nmode"]
pub type Field0R = crate::FieldReader;
#[doc = "Field `FIELD0` writer - Drives cmn_pll1_clk_datart1_div PMA input when in PMA isolation \n\nmode"]
pub type Field0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Drives cmn_pll0_clk_datart0_div PMA input when in PMA isolation \n\nmode"]
    #[inline(always)]
    pub fn field3(&self) -> Field3R {
        Field3R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Drives cmn_pll0_clk_datart1_div PMA input when in PMA isolation \n\nmode"]
    #[inline(always)]
    pub fn field2(&self) -> Field2R {
        Field2R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Drives cmn_pll1_clk_datart0_div PMA input when in PMA isolation \n\nmode"]
    #[inline(always)]
    pub fn field1(&self) -> Field1R {
        Field1R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Drives cmn_pll1_clk_datart1_div PMA input when in PMA isolation \n\nmode"]
    #[inline(always)]
    pub fn field0(&self) -> Field0R {
        Field0R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Drives cmn_pll0_clk_datart0_div PMA input when in PMA isolation \n\nmode"]
    #[inline(always)]
    #[must_use]
    pub fn field3(&mut self) -> Field3W<PmaIsoPllCtrl1Spec> {
        Field3W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Drives cmn_pll0_clk_datart1_div PMA input when in PMA isolation \n\nmode"]
    #[inline(always)]
    #[must_use]
    pub fn field2(&mut self) -> Field2W<PmaIsoPllCtrl1Spec> {
        Field2W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Drives cmn_pll1_clk_datart0_div PMA input when in PMA isolation \n\nmode"]
    #[inline(always)]
    #[must_use]
    pub fn field1(&mut self) -> Field1W<PmaIsoPllCtrl1Spec> {
        Field1W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Drives cmn_pll1_clk_datart1_div PMA input when in PMA isolation \n\nmode"]
    #[inline(always)]
    #[must_use]
    pub fn field0(&mut self) -> Field0W<PmaIsoPllCtrl1Spec> {
        Field0W::new(self, 12)
    }
}
#[doc = "PMA PLL control1 isolation register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pma_iso_pll_ctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pma_iso_pll_ctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmaIsoPllCtrl1Spec;
impl crate::RegisterSpec for PmaIsoPllCtrl1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pma_iso_pll_ctrl1::R`](R) reader structure"]
impl crate::Readable for PmaIsoPllCtrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`pma_iso_pll_ctrl1::W`](W) writer structure"]
impl crate::Writable for PmaIsoPllCtrl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets PMA_ISO_PLL_CTRL1 to value 0x1122"]
impl crate::Resettable for PmaIsoPllCtrl1Spec {
    const RESET_VALUE: u16 = 0x1122;
}
