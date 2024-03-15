#[doc = "Register `FC_AMP_HB2` reader"]
pub type R = crate::R<FcAmpHb2Spec>;
#[doc = "Register `FC_AMP_HB2` writer"]
pub type W = crate::W<FcAmpHb2Spec>;
#[doc = "Field `FC_AMP_HB1` reader - Frame Composer AMP Packet Header Register 2"]
pub type FcAmpHb1R = crate::FieldReader;
#[doc = "Field `FC_AMP_HB1` writer - Frame Composer AMP Packet Header Register 2"]
pub type FcAmpHb1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Frame Composer AMP Packet Header Register 2"]
    #[inline(always)]
    pub fn fc_amp_hb1(&self) -> FcAmpHb1R {
        FcAmpHb1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Frame Composer AMP Packet Header Register 2"]
    #[inline(always)]
    #[must_use]
    pub fn fc_amp_hb1(&mut self) -> FcAmpHb1W<FcAmpHb2Spec> {
        FcAmpHb1W::new(self, 0)
    }
}
#[doc = "Frame Composer AMP Packet Header Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_amp_hb2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_amp_hb2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcAmpHb2Spec;
impl crate::RegisterSpec for FcAmpHb2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_amp_hb2::R`](R) reader structure"]
impl crate::Readable for FcAmpHb2Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_amp_hb2::W`](W) writer structure"]
impl crate::Writable for FcAmpHb2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_AMP_HB2 to value 0"]
impl crate::Resettable for FcAmpHb2Spec {
    const RESET_VALUE: u8 = 0;
}
