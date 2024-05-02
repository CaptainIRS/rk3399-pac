#[doc = "Register `AFM_LT_A` reader"]
pub type R = crate::R<AfmLtASpec>;
#[doc = "Register `AFM_LT_A` writer"]
pub type W = crate::W<AfmLtASpec>;
#[doc = "Field `a_v_t` reader - first line of window A (vertical top line), value must\n\nbe greater or equal 2\n\n"]
pub type AVTR = crate::FieldReader<u16>;
#[doc = "Field `a_v_t` writer - first line of window A (vertical top line), value must\n\nbe greater or equal 2\n\n"]
pub type AVTW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `a_h_l` reader - first pixel of window A (horizontal left row),\n\nvalue must be greater or equal 5"]
pub type AHLR = crate::FieldReader<u16>;
#[doc = "Field `a_h_l` writer - first pixel of window A (horizontal left row),\n\nvalue must be greater or equal 5"]
pub type AHLW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - first line of window A (vertical top line), value must\n\nbe greater or equal 2\n\n"]
    #[inline(always)]
    pub fn a_v_t(&self) -> AVTR {
        AVTR::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - first pixel of window A (horizontal left row),\n\nvalue must be greater or equal 5"]
    #[inline(always)]
    pub fn a_h_l(&self) -> AHLR {
        AHLR::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - first line of window A (vertical top line), value must\n\nbe greater or equal 2\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn a_v_t(&mut self) -> AVTW<AfmLtASpec> {
        AVTW::new(self, 0)
    }
    #[doc = "Bits 16:28 - first pixel of window A (horizontal left row),\n\nvalue must be greater or equal 5"]
    #[inline(always)]
    #[must_use]
    pub fn a_h_l(&mut self) -> AHLW<AfmLtASpec> {
        AHLW::new(self, 16)
    }
}
#[doc = "Top Left corner of measure window A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`afm_lt_a::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`afm_lt_a::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AfmLtASpec;
impl crate::RegisterSpec for AfmLtASpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`afm_lt_a::R`](R) reader structure"]
impl crate::Readable for AfmLtASpec {}
#[doc = "`write(|w| ..)` method takes [`afm_lt_a::W`](W) writer structure"]
impl crate::Writable for AfmLtASpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AFM_LT_A to value 0"]
impl crate::Resettable for AfmLtASpec {
    const RESET_VALUE: u32 = 0;
}
