#[doc = "Register `SWREG107` reader"]
pub type R = crate::R<Swreg107Spec>;
#[doc = "Register `SWREG107` writer"]
pub type W = crate::W<Swreg107Spec>;
#[doc = "Field `H264_REFPIC_TERM_FLAG` reader - long term flag for reference pictuure index"]
pub type H264RefpicTermFlagR = crate::FieldReader<u32>;
#[doc = "Field `H264_REFPIC_TERM_FLAG` writer - long term flag for reference pictuure index"]
pub type H264RefpicTermFlagW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - long term flag for reference pictuure index"]
    #[inline(always)]
    pub fn h264_refpic_term_flag(&self) -> H264RefpicTermFlagR {
        H264RefpicTermFlagR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - long term flag for reference pictuure index"]
    #[inline(always)]
    #[must_use]
    pub fn h264_refpic_term_flag(&mut self) -> H264RefpicTermFlagW<Swreg107Spec> {
        H264RefpicTermFlagW::new(self, 0)
    }
}
#[doc = "long term flag for reference pictuure index\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg107::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg107::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg107Spec;
impl crate::RegisterSpec for Swreg107Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg107::R`](R) reader structure"]
impl crate::Readable for Swreg107Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg107::W`](W) writer structure"]
impl crate::Writable for Swreg107Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG107 to value 0"]
impl crate::Resettable for Swreg107Spec {
    const RESET_VALUE: u32 = 0;
}
