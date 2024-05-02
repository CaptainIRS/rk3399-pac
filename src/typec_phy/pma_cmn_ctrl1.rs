#[doc = "Register `PMA_CMN_CTRL1` reader"]
pub type R = crate::R<PmaCmnCtrl1Spec>;
#[doc = "Register `PMA_CMN_CTRL1` writer"]
pub type W = crate::W<PmaCmnCtrl1Spec>;
#[doc = "Field `FIELD8` reader - Current value of cmn_ready pin PMA output"]
pub type Field8R = crate::BitReader;
#[doc = "Field `FIELD7` reader - Current value of cmn_refclk_active PMA output"]
pub type Field7R = crate::BitReader;
#[doc = "Field `FIELD6` reader - Current value of cmn_macro_suspend_ack PMA output"]
pub type Field6R = crate::BitReader;
#[doc = "Field `FIELD5` reader - Drives cmn_ref_clk_rcv_out_en PMA input"]
pub type Field5R = crate::BitReader;
#[doc = "Field `FIELD5` writer - Drives cmn_ref_clk_rcv_out_en PMA input"]
pub type Field5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIELD4` reader - Drives cmn_ref_clk_sel PMA input"]
pub type Field4R = crate::FieldReader;
#[doc = "Field `FIELD4` writer - Drives cmn_ref_clk_sel PMA input"]
pub type Field4W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FIELD3` reader - Reserved"]
pub type Field3R = crate::FieldReader;
#[doc = "Field `FIELD2` reader - Drives cmn_psm_clk_dig_div PMA input"]
pub type Field2R = crate::FieldReader;
#[doc = "Field `FIELD2` writer - Drives cmn_psm_clk_dig_div PMA input"]
pub type Field2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FIELD1` reader - Drives cmn_ref_clk_dig_div PMA input"]
pub type Field1R = crate::FieldReader;
#[doc = "Field `FIELD1` writer - Drives cmn_ref_clk_dig_div PMA input"]
pub type Field1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FIELD0` reader - Drives cmn_ref_clk_ana_div PMA input"]
pub type Field0R = crate::FieldReader;
#[doc = "Field `FIELD0` writer - Drives cmn_ref_clk_ana_div PMA input"]
pub type Field0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Current value of cmn_ready pin PMA output"]
    #[inline(always)]
    pub fn field8(&self) -> Field8R {
        Field8R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Current value of cmn_refclk_active PMA output"]
    #[inline(always)]
    pub fn field7(&self) -> Field7R {
        Field7R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Current value of cmn_macro_suspend_ack PMA output"]
    #[inline(always)]
    pub fn field6(&self) -> Field6R {
        Field6R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Drives cmn_ref_clk_rcv_out_en PMA input"]
    #[inline(always)]
    pub fn field5(&self) -> Field5R {
        Field5R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Drives cmn_ref_clk_sel PMA input"]
    #[inline(always)]
    pub fn field4(&self) -> Field4R {
        Field4R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:9 - Reserved"]
    #[inline(always)]
    pub fn field3(&self) -> Field3R {
        Field3R::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bits 10:11 - Drives cmn_psm_clk_dig_div PMA input"]
    #[inline(always)]
    pub fn field2(&self) -> Field2R {
        Field2R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Drives cmn_ref_clk_dig_div PMA input"]
    #[inline(always)]
    pub fn field1(&self) -> Field1R {
        Field1R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Drives cmn_ref_clk_ana_div PMA input"]
    #[inline(always)]
    pub fn field0(&self) -> Field0R {
        Field0R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 3 - Drives cmn_ref_clk_rcv_out_en PMA input"]
    #[inline(always)]
    #[must_use]
    pub fn field5(&mut self) -> Field5W<PmaCmnCtrl1Spec> {
        Field5W::new(self, 3)
    }
    #[doc = "Bits 4:6 - Drives cmn_ref_clk_sel PMA input"]
    #[inline(always)]
    #[must_use]
    pub fn field4(&mut self) -> Field4W<PmaCmnCtrl1Spec> {
        Field4W::new(self, 4)
    }
    #[doc = "Bits 10:11 - Drives cmn_psm_clk_dig_div PMA input"]
    #[inline(always)]
    #[must_use]
    pub fn field2(&mut self) -> Field2W<PmaCmnCtrl1Spec> {
        Field2W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Drives cmn_ref_clk_dig_div PMA input"]
    #[inline(always)]
    #[must_use]
    pub fn field1(&mut self) -> Field1W<PmaCmnCtrl1Spec> {
        Field1W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Drives cmn_ref_clk_ana_div PMA input"]
    #[inline(always)]
    #[must_use]
    pub fn field0(&mut self) -> Field0W<PmaCmnCtrl1Spec> {
        Field0W::new(self, 14)
    }
}
#[doc = "PMA common control1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pma_cmn_ctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pma_cmn_ctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmaCmnCtrl1Spec;
impl crate::RegisterSpec for PmaCmnCtrl1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pma_cmn_ctrl1::R`](R) reader structure"]
impl crate::Readable for PmaCmnCtrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`pma_cmn_ctrl1::W`](W) writer structure"]
impl crate::Writable for PmaCmnCtrl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets PMA_CMN_CTRL1 to value 0"]
impl crate::Resettable for PmaCmnCtrl1Spec {
    const RESET_VALUE: u16 = 0;
}
