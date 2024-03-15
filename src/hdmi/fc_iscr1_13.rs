#[doc = "Register `FC_ISCR1_13` reader"]
pub type R = crate::R<FcIscr1_13Spec>;
#[doc = "Register `FC_ISCR1_13` writer"]
pub type W = crate::W<FcIscr1_13Spec>;
#[doc = "Field `FC_ISCR1_13` reader - Frame Composer ISRC1 Packet Body Register 13"]
pub type FcIscr1_13R = crate::FieldReader;
#[doc = "Field `FC_ISCR1_13` writer - Frame Composer ISRC1 Packet Body Register 13"]
pub type FcIscr1_13W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Frame Composer ISRC1 Packet Body Register 13"]
    #[inline(always)]
    pub fn fc_iscr1_13(&self) -> FcIscr1_13R {
        FcIscr1_13R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Frame Composer ISRC1 Packet Body Register 13"]
    #[inline(always)]
    #[must_use]
    pub fn fc_iscr1_13(&mut self) -> FcIscr1_13W<FcIscr1_13Spec> {
        FcIscr1_13W::new(self, 0)
    }
}
#[doc = "Frame Composer ISRC1 Packet Body Register 13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_iscr1_13::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_iscr1_13::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcIscr1_13Spec;
impl crate::RegisterSpec for FcIscr1_13Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_iscr1_13::R`](R) reader structure"]
impl crate::Readable for FcIscr1_13Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_iscr1_13::W`](W) writer structure"]
impl crate::Writable for FcIscr1_13Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_ISCR1_13 to value 0"]
impl crate::Resettable for FcIscr1_13Spec {
    const RESET_VALUE: u8 = 0;
}
