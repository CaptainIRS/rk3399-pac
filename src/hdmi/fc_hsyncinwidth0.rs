#[doc = "Register `FC_HSYNCINWIDTH0` reader"]
pub type R = crate::R<FcHsyncinwidth0Spec>;
#[doc = "Register `FC_HSYNCINWIDTH0` writer"]
pub type W = crate::W<FcHsyncinwidth0Spec>;
#[doc = "Field `H_IN_WIDTH` reader - Input video Hsync active pulse width. Integer\n\nnumber of pixel clock cycles \\[0...511\\]."]
pub type HInWidthR = crate::FieldReader;
#[doc = "Field `H_IN_WIDTH` writer - Input video Hsync active pulse width. Integer\n\nnumber of pixel clock cycles \\[0...511\\]."]
pub type HInWidthW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Input video Hsync active pulse width. Integer\n\nnumber of pixel clock cycles \\[0...511\\]."]
    #[inline(always)]
    pub fn h_in_width(&self) -> HInWidthR {
        HInWidthR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Input video Hsync active pulse width. Integer\n\nnumber of pixel clock cycles \\[0...511\\]."]
    #[inline(always)]
    #[must_use]
    pub fn h_in_width(&mut self) -> HInWidthW<FcHsyncinwidth0Spec> {
        HInWidthW::new(self, 0)
    }
}
#[doc = "Frame Composer Input Video HSync Width Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_hsyncinwidth0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_hsyncinwidth0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcHsyncinwidth0Spec;
impl crate::RegisterSpec for FcHsyncinwidth0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_hsyncinwidth0::R`](R) reader structure"]
impl crate::Readable for FcHsyncinwidth0Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_hsyncinwidth0::W`](W) writer structure"]
impl crate::Writable for FcHsyncinwidth0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_HSYNCINWIDTH0 to value 0"]
impl crate::Resettable for FcHsyncinwidth0Spec {
    const RESET_VALUE: u8 = 0;
}
