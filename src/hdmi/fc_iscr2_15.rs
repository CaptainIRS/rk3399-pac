#[doc = "Register `FC_ISCR2_15` reader"]
pub type R = crate::R<FcIscr2_15Spec>;
#[doc = "Register `FC_ISCR2_15` writer"]
pub type W = crate::W<FcIscr2_15Spec>;
#[doc = "Field `FC_ISCR2_15` reader - Frame Composer ISRC2 Packet Body Register 15;\n\nconfigures the ISRC2 packet body of the ISRC2\n\npacket"]
pub type FcIscr2_15R = crate::FieldReader;
#[doc = "Field `FC_ISCR2_15` writer - Frame Composer ISRC2 Packet Body Register 15;\n\nconfigures the ISRC2 packet body of the ISRC2\n\npacket"]
pub type FcIscr2_15W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Frame Composer ISRC2 Packet Body Register 15;\n\nconfigures the ISRC2 packet body of the ISRC2\n\npacket"]
    #[inline(always)]
    pub fn fc_iscr2_15(&self) -> FcIscr2_15R {
        FcIscr2_15R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Frame Composer ISRC2 Packet Body Register 15;\n\nconfigures the ISRC2 packet body of the ISRC2\n\npacket"]
    #[inline(always)]
    #[must_use]
    pub fn fc_iscr2_15(&mut self) -> FcIscr2_15W<FcIscr2_15Spec> {
        FcIscr2_15W::new(self, 0)
    }
}
#[doc = "Frame Composer ISRC2 Packet Body Register 15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_iscr2_15::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_iscr2_15::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcIscr2_15Spec;
impl crate::RegisterSpec for FcIscr2_15Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_iscr2_15::R`](R) reader structure"]
impl crate::Readable for FcIscr2_15Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_iscr2_15::W`](W) writer structure"]
impl crate::Writable for FcIscr2_15Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_ISCR2_15 to value 0"]
impl crate::Resettable for FcIscr2_15Spec {
    const RESET_VALUE: u8 = 0;
}
