#[doc = "Register `DENALI_CTL_214` reader"]
pub type R = crate::R<DenaliCtl214Spec>;
#[doc = "Register `DENALI_CTL_214` writer"]
pub type W = crate::W<DenaliCtl214Spec>;
#[doc = "Field `ODT_EN_F2` reader - Enable support of DRAM ODT. When enabled, controller will assert and de-assert ODT output to DRAM as needed."]
pub type OdtEnF2R = crate::BitReader;
#[doc = "Field `ODT_EN_F2` writer - Enable support of DRAM ODT. When enabled, controller will assert and de-assert ODT output to DRAM as needed."]
pub type OdtEnF2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN_ODT_ASSERT_EXCEPT_RD` reader - Enable controller to assert ODT at all times except during reads. Assumes single ODT pin connected. Set to 1 to enable."]
pub type EnOdtAssertExceptRdR = crate::BitReader;
#[doc = "Field `EN_ODT_ASSERT_EXCEPT_RD` writer - Enable controller to assert ODT at all times except during reads. Assumes single ODT pin connected. Set to 1 to enable."]
pub type EnOdtAssertExceptRdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WR_TO_ODTH_F0` reader - Defines the delay from a write command to ODT assertion."]
pub type WrToOdthF0R = crate::FieldReader;
#[doc = "Field `WR_TO_ODTH_F0` writer - Defines the delay from a write command to ODT assertion."]
pub type WrToOdthF0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `WR_TO_ODTH_F1` reader - Defines the delay from a write command to ODT assertion."]
pub type WrToOdthF1R = crate::FieldReader;
#[doc = "Field `WR_TO_ODTH_F1` writer - Defines the delay from a write command to ODT assertion."]
pub type WrToOdthF1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - Enable support of DRAM ODT. When enabled, controller will assert and de-assert ODT output to DRAM as needed."]
    #[inline(always)]
    pub fn odt_en_f2(&self) -> OdtEnF2R {
        OdtEnF2R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Enable controller to assert ODT at all times except during reads. Assumes single ODT pin connected. Set to 1 to enable."]
    #[inline(always)]
    pub fn en_odt_assert_except_rd(&self) -> EnOdtAssertExceptRdR {
        EnOdtAssertExceptRdR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:21 - Defines the delay from a write command to ODT assertion."]
    #[inline(always)]
    pub fn wr_to_odth_f0(&self) -> WrToOdthF0R {
        WrToOdthF0R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - Defines the delay from a write command to ODT assertion."]
    #[inline(always)]
    pub fn wr_to_odth_f1(&self) -> WrToOdthF1R {
        WrToOdthF1R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable support of DRAM ODT. When enabled, controller will assert and de-assert ODT output to DRAM as needed."]
    #[inline(always)]
    #[must_use]
    pub fn odt_en_f2(&mut self) -> OdtEnF2W<DenaliCtl214Spec> {
        OdtEnF2W::new(self, 0)
    }
    #[doc = "Bit 8 - Enable controller to assert ODT at all times except during reads. Assumes single ODT pin connected. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn en_odt_assert_except_rd(&mut self) -> EnOdtAssertExceptRdW<DenaliCtl214Spec> {
        EnOdtAssertExceptRdW::new(self, 8)
    }
    #[doc = "Bits 16:21 - Defines the delay from a write command to ODT assertion."]
    #[inline(always)]
    #[must_use]
    pub fn wr_to_odth_f0(&mut self) -> WrToOdthF0W<DenaliCtl214Spec> {
        WrToOdthF0W::new(self, 16)
    }
    #[doc = "Bits 24:29 - Defines the delay from a write command to ODT assertion."]
    #[inline(always)]
    #[must_use]
    pub fn wr_to_odth_f1(&mut self) -> WrToOdthF1W<DenaliCtl214Spec> {
        WrToOdthF1W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_214::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_214::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl214Spec;
impl crate::RegisterSpec for DenaliCtl214Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_214::R`](R) reader structure"]
impl crate::Readable for DenaliCtl214Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_214::W`](W) writer structure"]
impl crate::Writable for DenaliCtl214Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_214 to value 0"]
impl crate::Resettable for DenaliCtl214Spec {
    const RESET_VALUE: u32 = 0;
}
