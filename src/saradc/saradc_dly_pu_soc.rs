#[doc = "Register `SARADC_DLY_PU_SOC` reader"]
pub type R = crate::R<SaradcDlyPuSocSpec>;
#[doc = "Register `SARADC_DLY_PU_SOC` writer"]
pub type W = crate::W<SaradcDlyPuSocSpec>;
#[doc = "Field `DLY_PU_SOC` reader - delay between power up and start command The start signal will be asserted (DLY_PU_SOC + 2) sclk clock period later after power up"]
pub type DlyPuSocR = crate::FieldReader;
#[doc = "Field `DLY_PU_SOC` writer - delay between power up and start command The start signal will be asserted (DLY_PU_SOC + 2) sclk clock period later after power up"]
pub type DlyPuSocW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - delay between power up and start command The start signal will be asserted (DLY_PU_SOC + 2) sclk clock period later after power up"]
    #[inline(always)]
    pub fn dly_pu_soc(&self) -> DlyPuSocR {
        DlyPuSocR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - delay between power up and start command The start signal will be asserted (DLY_PU_SOC + 2) sclk clock period later after power up"]
    #[inline(always)]
    #[must_use]
    pub fn dly_pu_soc(&mut self) -> DlyPuSocW<SaradcDlyPuSocSpec> {
        DlyPuSocW::new(self, 0)
    }
}
#[doc = "delay between power up and start command\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`saradc_dly_pu_soc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`saradc_dly_pu_soc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SaradcDlyPuSocSpec;
impl crate::RegisterSpec for SaradcDlyPuSocSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`saradc_dly_pu_soc::R`](R) reader structure"]
impl crate::Readable for SaradcDlyPuSocSpec {}
#[doc = "`write(|w| ..)` method takes [`saradc_dly_pu_soc::W`](W) writer structure"]
impl crate::Writable for SaradcDlyPuSocSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SARADC_DLY_PU_SOC to value 0"]
impl crate::Resettable for SaradcDlyPuSocSpec {
    const RESET_VALUE: u32 = 0;
}
