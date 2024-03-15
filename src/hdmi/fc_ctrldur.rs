#[doc = "Register `FC_CTRLDUR` reader"]
pub type R = crate::R<FcCtrldurSpec>;
#[doc = "Register `FC_CTRLDUR` writer"]
pub type W = crate::W<FcCtrldurSpec>;
#[doc = "Field `CTRLPERIODDURATION` reader - Configuration of the control period minimum duration (minimum of 12 pixel clock cycles; refer to HDMI 1.4b specification). Integer number of pixel clocks cycles \\[0..223\\]."]
pub type CtrlperioddurationR = crate::FieldReader;
#[doc = "Field `CTRLPERIODDURATION` writer - Configuration of the control period minimum duration (minimum of 12 pixel clock cycles; refer to HDMI 1.4b specification). Integer number of pixel clocks cycles \\[0..223\\]."]
pub type CtrlperioddurationW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Configuration of the control period minimum duration (minimum of 12 pixel clock cycles; refer to HDMI 1.4b specification). Integer number of pixel clocks cycles \\[0..223\\]."]
    #[inline(always)]
    pub fn ctrlperiodduration(&self) -> CtrlperioddurationR {
        CtrlperioddurationR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Configuration of the control period minimum duration (minimum of 12 pixel clock cycles; refer to HDMI 1.4b specification). Integer number of pixel clocks cycles \\[0..223\\]."]
    #[inline(always)]
    #[must_use]
    pub fn ctrlperiodduration(&mut self) -> CtrlperioddurationW<FcCtrldurSpec> {
        CtrlperioddurationW::new(self, 0)
    }
}
#[doc = "Configuration of the control period minimum duration (minimum of 12 pixel clock cycles; refer to HDMI 1.4b specification). Integer number of pixel clocks cycles \\[0..223\\].\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_ctrldur::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_ctrldur::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcCtrldurSpec;
impl crate::RegisterSpec for FcCtrldurSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_ctrldur::R`](R) reader structure"]
impl crate::Readable for FcCtrldurSpec {}
#[doc = "`write(|w| ..)` method takes [`fc_ctrldur::W`](W) writer structure"]
impl crate::Writable for FcCtrldurSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_CTRLDUR to value 0"]
impl crate::Resettable for FcCtrldurSpec {
    const RESET_VALUE: u8 = 0;
}
