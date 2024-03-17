#[doc = "Register `DENALI_CTL_19` reader"]
pub type R = crate::R<DenaliCtl19Spec>;
#[doc = "Register `DENALI_CTL_19` writer"]
pub type W = crate::W<DenaliCtl19Spec>;
#[doc = "Field `DFIBUS_FREQ_F0` reader - Defines the DFI bus frequency for frequency copy 0."]
pub type DfibusFreqF0R = crate::FieldReader;
#[doc = "Field `DFIBUS_FREQ_F0` writer - Defines the DFI bus frequency for frequency copy 0."]
pub type DfibusFreqF0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DFIBUS_FREQ_F1` reader - Defines the DFI bus frequency for frequency copy 1."]
pub type DfibusFreqF1R = crate::FieldReader;
#[doc = "Field `DFIBUS_FREQ_F1` writer - Defines the DFI bus frequency for frequency copy 1."]
pub type DfibusFreqF1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DFIBUS_FREQ_F2` reader - Defines the DFI bus frequency for frequency copy 2."]
pub type DfibusFreqF2R = crate::FieldReader;
#[doc = "Field `DFIBUS_FREQ_F2` writer - Defines the DFI bus frequency for frequency copy 2."]
pub type DfibusFreqF2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Defines the DFI bus frequency for frequency copy 0."]
    #[inline(always)]
    pub fn dfibus_freq_f0(&self) -> DfibusFreqF0R {
        DfibusFreqF0R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Defines the DFI bus frequency for frequency copy 1."]
    #[inline(always)]
    pub fn dfibus_freq_f1(&self) -> DfibusFreqF1R {
        DfibusFreqF1R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Defines the DFI bus frequency for frequency copy 2."]
    #[inline(always)]
    pub fn dfibus_freq_f2(&self) -> DfibusFreqF2R {
        DfibusFreqF2R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Defines the DFI bus frequency for frequency copy 0."]
    #[inline(always)]
    #[must_use]
    pub fn dfibus_freq_f0(&mut self) -> DfibusFreqF0W<DenaliCtl19Spec> {
        DfibusFreqF0W::new(self, 0)
    }
    #[doc = "Bits 8:12 - Defines the DFI bus frequency for frequency copy 1."]
    #[inline(always)]
    #[must_use]
    pub fn dfibus_freq_f1(&mut self) -> DfibusFreqF1W<DenaliCtl19Spec> {
        DfibusFreqF1W::new(self, 8)
    }
    #[doc = "Bits 16:20 - Defines the DFI bus frequency for frequency copy 2."]
    #[inline(always)]
    #[must_use]
    pub fn dfibus_freq_f2(&mut self) -> DfibusFreqF2W<DenaliCtl19Spec> {
        DfibusFreqF2W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_19::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_19::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl19Spec;
impl crate::RegisterSpec for DenaliCtl19Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_19::R`](R) reader structure"]
impl crate::Readable for DenaliCtl19Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_19::W`](W) writer structure"]
impl crate::Writable for DenaliCtl19Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_19 to value 0"]
impl crate::Resettable for DenaliCtl19Spec {
    const RESET_VALUE: u32 = 0;
}
