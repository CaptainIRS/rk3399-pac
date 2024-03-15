#[doc = "Register `FC_EXCTRLDUR` reader"]
pub type R = crate::R<FcExctrldurSpec>;
#[doc = "Register `FC_EXCTRLDUR` writer"]
pub type W = crate::W<FcExctrldurSpec>;
#[doc = "Field `EXCTRLPERIODDURATION` reader - Configuration of the extended control period minimum duration (minimum of 32 pixel clock cycles; refer to HDMI 1.4b specification). Integer number of pixel clocks cycles \\[0..223\\]."]
pub type ExctrlperioddurationR = crate::FieldReader;
#[doc = "Field `EXCTRLPERIODDURATION` writer - Configuration of the extended control period minimum duration (minimum of 32 pixel clock cycles; refer to HDMI 1.4b specification). Integer number of pixel clocks cycles \\[0..223\\]."]
pub type ExctrlperioddurationW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Configuration of the extended control period minimum duration (minimum of 32 pixel clock cycles; refer to HDMI 1.4b specification). Integer number of pixel clocks cycles \\[0..223\\]."]
    #[inline(always)]
    pub fn exctrlperiodduration(&self) -> ExctrlperioddurationR {
        ExctrlperioddurationR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Configuration of the extended control period minimum duration (minimum of 32 pixel clock cycles; refer to HDMI 1.4b specification). Integer number of pixel clocks cycles \\[0..223\\]."]
    #[inline(always)]
    #[must_use]
    pub fn exctrlperiodduration(&mut self) -> ExctrlperioddurationW<FcExctrldurSpec> {
        ExctrlperioddurationW::new(self, 0)
    }
}
#[doc = "Configuration of the extended control period minimum duration (minimum of 32 pixel clock cycles; refer to HDMI 1.4b specification). Integer number of pixel clocks cycles \\[0..223\\].\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_exctrldur::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_exctrldur::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcExctrldurSpec;
impl crate::RegisterSpec for FcExctrldurSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_exctrldur::R`](R) reader structure"]
impl crate::Readable for FcExctrldurSpec {}
#[doc = "`write(|w| ..)` method takes [`fc_exctrldur::W`](W) writer structure"]
impl crate::Writable for FcExctrldurSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_EXCTRLDUR to value 0"]
impl crate::Resettable for FcExctrldurSpec {
    const RESET_VALUE: u8 = 0;
}
