#[doc = "Register `SWREG60` reader"]
pub type R = crate::R<Swreg60Spec>;
#[doc = "Register `SWREG60` writer"]
pub type W = crate::W<Swreg60Spec>;
#[doc = "Field `SW_ADDIT_CH_ST_ADR` reader - the start address for additional chrominance data format\n\nThe usage is enabled by sw_addit_ch_fmt_wen"]
pub type SwAdditChStAdrR = crate::FieldReader<u32>;
#[doc = "Field `SW_ADDIT_CH_ST_ADR` writer - the start address for additional chrominance data format\n\nThe usage is enabled by sw_addit_ch_fmt_wen"]
pub type SwAdditChStAdrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - the start address for additional chrominance data format\n\nThe usage is enabled by sw_addit_ch_fmt_wen"]
    #[inline(always)]
    pub fn sw_addit_ch_st_adr(&self) -> SwAdditChStAdrR {
        SwAdditChStAdrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - the start address for additional chrominance data format\n\nThe usage is enabled by sw_addit_ch_fmt_wen"]
    #[inline(always)]
    #[must_use]
    pub fn sw_addit_ch_st_adr(&mut self) -> SwAdditChStAdrW<Swreg60Spec> {
        SwAdditChStAdrW::new(self, 2)
    }
}
#[doc = "additional chrominance address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg60::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg60::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg60Spec;
impl crate::RegisterSpec for Swreg60Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg60::R`](R) reader structure"]
impl crate::Readable for Swreg60Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg60::W`](W) writer structure"]
impl crate::Writable for Swreg60Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG60 to value 0"]
impl crate::Resettable for Swreg60Spec {
    const RESET_VALUE: u32 = 0;
}
