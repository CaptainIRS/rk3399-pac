#[doc = "Register `VSM_MODE` reader"]
pub type R = crate::R<VsmModeSpec>;
#[doc = "Register `VSM_MODE` writer"]
pub type W = crate::W<VsmModeSpec>;
#[doc = "Field `vsm_meas_en` reader - 1: enable measure."]
pub type VsmMeasEnR = crate::BitReader;
#[doc = "Field `vsm_meas_en` writer - 1: enable measure."]
pub type VsmMeasEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `vsm_meas_irq_enable` reader - 1: VS measure done IRQ enable."]
pub type VsmMeasIrqEnableR = crate::BitReader;
#[doc = "Field `vsm_meas_irq_enable` writer - 1: VS measure done IRQ enable."]
pub type VsmMeasIrqEnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1: enable measure."]
    #[inline(always)]
    pub fn vsm_meas_en(&self) -> VsmMeasEnR {
        VsmMeasEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1: VS measure done IRQ enable."]
    #[inline(always)]
    pub fn vsm_meas_irq_enable(&self) -> VsmMeasIrqEnableR {
        VsmMeasIrqEnableR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1: enable measure."]
    #[inline(always)]
    #[must_use]
    pub fn vsm_meas_en(&mut self) -> VsmMeasEnW<VsmModeSpec> {
        VsmMeasEnW::new(self, 0)
    }
    #[doc = "Bit 1 - 1: VS measure done IRQ enable."]
    #[inline(always)]
    #[must_use]
    pub fn vsm_meas_irq_enable(&mut self) -> VsmMeasIrqEnableW<VsmModeSpec> {
        VsmMeasIrqEnableW::new(self, 1)
    }
}
#[doc = "VS Measure Mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vsm_mode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vsm_mode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VsmModeSpec;
impl crate::RegisterSpec for VsmModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vsm_mode::R`](R) reader structure"]
impl crate::Readable for VsmModeSpec {}
#[doc = "`write(|w| ..)` method takes [`vsm_mode::W`](W) writer structure"]
impl crate::Writable for VsmModeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VSM_MODE to value 0"]
impl crate::Resettable for VsmModeSpec {
    const RESET_VALUE: u32 = 0;
}
