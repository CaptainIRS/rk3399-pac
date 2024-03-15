#[doc = "Register `FC_ISCR1_11` reader"]
pub type R = crate::R<FcIscr1_11Spec>;
#[doc = "Register `FC_ISCR1_11` writer"]
pub type W = crate::W<FcIscr1_11Spec>;
#[doc = "Field `FC_ISCR1_11` reader - Frame Composer ISRC1 Packet Body Register 11"]
pub type FcIscr1_11R = crate::FieldReader;
#[doc = "Field `FC_ISCR1_11` writer - Frame Composer ISRC1 Packet Body Register 11"]
pub type FcIscr1_11W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Frame Composer ISRC1 Packet Body Register 11"]
    #[inline(always)]
    pub fn fc_iscr1_11(&self) -> FcIscr1_11R {
        FcIscr1_11R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Frame Composer ISRC1 Packet Body Register 11"]
    #[inline(always)]
    #[must_use]
    pub fn fc_iscr1_11(&mut self) -> FcIscr1_11W<FcIscr1_11Spec> {
        FcIscr1_11W::new(self, 0)
    }
}
#[doc = "Frame Composer ISRC1 Packet Body Register 11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_iscr1_11::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_iscr1_11::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcIscr1_11Spec;
impl crate::RegisterSpec for FcIscr1_11Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_iscr1_11::R`](R) reader structure"]
impl crate::Readable for FcIscr1_11Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_iscr1_11::W`](W) writer structure"]
impl crate::Writable for FcIscr1_11Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_ISCR1_11 to value 0"]
impl crate::Resettable for FcIscr1_11Spec {
    const RESET_VALUE: u8 = 0;
}
