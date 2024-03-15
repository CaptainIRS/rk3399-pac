#[doc = "Register `FC_HSYNCINDELAY0` reader"]
pub type R = crate::R<FcHsyncindelay0Spec>;
#[doc = "Register `FC_HSYNCINDELAY0` writer"]
pub type W = crate::W<FcHsyncindelay0Spec>;
#[doc = "Field `H_IN_DELAY` reader - Input video Hsync active edge delay. Integer number of pixel clock cycles from \"de\" non active edge of the last \"de\" valid period \\[0...4095\\]."]
pub type HInDelayR = crate::FieldReader;
#[doc = "Field `H_IN_DELAY` writer - Input video Hsync active edge delay. Integer number of pixel clock cycles from \"de\" non active edge of the last \"de\" valid period \\[0...4095\\]."]
pub type HInDelayW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Input video Hsync active edge delay. Integer number of pixel clock cycles from \"de\" non active edge of the last \"de\" valid period \\[0...4095\\]."]
    #[inline(always)]
    pub fn h_in_delay(&self) -> HInDelayR {
        HInDelayR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Input video Hsync active edge delay. Integer number of pixel clock cycles from \"de\" non active edge of the last \"de\" valid period \\[0...4095\\]."]
    #[inline(always)]
    #[must_use]
    pub fn h_in_delay(&mut self) -> HInDelayW<FcHsyncindelay0Spec> {
        HInDelayW::new(self, 0)
    }
}
#[doc = "Input video Hsync active edge delay. Integer number of pixel clock cycles from \"de\" non active edge of the last \"de\" valid period \\[0...4095\\].\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_hsyncindelay0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_hsyncindelay0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcHsyncindelay0Spec;
impl crate::RegisterSpec for FcHsyncindelay0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_hsyncindelay0::R`](R) reader structure"]
impl crate::Readable for FcHsyncindelay0Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_hsyncindelay0::W`](W) writer structure"]
impl crate::Writable for FcHsyncindelay0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_HSYNCINDELAY0 to value 0"]
impl crate::Resettable for FcHsyncindelay0Spec {
    const RESET_VALUE: u8 = 0;
}
