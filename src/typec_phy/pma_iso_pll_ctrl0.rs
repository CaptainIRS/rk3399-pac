#[doc = "Register `PMA_ISO_PLL_CTRL0` reader"]
pub type R = crate::R<PmaIsoPllCtrl0Spec>;
#[doc = "Register `PMA_ISO_PLL_CTRL0` writer"]
pub type W = crate::W<PmaIsoPllCtrl0Spec>;
#[doc = "Field `FIELD4` reader - Drives cmn_pll0_en PMA input when in PMA isolation mode"]
pub type Field4R = crate::BitReader;
#[doc = "Field `FIELD4` writer - Drives cmn_pll0_en PMA input when in PMA isolation mode"]
pub type Field4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIELD3` reader - Drives cmn_pll1_en PMA input when in PMA isolation mode"]
pub type Field3R = crate::BitReader;
#[doc = "Field `FIELD3` writer - Drives cmn_pll1_en PMA input when in PMA isolation mode"]
pub type Field3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIELD2` reader - Drives cmn_pll0_clk_datart_en PMA input when in PMA isolation \n\nmode"]
pub type Field2R = crate::BitReader;
#[doc = "Field `FIELD2` writer - Drives cmn_pll0_clk_datart_en PMA input when in PMA isolation \n\nmode"]
pub type Field2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIELD1` reader - Drives cmn_pll1_clk_datart_en PMA input when in PMA isolation \n\nmode"]
pub type Field1R = crate::BitReader;
#[doc = "Field `FIELD1` writer - Drives cmn_pll1_clk_datart_en PMA input when in PMA isolation \n\nmode"]
pub type Field1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIELD0` reader - Reserved"]
pub type Field0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - Drives cmn_pll0_en PMA input when in PMA isolation mode"]
    #[inline(always)]
    pub fn field4(&self) -> Field4R {
        Field4R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Drives cmn_pll1_en PMA input when in PMA isolation mode"]
    #[inline(always)]
    pub fn field3(&self) -> Field3R {
        Field3R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Drives cmn_pll0_clk_datart_en PMA input when in PMA isolation \n\nmode"]
    #[inline(always)]
    pub fn field2(&self) -> Field2R {
        Field2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Drives cmn_pll1_clk_datart_en PMA input when in PMA isolation \n\nmode"]
    #[inline(always)]
    pub fn field1(&self) -> Field1R {
        Field1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:15 - Reserved"]
    #[inline(always)]
    pub fn field0(&self) -> Field0R {
        Field0R::new((self.bits >> 4) & 0x0fff)
    }
}
impl W {
    #[doc = "Bit 0 - Drives cmn_pll0_en PMA input when in PMA isolation mode"]
    #[inline(always)]
    #[must_use]
    pub fn field4(&mut self) -> Field4W<PmaIsoPllCtrl0Spec> {
        Field4W::new(self, 0)
    }
    #[doc = "Bit 1 - Drives cmn_pll1_en PMA input when in PMA isolation mode"]
    #[inline(always)]
    #[must_use]
    pub fn field3(&mut self) -> Field3W<PmaIsoPllCtrl0Spec> {
        Field3W::new(self, 1)
    }
    #[doc = "Bit 2 - Drives cmn_pll0_clk_datart_en PMA input when in PMA isolation \n\nmode"]
    #[inline(always)]
    #[must_use]
    pub fn field2(&mut self) -> Field2W<PmaIsoPllCtrl0Spec> {
        Field2W::new(self, 2)
    }
    #[doc = "Bit 3 - Drives cmn_pll1_clk_datart_en PMA input when in PMA isolation \n\nmode"]
    #[inline(always)]
    #[must_use]
    pub fn field1(&mut self) -> Field1W<PmaIsoPllCtrl0Spec> {
        Field1W::new(self, 3)
    }
}
#[doc = "PMA PLL control0 isolation register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pma_iso_pll_ctrl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pma_iso_pll_ctrl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmaIsoPllCtrl0Spec;
impl crate::RegisterSpec for PmaIsoPllCtrl0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pma_iso_pll_ctrl0::R`](R) reader structure"]
impl crate::Readable for PmaIsoPllCtrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`pma_iso_pll_ctrl0::W`](W) writer structure"]
impl crate::Writable for PmaIsoPllCtrl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets PMA_ISO_PLL_CTRL0 to value 0x0f"]
impl crate::Resettable for PmaIsoPllCtrl0Spec {
    const RESET_VALUE: u16 = 0x0f;
}
