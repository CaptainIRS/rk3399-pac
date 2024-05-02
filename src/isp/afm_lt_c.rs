#[doc = "Register `AFM_LT_C` reader"]
pub type R = crate::R<AfmLtCSpec>;
#[doc = "Register `AFM_LT_C` writer"]
pub type W = crate::W<AfmLtCSpec>;
#[doc = "Field `c_v_t` reader - first line of window C (vertical top line), value must\n\nbe greater or equal 2"]
pub type CVTR = crate::FieldReader<u16>;
#[doc = "Field `c_v_t` writer - first line of window C (vertical top line), value must\n\nbe greater or equal 2"]
pub type CVTW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `c_h_l` reader - first pixel of window C (horizontal left row),\n\nvalue must be greater or equal 5"]
pub type CHLR = crate::FieldReader<u16>;
#[doc = "Field `c_h_l` writer - first pixel of window C (horizontal left row),\n\nvalue must be greater or equal 5"]
pub type CHLW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - first line of window C (vertical top line), value must\n\nbe greater or equal 2"]
    #[inline(always)]
    pub fn c_v_t(&self) -> CVTR {
        CVTR::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - first pixel of window C (horizontal left row),\n\nvalue must be greater or equal 5"]
    #[inline(always)]
    pub fn c_h_l(&self) -> CHLR {
        CHLR::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - first line of window C (vertical top line), value must\n\nbe greater or equal 2"]
    #[inline(always)]
    #[must_use]
    pub fn c_v_t(&mut self) -> CVTW<AfmLtCSpec> {
        CVTW::new(self, 0)
    }
    #[doc = "Bits 16:28 - first pixel of window C (horizontal left row),\n\nvalue must be greater or equal 5"]
    #[inline(always)]
    #[must_use]
    pub fn c_h_l(&mut self) -> CHLW<AfmLtCSpec> {
        CHLW::new(self, 16)
    }
}
#[doc = "Top left corner of measure window C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`afm_lt_c::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`afm_lt_c::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AfmLtCSpec;
impl crate::RegisterSpec for AfmLtCSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`afm_lt_c::R`](R) reader structure"]
impl crate::Readable for AfmLtCSpec {}
#[doc = "`write(|w| ..)` method takes [`afm_lt_c::W`](W) writer structure"]
impl crate::Writable for AfmLtCSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AFM_LT_C to value 0"]
impl crate::Resettable for AfmLtCSpec {
    const RESET_VALUE: u32 = 0;
}
