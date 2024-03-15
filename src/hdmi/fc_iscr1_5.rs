#[doc = "Register `FC_ISCR1_5` reader"]
pub type R = crate::R<FcIscr1_5Spec>;
#[doc = "Register `FC_ISCR1_5` writer"]
pub type W = crate::W<FcIscr1_5Spec>;
#[doc = "Field `FC_ISCR1_5` reader - Frame Composer ISRC1 Packet Body Register 5"]
pub type FcIscr1_5R = crate::FieldReader;
#[doc = "Field `FC_ISCR1_5` writer - Frame Composer ISRC1 Packet Body Register 5"]
pub type FcIscr1_5W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Frame Composer ISRC1 Packet Body Register 5"]
    #[inline(always)]
    pub fn fc_iscr1_5(&self) -> FcIscr1_5R {
        FcIscr1_5R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Frame Composer ISRC1 Packet Body Register 5"]
    #[inline(always)]
    #[must_use]
    pub fn fc_iscr1_5(&mut self) -> FcIscr1_5W<FcIscr1_5Spec> {
        FcIscr1_5W::new(self, 0)
    }
}
#[doc = "Frame Composer ISRC1 Packet Body Register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_iscr1_5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_iscr1_5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcIscr1_5Spec;
impl crate::RegisterSpec for FcIscr1_5Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_iscr1_5::R`](R) reader structure"]
impl crate::Readable for FcIscr1_5Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_iscr1_5::W`](W) writer structure"]
impl crate::Writable for FcIscr1_5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_ISCR1_5 to value 0"]
impl crate::Resettable for FcIscr1_5Spec {
    const RESET_VALUE: u8 = 0;
}
