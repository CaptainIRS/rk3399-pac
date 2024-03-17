#[doc = "Register `DENALI_CTL_264` reader"]
pub type R = crate::R<DenaliCtl264Spec>;
#[doc = "Register `DENALI_CTL_264` writer"]
pub type W = crate::W<DenaliCtl264Spec>;
#[doc = "Field `CALVL_SEQ_EN` reader - Specifies which CA training patterns will be used. Clear to 0 for pattern 0 only, program to 1 for patterns 0 and 1, program to 2 for patterns 0, 1 and 2, or program to 3 for all patterns."]
pub type CalvlSeqEnR = crate::FieldReader;
#[doc = "Field `CALVL_SEQ_EN` writer - Specifies which CA training patterns will be used. Clear to 0 for pattern 0 only, program to 1 for patterns 0 and 1, program to 2 for patterns 0, 1 and 2, or program to 3 for all patterns."]
pub type CalvlSeqEnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DFI_PHY_CALVL_MODE` reader - Specifies the PHY support for DFI CA training. Set to 1 for supported."]
pub type DfiPhyCalvlModeR = crate::BitReader;
#[doc = "Field `DFI_PHY_CALVL_MODE` writer - Specifies the PHY support for DFI CA training. Set to 1 for supported."]
pub type DfiPhyCalvlModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALVL_PERIODIC` reader - Enables the use of the dfi_lvl_periodic signal during CA training. Set to 1 to enable."]
pub type CalvlPeriodicR = crate::BitReader;
#[doc = "Field `CALVL_PERIODIC` writer - Enables the use of the dfi_lvl_periodic signal during CA training. Set to 1 to enable."]
pub type CalvlPeriodicW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 8:9 - Specifies which CA training patterns will be used. Clear to 0 for pattern 0 only, program to 1 for patterns 0 and 1, program to 2 for patterns 0, 1 and 2, or program to 3 for all patterns."]
    #[inline(always)]
    pub fn calvl_seq_en(&self) -> CalvlSeqEnR {
        CalvlSeqEnR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 16 - Specifies the PHY support for DFI CA training. Set to 1 for supported."]
    #[inline(always)]
    pub fn dfi_phy_calvl_mode(&self) -> DfiPhyCalvlModeR {
        DfiPhyCalvlModeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Enables the use of the dfi_lvl_periodic signal during CA training. Set to 1 to enable."]
    #[inline(always)]
    pub fn calvl_periodic(&self) -> CalvlPeriodicR {
        CalvlPeriodicR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 8:9 - Specifies which CA training patterns will be used. Clear to 0 for pattern 0 only, program to 1 for patterns 0 and 1, program to 2 for patterns 0, 1 and 2, or program to 3 for all patterns."]
    #[inline(always)]
    #[must_use]
    pub fn calvl_seq_en(&mut self) -> CalvlSeqEnW<DenaliCtl264Spec> {
        CalvlSeqEnW::new(self, 8)
    }
    #[doc = "Bit 16 - Specifies the PHY support for DFI CA training. Set to 1 for supported."]
    #[inline(always)]
    #[must_use]
    pub fn dfi_phy_calvl_mode(&mut self) -> DfiPhyCalvlModeW<DenaliCtl264Spec> {
        DfiPhyCalvlModeW::new(self, 16)
    }
    #[doc = "Bit 24 - Enables the use of the dfi_lvl_periodic signal during CA training. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn calvl_periodic(&mut self) -> CalvlPeriodicW<DenaliCtl264Spec> {
        CalvlPeriodicW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_264::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_264::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl264Spec;
impl crate::RegisterSpec for DenaliCtl264Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_264::R`](R) reader structure"]
impl crate::Readable for DenaliCtl264Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_264::W`](W) writer structure"]
impl crate::Writable for DenaliCtl264Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_264 to value 0"]
impl crate::Resettable for DenaliCtl264Spec {
    const RESET_VALUE: u32 = 0;
}
